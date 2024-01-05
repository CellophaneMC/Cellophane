use std::fmt::Debug;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use bevy_app::{App, Plugin, PostStartup, PreUpdate, Update};
use bevy_ecs::entity::Entity;
use bevy_ecs::event::EventWriter;
use bevy_ecs::prelude::{Commands, Component, Event, Query, World};
use bevy_ecs::system::{Res, Resource, SystemState};
use bevy_reflect::Reflect;
use bytes::Bytes;
use flume::{Receiver, RecvError, Sender, TryRecvError};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::runtime::Runtime;
use tokio::task::JoinHandle;
use tokio::time::Instant;
use tracing::{error, info, warn};

use cellophanemc_protocol::{Decoder, Encoder};
use cellophanemc_protocol::packets::client::{ClientConfigurationPacket, ClientHandshakePacket, ClientLoginPacket, ClientPacket, ClientPlayPacket, ClientStatusPacket, HandshakeState};

use crate::decode::PacketDecoder;
use crate::encode::PacketEncoder;

pub mod decode;
pub mod encode;

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        if let Err(e) = build_plugin(app) {
            error!("Failed to build network plugin: {e}");
        }
    }
}

fn build_plugin(app: &mut App) -> anyhow::Result<()> {
    let runtime = Runtime::new()?;

    let (new_connections_send, new_connections_recv) = flume::bounded::<RemoteConnection>(64);
    let shared = SharedNetworkState(Arc::new(SharedNetworkStateInner {
        new_connections_send,
        new_connections_recv,
    }));

    app.insert_resource(shared.clone());

    let accept_loop_system = move |shared: Res<SharedNetworkState>| {
        let _guard = runtime.handle().enter();
        tokio::spawn(accept_loop(shared.clone()));
    };

    let spawn_new_connections = move |world: &mut World| {
        while let Ok(connection) = shared.0.new_connections_recv.try_recv() {
            world.spawn(connection);
        }
    };

    app.add_systems(PostStartup, accept_loop_system);
    app.add_systems(PreUpdate, spawn_new_connections);
    app.add_event::<ClientPacketReceivedEvent>();
    app.add_event::<DisconnectEvent>();
    app.add_systems(Update, run_packet_event_loop);
    app.register_type::<HandshakeState>();

    Ok(())
}

async fn accept_loop(shared: SharedNetworkState) {
    let addr = "0.0.0.0:25565";
    let listener = match TcpListener::bind(addr).await {
        Ok(listener) => {
            info!("Listening at: {addr}");
            listener
        }
        Err(e) => {
            error!("Failed to start TCP listener: {e}");
            return;
        }
    };

    let timeout = Duration::from_secs(5);

    loop {
        match listener.accept().await {
            Ok((stream, remote_addr)) => {
                let shared = shared.clone();

                tokio::spawn(async move {
                    if let Err(e) = tokio::time::timeout(
                        timeout,
                        handle_connection(shared, stream, remote_addr),
                    )
                        .await
                    {
                        error!("Connection timed out: {e}");
                    }
                });
            }
            Err(e) => {
                println!("Failed to accept connection: {e}");
            }
        }
    }
}

const READ_BUF_SIZE: usize = 4096;

#[derive(Clone, Debug)]
pub struct PacketFrame {
    pub timestamp: Instant,
    pub payload: Bytes,
}

async fn handle_connection(shared: SharedNetworkState, stream: TcpStream, remote_addr: SocketAddr) {
    println!("handle new connection: {}", remote_addr);
    if let Err(e) = stream.set_nodelay(true) {
        error!("Failed to set TCP_NODELAY: {e}");
    }
    let (mut reader, mut writer) = stream.into_split();

    let (incoming_sender, incoming_receiver) = flume::unbounded::<PacketFrame>();
    let recv_task = tokio::spawn(async move {
        let mut decoder = PacketDecoder::new();

        loop {
            let payload = match decoder.read_frame(&mut reader).await {
                Ok(payload) => payload,
                Err(e) => {
                    error!("error decoding packet frame: {e:#}");
                    break;
                }
            };

            let timestamp = Instant::now();
            let frame = PacketFrame {
                timestamp,
                payload,
            };

            if let Err(e) = incoming_sender.try_send(frame) {
                error!("error sending packet frame: {e:#}");
            }
        }
    });

    let (outgoing_sender, outgoing_receiver) = flume::unbounded::<Bytes>();
    let send_task = tokio::spawn(async move {
        let mut encoder = PacketEncoder::new();

        loop {
            let bytes = match outgoing_receiver.recv_async().await {
                Ok(frame) => frame,
                Err(e) => match e {
                    RecvError::Disconnected => break
                },
            };

            if let Err(e) = encoder.write_frame(&mut writer, &bytes).await {
                warn!("error writing data to stream: {e}")
            }
        }
    });

    let connection = RemoteConnection {
        remote_addr,
        recv: incoming_receiver,
        send: outgoing_sender,
        recv_task,
        send_task,
        state: HandshakeState::Handshaking,
    };

    let _ = shared.0.new_connections_send.send_async(connection).await;
}

#[allow(clippy::type_complexity)]
fn run_packet_event_loop(
    world: &mut World,
    state: &mut SystemState<(
        Query<(Entity, &mut RemoteConnection)>,
        EventWriter<ClientPacketReceivedEvent>,
        EventWriter<DisconnectEvent>,
        Commands,
    )>,
) {
    let (mut connections, mut packet_events, mut disconnect_events, mut commands) =
        state.get_mut(world);

    for (entity, mut connection) in &mut connections {
        let result = connection.try_recv();

        match result {
            Ok(frame) => {
                let cursor = &mut std::io::Cursor::new(frame.payload);
                let packet = match connection.state {
                    HandshakeState::Handshaking => ClientHandshakePacket::read(cursor).map(|p| ClientPacket::Handshake(p)),
                    HandshakeState::Status => ClientStatusPacket::read(cursor).map(|p| ClientPacket::Status(p)),
                    HandshakeState::Login => ClientLoginPacket::read(cursor).map(|p| ClientPacket::Login(p)),
                    HandshakeState::Configuration => ClientConfigurationPacket::read(cursor).map(|p| ClientPacket::Configuration(p)),
                    HandshakeState::Play => ClientPlayPacket::read(cursor).map(|p| ClientPacket::Play(p))
                };

                match packet {
                    Ok(packet) => packet_events.send(ClientPacketReceivedEvent {
                        connection: entity,
                        packet,
                    }),
                    Err(e) => {
                        error!("error reading packet: {e:#}");
                    }
                }
            }
            Err(TryRecvError::Empty) => {}
            Err(TryRecvError::Disconnected) => {
                println!("Client disconnected");
                disconnect_events.send(DisconnectEvent { connection: entity });
                commands.entity(entity).despawn();
            }
        }
    }

    state.apply(world);
}

#[derive(Resource, Clone)]
pub struct SharedNetworkState(Arc<SharedNetworkStateInner>);

struct SharedNetworkStateInner {
    new_connections_send: Sender<RemoteConnection>,
    new_connections_recv: Receiver<RemoteConnection>,
}

#[derive(Component)]
pub struct RemoteConnection {
    pub remote_addr: SocketAddr,
    recv: Receiver<PacketFrame>,
    send: Sender<Bytes>,
    recv_task: JoinHandle<()>,
    send_task: JoinHandle<()>,
    pub state: HandshakeState,
}

impl RemoteConnection {
    pub fn try_recv(&mut self) -> Result<PacketFrame, TryRecvError> {
        self.recv.try_recv()
    }

    pub fn send_raw_packet(&mut self, bytes: &[u8]) -> anyhow::Result<()> {
        self.send.try_send(Bytes::from(bytes.to_vec()))?;
        Ok(())
    }

    pub fn send_packet<T>(&mut self, packet: T) -> anyhow::Result<()>
        where
            T: Encoder + Debug,
    {
        let mut buf = Vec::with_capacity(4096);
        packet.write(&mut buf)?;
        self.send_raw_packet(&buf)
    }
}

impl Drop for RemoteConnection {
    fn drop(&mut self) {
        println!("dropping connection {}", self.remote_addr);
        self.recv_task.abort();
        self.send_task.abort();
    }
}

#[derive(Event, Clone, Debug)]
pub struct ClientPacketReceivedEvent {
    pub connection: Entity,
    pub packet: ClientPacket,
}

#[derive(Event, Clone, Debug)]
pub struct DisconnectEvent {
    pub connection: Entity,
}

#[derive(Event, Clone, Debug)]
pub struct HandshakeEvent {
    connection: Entity,
    protocol_version: i32,
    server_address: String,
    server_port: u16,
    next_state: HandshakeState,
}

impl HandshakeEvent {
    pub fn connection(&self) -> Entity {
        self.connection
    }

    pub fn protocol_version(&self) -> i32 {
        self.protocol_version
    }

    pub fn server_address(&self) -> &str {
        &self.server_address
    }

    pub fn server_port(&self) -> u16 {
        self.server_port
    }

    pub fn next_state(&self) -> HandshakeState {
        self.next_state
    }
}

#[derive(Event, Clone, Debug)]
pub struct ClientStatusRequestEvent {
    pub connection: Entity,
}
