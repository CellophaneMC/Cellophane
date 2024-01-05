use std::collections::HashMap;

use bevy_app::{App, Plugin, Startup};
use bevy_ecs::prelude::{Commands, Component};
use bevy_ecs::prelude::*;
use bevy_reflect::prelude::*;
use bevy_reflect::Reflect;

use cellophanemc_core::chunk_pos::ChunkPos;

use crate::chunk::Chunk;
use crate::chunk_view::ViewDistances;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_world);
        app.register_type::<World>();
    }
}

#[derive(Debug, Component, Reflect, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[reflect(Component, Default, PartialEq)]
pub struct World {
    pub id: String,
}

impl World {
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_owned()
        }
    }
}

#[derive(Default, Component)]
pub struct ChunkHolder {
    chunks: HashMap<ChunkPos, Chunk>,
}

impl ChunkHolder {
    fn new() -> Self {
        Self::default()
    }
}

fn init_world(
    mut commands: Commands,
) {
    commands.spawn((
        World::new("world"),
        ChunkHolder::new(),
        ViewDistances::new(12)
    ));
}
