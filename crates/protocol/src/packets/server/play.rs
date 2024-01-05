use bit_set::BitSet;
use glam::IVec3;

use cellophanemc_nbt::aa::Value;

use super::*;

packets!(
    BundleDelimiter {
    }

    SpawnEntity {
        entity_id VarInt;
        entity_uuid Uuid;
        entity_type VarInt;
        x f64;
        y f64;
        z f64;
        pitch Angle;
        yaw Angle;
        head_yaw Angle;
        data VarInt;
        velocity_x i16;
        velocity_y i16;
        velocity_z i16;
    }

    KeepAliveRequest {
        id u64;
    }

    ChunkData {
        chunk_x i32;
        chunk_z i32;
        heightmaps Value;
        data VarIntPrefixedVec<u8>;
        block_entities VarIntPrefixedVec<ChunkDataBlockEntity>;
        sky_light_mask BitSet;
        block_light_mask BitSet;
        empty_sky_light_mask BitSet;
        empty_block_light_mask BitSet;
        sky_updates VarIntPrefixedVec<ChunkDataBlocks>;
        block_updates VarIntPrefixedVec<ChunkDataBlocks>;
    }

    JoinGame {
        entity_id i32;
        is_hardcore bool;
        dimensions VarIntPrefixedVec<String>;
        max_players VarInt;
        view_distance VarInt;
        simulation_distance VarInt;
        reduced_debug_info bool;
        enable_respawn_screen bool;
        do_limited_crafting bool;
        dimension_type String;
        dimension_name String;
        hashed_seed i64;
        gamemde u8;
        previous_gamemode i8;
        is_debug bool;
        is_flat bool;
        death_info Option<JoinGameDeathInfo>;
        portal_cooldown VarInt;
    },

    JoinGameDeathInfo {
        dimension_name String;
        position u64;
    },

    MovePlayer {
        x f64;
        y f64;
        z f64;
        yaw f32;
        pitch f32;
        flags u8;
        teleport_id VarInt;
    },

    SetChunkCacheCenter {
        chunk_x VarInt;
        chunk_z VarInt;
    }

    SetChunkCacheRadius {
        radius VarInt;
    }

    SetSimulationDistance {
        simulation_distance VarInt;
    }

    SetSpawn {
        pos IVec3;
        angle f32;
    },

    ChunkDataBlockEntity {
        packed_xz u8;
        y u16;
        kind VarInt;
        data Value;
    },


    ChunkDataBlocks {
        blocks VarIntPrefixedVec<u8>
    }
);
