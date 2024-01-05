use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum Particle {
    #[cfg_attr(feature = "serde", serde(rename = "minecraft:block"))]
    Block,
    #[cfg_attr(feature = "serde", serde(rename = "minecraft:block_marker"))]
    BlockMarker,
    #[cfg_attr(feature = "serde", serde(rename = "minecraft:bubble"))]
    Bubble,
    #[cfg_attr(feature = "serde", serde(rename = "minecraft:cloud"))]
    Cloud,
    #[cfg_attr(feature = "serde", serde(rename = "minecraft:crit"))]
    Crit,
    #[cfg_attr(feature = "serde", serde(rename = "minecraft:damage_indicator"))]
    DamageIndicator,
    #[cfg_attr(feature = "serde", serde(rename = "minecraft:dragon_breath"))]
    DragonBreath,

    #[cfg_attr(feature = "serde", serde(rename = "minecraft:crimson_spore"))]
    CrimsonSpore,
    #[cfg_attr(feature = "serde", serde(rename = "minecraft:white_ash"))]
    WhiteAsh,
    #[cfg_attr(feature = "serde", serde(rename = "minecraft:ash"))]
    Ash,
    #[cfg_attr(feature = "serde", serde(rename = "minecraft:warped_spore"))]
    WarpedSpore,
}
