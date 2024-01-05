use bevy_app::{App, Plugin, PreUpdate};
use bevy_ecs::entity::Entity;
use bevy_ecs::event::EventReader;
use bevy_ecs::prelude::Query;
use bevy_transform::prelude::Transform;

use cellophanemc_network::ClientPacketReceivedEvent;
use cellophanemc_protocol::packets::client::ClientPacket::Play;
use cellophanemc_protocol::packets::client::ClientPlayPacket;

pub struct PlayerMovePlugin;

impl Plugin for PlayerMovePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, handle_player_move);
    }
}

fn handle_player_move(
    mut packets: EventReader<ClientPacketReceivedEvent>,
    mut connections: Query<(&mut Transform)>,
) {
    for packet in packets.iter() {
        if let Play(p) = &packet.packet {
            match p {
                ClientPlayPacket::PlayerUpdatePosition(p) => {
                    update_player_position(
                        packet.connection, &mut connections,
                        Some(p.x), Some(p.y), Some(p.z),
                        None, None, p.on_ground,
                    );
                }
                ClientPlayPacket::PlayerUpdateRotation(p) =>
                    update_player_position(
                        packet.connection, &mut connections,
                        None, None, None, Some(p.yaw),
                        Some(p.pitch), p.on_ground,
                    ),
                ClientPlayPacket::PlayerUpdatePositionRotation(p) =>
                    update_player_position(
                        packet.connection, &mut connections,
                        Some(p.x), Some(p.y), Some(p.z), Some(p.yaw), Some(p.pitch), p.on_ground,
                    ),
                ClientPlayPacket::PlayerUpdateOnGround(p) =>
                    update_player_position(
                        packet.connection, &mut connections,
                        None, None, None, None, None, p.on_ground,
                    ),
                _ => {}
            };
        }
    }
}

fn update_player_position(
    entity: Entity,
    mut query: &mut Query<(&mut Transform)>,
    x: Option<f64>,
    y: Option<f64>,
    z: Option<f64>,
    yaw: Option<f32>,
    pitch: Option<f32>,
    on_ground: bool,
) {
    if let Ok((mut transform)) = query.get_mut(entity) {
        let x = x.unwrap_or(transform.translation.x as f64);
        let y = y.unwrap_or(transform.translation.y as f64);
        let z = z.unwrap_or(transform.translation.z as f64);
        let yaw = wrap_degrees(yaw.unwrap_or(transform.rotation.y));
        let pitch = wrap_degrees(pitch.unwrap_or(transform.rotation.x));

        let prev_x = transform.translation.x as f64;
        let prev_y = transform.translation.y as f64;
        let prev_z = transform.translation.z as f64;
        let prev_yaw = transform.scale.y;
        let prev_pitch = transform.scale.x;

        if prev_x != x || prev_y != y || prev_z != z || prev_yaw != yaw || prev_pitch != pitch {
            transform.translation.x = x as f32;
            transform.translation.y = y as f32;
            transform.translation.z = z as f32;
            transform.rotation.y = yaw;
            transform.rotation.x = pitch;
        }
    }
}

fn wrap_degrees(degrees: f32) -> f32 {
    let mut d = degrees % 360.0;
    if d >= 180.0 {
        d -= 360.0;
    }
    if d < -180.0 {
        d += 360.0
    }
    d
}
