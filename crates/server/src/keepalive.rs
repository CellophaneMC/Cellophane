use std::time::{Duration, Instant, SystemTime};

use bevy_app::{App, Plugin, PostUpdate, PreUpdate};
use bevy_ecs::change_detection::Res;
use bevy_ecs::component::Component;
use bevy_ecs::entity::Entity;
use bevy_ecs::event::EventReader;
use bevy_ecs::system::{Commands, Query, Resource};
use bevy_reflect::Reflect;

use cellophanemc_network::{ClientPacketReceivedEvent, RemoteConnection};
use cellophanemc_protocol::packets::client::ClientPacket::Play;
use cellophanemc_protocol::packets::client::ClientPlayPacket::KeepAliveResponse;
use cellophanemc_protocol::packets::server::{KeepAliveRequest, ServerPlayPacket};

pub struct KeepAlivePlugin;

impl Plugin for KeepAlivePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<KeepAliveSettings>();
        app.add_systems(PreUpdate, handle_keepalive_response);
        app.add_systems(PostUpdate, send_keepalive);
        app.register_type::<KeepAliveState>();
    }
}

#[derive(Resource, Debug)]
pub struct KeepAliveSettings {
    pub interval: Duration,
}

#[derive(Component, Reflect, Debug)]
pub struct KeepAliveState {
    pending: bool,
    last_id: u64,
    last_send: Instant,
}

impl KeepAliveState {
    pub fn new() -> Self {
        Self {
            pending: false,
            last_id: 0,
            last_send: Instant::now(),
        }
    }

    /// When the last keepalive was sent for this client.
    pub fn last_send(&self) -> Instant {
        self.last_send
    }
}

impl Default for KeepAliveSettings {
    fn default() -> Self {
        Self {
            interval: Duration::from_secs(5)
        }
    }
}

fn send_keepalive(
    mut connections: Query<(Entity, &mut RemoteConnection, &mut KeepAliveState)>,
    settings: Res<KeepAliveSettings>,
    mut commands: Commands,
) {
    let now = Instant::now();

    for (entity, mut connection, mut state) in &mut connections {
        if now.duration_since(state.last_send) >= settings.interval {
            if state.pending {
                // TODO: message
                commands.entity(entity).remove::<RemoteConnection>();
            } else {
                state.pending = true;
                state.last_id = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis() as u64;
                state.last_send = now;
                let _ = connection.send_packet(ServerPlayPacket::KeepAliveRequest(KeepAliveRequest {
                    id: state.last_id
                }));
            }
        }
    }
}

fn handle_keepalive_response(
    mut packets: EventReader<ClientPacketReceivedEvent>,
    mut connections: Query<(Entity, &mut KeepAliveState)>,
    mut commands: Commands,
) {
    for packet in packets.iter() {
        if let Play(KeepAliveResponse(response)) = &packet.packet {
            if let Ok((entity, mut state)) = connections.get_mut(packet.connection) {
                if state.pending && response.id == state.last_id {
                    state.pending = false;
                } else {
                    // TODO: message
                    commands.entity(entity).remove::<RemoteConnection>();
                }
            }
        }
    }
}
