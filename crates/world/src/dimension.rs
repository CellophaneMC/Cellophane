use serde::{Deserialize, Serialize};

use cellophanemc_data::IntSampler;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DimensionType {
    #[serde(skip_serializing_if = "Option::is_none")]
    fixed_time: Option<i64>,
    has_skylight: bool,
    has_ceiling: bool,
    ultrawarm: bool,
    natural: bool,
    coordinate_scale: f64,
    bed_works: bool,
    respawn_anchor_works: bool,
    min_y: i32,
    height: i32,
    logical_height: i32,
    infiniburn: String,
    effects: String,
    ambient_light: f32,
    piglin_safe: bool,
    has_raids: bool,
    monster_spawn_light_level: IntSampler,
    monster_spawn_block_light_limit: i32,
}

enum DimensionEffect {
    Overworld,
    Nether,
    End,
}

impl DimensionEffect {
    fn clouds_height(&self) -> f32 {
        match self {
            DimensionEffect::Overworld => 192.0,
            DimensionEffect::Nether => f32::NAN,
            DimensionEffect::End => f32::NAN,
        }
    }

    fn is_alternate_sky_color(&self) -> bool {
        match self {
            DimensionEffect::Overworld => false,
            DimensionEffect::Nether => false,
            DimensionEffect::End => true,
        }
    }

    fn sky_type(&self) -> SkyType {
        match self {
            DimensionEffect::Overworld => SkyType::Normal,
            DimensionEffect::Nether => SkyType::None,
            DimensionEffect::End => SkyType::End,
        }
    }

    fn should_brighten_lighting(&self) -> bool {
        match self {
            DimensionEffect::Overworld => false,
            DimensionEffect::Nether => false,
            DimensionEffect::End => true,
        }
    }

    fn is_darkened(&self) -> bool {
        match self {
            DimensionEffect::Overworld => false,
            DimensionEffect::Nether => true,
            DimensionEffect::End => false,
        }
    }
}

pub enum SkyType {
    None,
    Normal,
    End,
}
