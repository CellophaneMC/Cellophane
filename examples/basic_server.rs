use std::fmt::Debug;
use std::fs::{File, read_dir};
use std::io;
use std::io::ErrorKind;

use bevy::prelude::*;
use bevy::reflect::Uuid;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use serde::{Deserialize, Serialize, Serializer};
use serde::de::DeserializeOwned;
use serde_json::json;

use cellophanemc_network::*;
use cellophanemc_profile::GameProfile;
use cellophanemc_protocol::packets::client::{ClientConfigurationPacket, ClientHandshakePacket, ClientLoginPacket, ClientPacket, ClientStatusPacket, HandshakeState};
use cellophanemc_protocol::packets::server::{FinishConfiguration, LoginSuccess, Pong, RegistryData, Response, ServerConfigurationPacket, ServerLoginPacket, ServerStatusPacket};
use cellophanemc_server::chunk_view::PlayerChunkLoaderPlugin;
use cellophanemc_server::keepalive::{KeepAlivePlugin, KeepAliveState};
use cellophanemc_server::place_new_player;
use cellophanemc_server::player::{Player, PlayerSettings};
use cellophanemc_server::player_move::PlayerMovePlugin;
use cellophanemc_server::world::{World, WorldPlugin};
use cellophanemc_world::biome::Biome;
use cellophanemc_world::damage::DamageType;
use cellophanemc_world::dimension::DimensionType;

fn main() {
    println!("start server");
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(WorldInspectorPlugin::new())
        .register_type::<ClientProtocolVersion>()
        .add_plugins(NetworkPlugin)
        .add_plugins(KeepAlivePlugin)
        .add_plugins(PlayerMovePlugin)
        .add_plugins(WorldPlugin)
        .add_plugins(PlayerChunkLoaderPlugin)
        .add_event::<ConfigurationStartEvent>()
        .add_systems(Update, handle_handshake)
        .add_systems(Update, handle_status)
        .add_systems(Update, handle_login)
        .add_systems(Update, handle_configuration_start)
        .add_systems(Update, handle_configuration_ack)
        .register_type::<Player>()
        .register_type::<Uuid>()
        .register_type::<GameProfile>()
        .run();
}

#[derive(Event, Debug, Copy, Clone)]
struct ConfigurationStartEvent {
    connection: Entity,
}

#[derive(Component, Reflect, Debug, Copy, Clone)]
struct ClientProtocolVersion(i32);

fn handle_handshake(
    mut commands: Commands,
    mut connections: Query<&mut RemoteConnection>,
    mut events: EventReader<ClientPacketReceivedEvent>,
) {
    for event in events.iter() {
        let packet = match &event.packet {
            ClientPacket::Handshake(p) => {
                match p {
                    ClientHandshakePacket::Handshake(p) => p
                }
            }
            _ => continue
        };

        commands.entity(event.connection).insert(ClientProtocolVersion(packet.protocol_version));

        if let Ok(mut connection) = connections.get_mut(event.connection) {
            connection.state = packet.next_state
        }
    }
}

fn handle_status(
    mut connections: Query<(&mut RemoteConnection, &ClientProtocolVersion)>,
    mut events: EventReader<ClientPacketReceivedEvent>,
) {
    for event in events.iter() {
        let packet = match &event.packet {
            ClientPacket::Status(p) => p,
            _ => continue
        };

        if let Ok((mut connection, protocol)) = connections.get_mut(event.connection) {
            match packet {
                ClientStatusPacket::Request(p) => {
                    let packet = ServerStatusPacket::Response(Response {
                        response: json!({
                    "version": {
                        "protocol": protocol.0,
                        "name": "karbon 1.20.2",
                    },
                        "players":{
                            "online": 0,
                            "max": 100,
                            "sample": []
                        },
                        "description":{
                            "text": "Karbon"
                        }
                })
                            .to_string(),
                    });

                    let _ = connection.send_packet(packet);
                }
                ClientStatusPacket::Ping(p) => {
                    let packet = ServerStatusPacket::Pong(Pong {
                        payload: p.payload
                    });
                    let _ = connection.send_packet(packet);
                }
            }
        }
    }
}

fn handle_login(
    mut configuration_start_events: EventWriter<ConfigurationStartEvent>,
    mut connections: Query<(Entity, &mut RemoteConnection)>,
    mut events: EventReader<ClientPacketReceivedEvent>,
    mut commands: Commands,
) {
    for event in events.iter() {
        match &event.packet {
            ClientPacket::Login(p) => {
                match p {
                    ClientLoginPacket::LoginStart(p) => {
                        println!("login start {}", p.username);

                        if let Ok((entity, mut connection)) = connections.get_mut(event.connection) {
                            commands.entity(entity).insert(
                                cellophanemc_server::player::Player::new(
                                    cellophanemc_profile::GameProfile::new(
                                        p.uuid,
                                        p.username.clone(),
                                    )
                                )
                            );
                            println!("added world to player");
                            let _ = connection.send_packet(ServerLoginPacket::LoginSuccess(
                                LoginSuccess {
                                    uuid: p.uuid,
                                    username: p.username.clone(),
                                    properties: vec![],
                                }
                            ));
                        }
                    }
                    ClientLoginPacket::EncryptionResponse(_) => {}
                    ClientLoginPacket::LoginPluginResponse(_) => {}
                    ClientLoginPacket::LoginAck(_) => {
                        println!("Configuration ack");
                        if let Ok((_, mut connection)) = connections.get_mut(event.connection) {
                            connection.state = HandshakeState::Configuration;
                            configuration_start_events.send(ConfigurationStartEvent { connection: event.connection });
                        };
                    }
                }
            }
            _ => continue
        };
    }
}

const DIMENSION_TYPE_OVERWORLD: &str = include_str!("../assets/minecraft/dimension_type/overworld.json");

fn handle_configuration_start(
    mut events: EventReader<ConfigurationStartEvent>,
    mut connections: Query<(Entity, &mut RemoteConnection)>,
    mut commands: Commands,
) {
    for event in events.iter() {
        if let Ok((entity, mut connection)) = connections.get_mut(event.connection) {
            let registry = cellophanemc_nbt::value::to_value(RegistryCodec {
                dimension_type: Registry {
                    name: "minecraft:dimension_type".to_string(),
                    value: vec![
                        RegistryEntry {
                            name: "minecraft:overworld".to_string(),
                            id: 0,
                            element: serde_json::de::from_str::<DimensionType>(DIMENSION_TYPE_OVERWORLD).unwrap(),
                        }
                    ],
                },
                damage_type: load_registry("damage_type").unwrap(),
                worldgen_biome: load_registry("worldgen/biome").unwrap(),
            }).unwrap();

            commands.get_or_spawn(entity).insert(
                (
                    KeepAliveState::new(),
                    Transform::default(),
                    World::new("world")
                )
            );

            // println!("nbt: {:?}", registry);

            let _ = connection.send_packet(
                ServerConfigurationPacket::RegistryData(
                    RegistryData { data: registry }
                )
            );
            let _ = connection.send_packet(
                ServerConfigurationPacket::FinishConfiguration(
                    FinishConfiguration {}
                )
            );
        }
    }
}

fn handle_configuration_ack(
    mut connections: Query<(Entity, &mut RemoteConnection)>,
    mut events: EventReader<ClientPacketReceivedEvent>,
    mut commands: Commands,
) {
    for event in events.iter() {
        if let ClientPacket::Configuration(p) = &event.packet {
            match p {
                ClientConfigurationPacket::ClientInformation(packet) => {
                    if let Ok((entity, mut connection)) = connections.get_mut(event.connection) {
                        commands.entity(entity).insert(
                            PlayerSettings::from(packet)
                        );
                    }
                }
                ClientConfigurationPacket::PluginMessageConfiguration(_) => {}
                ClientConfigurationPacket::FinishConfiguration(_) => {
                    println!("Client finishing configuration!");
                    if let Ok((entity, mut connection)) = connections.get_mut(event.connection) {
                        connection.state = HandshakeState::Play;
                        place_new_player(&mut connection);
                    }
                }
                ClientConfigurationPacket::KeepAliveResponse(_) => {}
                ClientConfigurationPacket::Pong(_) => {}
                ClientConfigurationPacket::ResourcePack(_) => {}
            }
        }
    }
}

fn load_registry<'a, T: Debug + DeserializeOwned>(name: &str) -> Result<Registry<T>, io::Error> {
    let dir = read_dir(format!("assets/minecraft/{}", name))?;
    let mut value: Vec<RegistryEntry<T>> = Vec::new();

    let mut id = 0;
    for entry in dir {
        let entry = entry?;
        let path = entry.path();
        let name = path.file_stem().unwrap().to_str().unwrap().to_string();
        if let Some(extension) = path.extension() {
            if extension == "json" {
                let file = File::open(&path)?;
                let reader = io::BufReader::new(file);

                let element: T = serde_json::from_reader(reader).map_err(|e| {
                    io::Error::new(ErrorKind::InvalidData, format!("Can't load {} - {}", name, e))
                })?;
                let registry_entry = RegistryEntry {
                    name,
                    id,
                    element,
                };
                value.push(registry_entry);
            }
        }
    }

    let r = Registry {
        name: format!("minecraft:{}", name).to_string(),
        value,
    };
    Ok(r)
}

#[derive(Debug, Serialize, Deserialize)]
struct Registry<T: Debug> {
    #[serde(rename = "type")]
    pub name: String,
    pub value: Vec<RegistryEntry<T>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RegistryEntry<T> {
    pub name: String,
    pub id: i32,
    pub element: T,
}

#[derive(Debug, Serialize, Deserialize)]
struct RegistryCodec {
    #[serde(rename = "minecraft:dimension_type")]
    pub dimension_type: Registry<DimensionType>,
    #[serde(rename = "minecraft:damage_type")]
    pub damage_type: Registry<DamageType>,
    #[serde(rename = "minecraft:worldgen/biome")]
    pub worldgen_biome: Registry<Biome>,
}

fn handle_disconnect(mut events: EventReader<DisconnectEvent>) {
    for event in events.iter() {
        println!("disconnect: {:?}", event);
    }
}
