use std::collections::{BTreeMap, HashMap};
use std::iter::Map;
use serde::{Deserialize, Serialize};
use crate::entity::EntityCategory;
use crate::generation::CarvingSteps;

use crate::particle::Particle;
use crate::sound::Music;

#[derive(Debug, Clone, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Biome {
    #[cfg_attr(feature = "serde", serde(flatten))]
    climate: BiomeClimate,
    effects: BiomeEffects,
    #[cfg_attr(feature = "serde", serde(flatten))]
    generation: BiomeGeneration,
    #[cfg_attr(feature = "serde", serde(flatten))]
    mob_spawn: BiomeMobSpawn,
}

#[derive(Debug, Clone, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BiomeClimate {
    has_precipitation: bool,
    temperature: f32,
    #[cfg_attr(feature = "serde", serde(default))]
    temperature_modifier: TemperatureModifier,
    downfall: f32,
}

#[derive(Debug, Clone, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum TemperatureModifier {
    #[default]
    #[cfg_attr(feature = "serde", serde(rename = "none"))]
    None,
    #[cfg_attr(feature = "serde", serde(rename = "frozen"))]
    Frozen,
}

#[derive(Debug, Clone, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BiomeEffects {
    fog_color: i32,
    water_color: i32,
    water_fog_color: i32,
    sky_color: i32,
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none"))]
    foliage_color: Option<i32>,
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none"))]
    grass_color: Option<i32>,
    #[cfg_attr(feature = "serde", serde(default))]
    grass_color_modifier: GrassColorModifier,
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none"))]
    particle: Option<AmbientParticle>,
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none"))]
    ambient_sound: Option<String>,
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none"))]
    mood_sound: Option<AmbientMoodSound>,
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none"))]
    additions_sound: Option<AmbientAdditionsSound>,
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none"))]
    music: Option<Music>,
}

#[derive(Debug, Clone, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum GrassColorModifier {
    #[default]
    #[cfg_attr(feature = "serde", serde(rename = "none"))]
    None,
    #[cfg_attr(feature = "serde", serde(rename = "dark_forest"))]
    DarkForest,
    #[cfg_attr(feature = "serde", serde(rename = "swamp"))]
    Swamp,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AmbientParticle {
    options: Particle,
    probability: f32,
}

#[derive(Debug, Clone, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AmbientMoodSound {
    sound: String,
    tick_delay: i32,
    block_search_extent: i32,
    offset: f64,
}

#[derive(Debug, Clone, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AmbientAdditionsSound {
    sound: String,
    tick_chance: f64,
}

#[derive(Debug, Clone, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BiomeGeneration {
    carvers: BTreeMap<CarvingSteps, Carver>,
    features: Vec<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(untagged))]
pub enum Carver {
    Single(String),
    Array(Vec<String>),
}

#[derive(Debug, Clone, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BiomeMobSpawn {
    #[cfg_attr(feature = "serde", serde(default))]
    creature_spawn_probability: f32,
    spawn_costs: BTreeMap<String, BiomeMobSpawnCost>,
    spawners: BTreeMap<EntityCategory, Vec<BiomeNaturalSpawner>>,
}

#[derive(Debug, Clone, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BiomeMobSpawnCost {
    energy_budget: f64,
    charge: f64,
}

#[derive(Debug, Clone, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BiomeNaturalSpawner {
    #[cfg_attr(feature = "serde", serde(rename = "type"))]
    entity_type: String,
    weight: i32,
    #[cfg_attr(feature = "serde", serde(rename = "minCount"))]
    min_count: u32,
    #[cfg_attr(feature = "serde", serde(rename = "maxCount"))]
    max_count: u32,
}
