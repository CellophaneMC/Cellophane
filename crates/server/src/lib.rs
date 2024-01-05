use bevy_ecs::component::Component;
use bit_set::BitSet;
use glam::ivec3;
use rand::Rng;
use rand_core::RngCore;

use cellophanemc_core::block_state::BlockStateId;
use cellophanemc_core::chunk_pos::ChunkPos;
use cellophanemc_core::palette::Palette;
use cellophanemc_core::volume::BlockVolumeMut;
use cellophanemc_nbt::aa::Value::Compound;
use cellophanemc_network::RemoteConnection;
use cellophanemc_protocol::Encoder;
use cellophanemc_protocol::packets::server::{ChunkData, JoinGame, MovePlayer, ServerPlayPacket, SetChunkCacheCenter, SetSpawn};
use chunk::Chunk;

pub mod keepalive;
pub mod player_move;
pub mod chunk_view;
pub mod player;
pub mod chunk_map;
pub mod world;
pub mod chunk_generator;
pub mod chunk;
pub mod biome_generator;
mod chunk_storage;

#[derive(Component)]
struct Player;

pub fn place_new_player(
    connection: &mut RemoteConnection,
) {
    let _ = connection.send_packet(ServerPlayPacket::JoinGame(JoinGame {
        entity_id: 0,
        is_hardcore: false,
        dimensions: vec![
            "minecraft:overworld".to_string()
        ],
        max_players: 100,
        view_distance: 16,
        simulation_distance: 16,
        reduced_debug_info: false,
        enable_respawn_screen: true,
        do_limited_crafting: false,
        dimension_type: "minecraft:overworld".to_string(),
        dimension_name: "minecraft:overworld".to_string(),
        hashed_seed: 0,
        gamemde: 0,
        previous_gamemode: -1,
        is_debug: false,
        is_flat: true,
        death_info: None,
        portal_cooldown: 40,
    }));

    let _ = connection.send_packet(ServerPlayPacket::SetChunkCacheCenter(SetChunkCacheCenter {
        chunk_x: 0,
        chunk_z: 0,
    }));

    let _ = connection.send_packet(ServerPlayPacket::SetSpawn(SetSpawn {
        pos: ivec3(0, 2, 0),
        angle: 93.0,
    }));

    let _ = connection.send_packet(ServerPlayPacket::MovePlayer(MovePlayer {
        x: 0.0,
        y: 2.0,
        z: 0.0,
        yaw: 0.0,
        pitch: 0.0,
        flags: 0,
        teleport_id: 0,
    }));


    let mut chunk = Chunk::new(ChunkPos::new(0, 0));

    let mut rng = rand::thread_rng();

    for block_x in 0..16 {
        for block_z in 0..16 {
            let random_bool: bool = rng.gen();
            if random_bool {
                chunk.set_block_at(block_x, 0, block_z, BlockStateId(1));
            } else {
                chunk.set_block_at(block_x, 0, block_z, BlockStateId(2));
            }
        }
    }

    let _ = connection.send_packet(ServerPlayPacket::ChunkData(ChunkData::from(&chunk)));
}

impl From<&Chunk> for ChunkData {
    fn from(chunk: &Chunk) -> Self {
        let mut data = vec![];
        for x in &chunk.sections {
            x.write(&mut data);
        }

        ChunkData {
            chunk_x: 0,
            chunk_z: 0,
            heightmaps: Compound(Default::default()),
            data,
            block_entities: vec![],
            sky_light_mask: BitSet::new(),
            block_light_mask: BitSet::new(),
            empty_sky_light_mask: BitSet::new(),
            empty_block_light_mask: BitSet::new(),
            sky_updates: vec![],
            block_updates: vec![],
        }
    }
}
