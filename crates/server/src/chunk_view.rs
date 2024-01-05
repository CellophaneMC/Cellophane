use std::cmp::max;
use std::collections::{BinaryHeap, HashSet};

use bevy_app::{App, Plugin, Update};
use bevy_ecs::entity::Entity;
use bevy_ecs::prelude::{Component, Query, Resource, With};
use bevy_ecs::query::{Added, Changed};
use bevy_ecs::system::{Commands, Res};
use bevy_reflect::Reflect;
use bevy_transform::prelude::Transform;

use cellophanemc_core::chunk_pos::ChunkPos;
use cellophanemc_network::RemoteConnection;
use cellophanemc_protocol::packets::server::{ServerPlayPacket, SetChunkCacheCenter, SetChunkCacheRadius, SetSimulationDistance};
use cellophanemc_world::chunk::generate_bfs_order;

use crate::chunk_map::{ChunkFilter, CylindricalChunkFilter};
use crate::player::PlayerSettings;
use crate::world::{ChunkHolder, World};

pub struct PlayerChunkLoaderPlugin;

impl Plugin for PlayerChunkLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, add_view_distances);
        app.add_systems(Update, update_chunk_view);
        app.add_systems(Update, update_transform);
        app.init_resource::<ChunkRadiusIteration>();
        app.register_type::<ViewDistances>();
    }
}

#[derive(Debug, Component, Reflect, Eq, PartialEq)]
pub struct ViewDistances {
    pub tick_distance: usize,
    pub load_distance: usize,
    pub send_distance: usize,
}

impl Default for ViewDistances {
    fn default() -> Self {
        Self {
            tick_distance: 0,
            load_distance: 0,
            send_distance: 0,
        }
    }
}

impl ViewDistances {
    pub fn new(distance: usize) -> Self {
        Self {
            tick_distance: (distance),
            load_distance: (distance + 1),
            send_distance: (distance + 1),
        }
    }

    fn tick_distance(
        player: &ViewDistances,
        world: &ViewDistances,
    ) -> usize {
        if player.tick_distance > 0 {
            player.tick_distance
        } else {
            world.tick_distance
        }
    }

    fn load_distance(
        tick_view_distance: usize,
        player: &ViewDistances,
        world: &ViewDistances,
    ) -> usize {
        let load_distance = if player.load_distance < 0 {
            world.load_distance
        } else {
            player.load_distance
        };
        (tick_view_distance + 1).max(load_distance)
    }

    fn send_view_distance(
        load_view_distance: usize,
        client_view_distance: usize,
        player: &ViewDistances,
        world: &ViewDistances,
    ) -> usize {
        std::cmp::min(
            load_view_distance - 1,
            if player.send_distance > 0 {
                player.send_distance
            } else {
                if client_view_distance > 0 {
                    client_view_distance + 1
                } else {
                    if world.send_distance > 0 {
                        world.send_distance
                    } else {
                        load_view_distance - 1
                    }
                }
            },
        )
    }
}

#[derive(Debug, Resource)]
struct ChunkRadiusIteration(pub Vec<Vec<ChunkPos>>);

impl Default for ChunkRadiusIteration {
    fn default() -> Self {
        let mut list = Vec::new();
        for i in 0..16 {
            list.push(generate_bfs_order(i))
        }

        ChunkRadiusIteration(list)
    }
}

#[derive(Debug, Component, Default)]
pub struct PlayerChunkLoader {
    last_chunk: ChunkPos,
    last_view_distance: ViewDistances,
    last_sent_chunk_radius: Option<usize>,
    last_sent_simulation_distance: Option<usize>,
    sent_chunk: HashSet<ChunkPos>,
}

impl PlayerChunkLoader {
    pub fn want_chunk_loaded(
        center: &ChunkPos,
        chunk: &ChunkPos,
        distance: usize,
    ) -> bool {
        // expect sendRadius to be = 1 + target viewable radius
        CylindricalChunkFilter::new(*center, distance).contains(chunk, true)
    }

    fn want_chunk_sent(
        &self,
        chunk: &ChunkPos,
    ) -> bool {
        let delta = ChunkPos {
            x: self.last_chunk.x - chunk.x,
            z: self.last_chunk.z - chunk.z,
        };
        max(delta.x.abs(), delta.z.abs()) <= (self.last_view_distance.send_distance as i32 + 1) &&
            PlayerChunkLoader::want_chunk_loaded(&self.last_chunk, chunk, self.last_view_distance.send_distance)
    }
}

impl PlayerChunkLoader {
    fn new() -> Self {
        PlayerChunkLoader::default()
    }
}

fn update_transform(
    mut query: Query<(&Transform, &mut PlayerChunkLoader, &mut RemoteConnection), Changed<Transform>>
) {
    for (t, l, c) in query.iter() {}
}

fn add_view_distances(
    mut query: Query<(Entity, &PlayerSettings), Added<PlayerSettings>>,
    mut commands: Commands,
) {
    for (entity, settings) in query.iter() {
        println!("add view distance to player");
        commands.entity(entity).insert(ViewDistances::default());
        commands.entity(entity).insert(PlayerChunkLoader::new());
    }
}

fn update_chunk_view(
    chunk_iter_list: Res<ChunkRadiusIteration>,
    mut player_query: Query<(&Transform, &World, &mut PlayerChunkLoader, &mut LoadChunkQueue, &ViewDistances, &PlayerSettings, &mut RemoteConnection), Changed<Transform>>,
    mut world_query: Query<(&World, &ViewDistances), With<ChunkHolder>>,
) {
    // println!("iterate world");
    for (transform, player_world, mut loader, mut load_queue, player_view_distances, settings, mut connection) in &mut player_query {
        for (world, world_view_distances) in world_query.iter() {
            if world != player_world {
                continue;
            }
            let current_chunk = ChunkPos {
                x: (transform.translation.x as i32) >> 4,
                z: (transform.translation.z as i32) >> 4,
            };
            let tick_distance = ViewDistances::tick_distance(
                player_view_distances,
                &world_view_distances,
            );

            let load_distance = ViewDistances::load_distance(
                tick_distance,
                player_view_distances,
                &world_view_distances,
            );
            let client_view_distance = settings.view_distance as usize;
            let send_distance = ViewDistances::send_view_distance(
                load_distance,
                client_view_distance,
                player_view_distances,
                &world_view_distances,
            );

            let view_distance = ViewDistances {
                tick_distance,
                load_distance,
                send_distance,
            };
            if view_distance == loader.last_view_distance {
                continue;
            }

            if current_chunk == loader.last_chunk {
                continue;
            }

            println!("iterate player: {:?} - {:?}", current_chunk, view_distance);

            if loader.last_sent_chunk_radius != Some(send_distance) {
                let _ = connection.send_packet(
                    ServerPlayPacket::SetChunkCacheRadius(SetChunkCacheRadius {
                        radius: send_distance as i32
                    })
                );
            }

            if loader.last_sent_simulation_distance != Some(tick_distance) {
                let _ = connection.send_packet(
                    ServerPlayPacket::SetSimulationDistance(SetSimulationDistance {
                        simulation_distance: tick_distance as i32
                    })
                );
            }

            loader.last_chunk = current_chunk;

            for delta_pos in chunk_iter_list.0[load_distance].iter() {
                let chunk_pos = ChunkPos::new(current_chunk.x + delta_pos.x, current_chunk.z + delta_pos.z);
                let square_distance = max(delta_pos.x.abs(), delta_pos.z.abs()) as usize;
                let send_chunk = (square_distance <= send_distance) && PlayerChunkLoader::want_chunk_loaded(&current_chunk, &chunk_pos, send_distance);
                let sent_chunk = if send_chunk {
                    loader.sent_chunk.contains(&chunk_pos)
                } else {
                    loader.sent_chunk.remove(&chunk_pos)
                };

                if !send_chunk && sent_chunk {
                    println!("unload: {:?}", chunk_pos)
                } else {
                    println!("load: {:?}", chunk_pos)
                }
            }

            let _ = connection.send_packet(
                ServerPlayPacket::SetChunkCacheCenter(SetChunkCacheCenter {
                    chunk_x: current_chunk.x,
                    chunk_z: current_chunk.z,
                })
            );
        }
    }
}

#[derive(Component)]
struct LoadChunkQueue(BinaryHeap<PriorityChunkPos>);

#[derive(Component)]
struct LoadingChunkQueue(BinaryHeap<PriorityChunkPos>);

#[derive(Component)]
struct GenChunkQueue(BinaryHeap<PriorityChunkPos>);

#[derive(Component)]
struct SendChunkQueue(BinaryHeap<PriorityChunkPos>);

#[derive(Debug, PartialEq, Eq)]
struct PriorityChunkPos {
    chunk: ChunkPos,
    center: ChunkPos,
}

// impl PartialOrd<Self> for PriorityChunkPos {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         Some(self.cmp(other))
//     }
// }
//
// impl Ord for PriorityChunkPos {
//     fn cmp(&self, other: &Self) -> Ordering{
//         // (self.chunk.x - self.center.x).abs() + (self.chunk.z - self.center.z).cmp(
//         //     &((other.chunk.x - self.center.x).abs() + (other.chunk.z - self.center.z))
//         // )
//     }
// }

#[derive(Debug, Eq, PartialEq)]
enum ChunkTicketStage {
    None,
    Loading,
    Loaded,
    Generating,
    Generated,
    Tick,
}
