use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum EntityCategory {
    #[cfg_attr(feature = "serde", serde(rename = "ambient"))]
    Ambient,
    #[cfg_attr(feature = "serde", serde(rename = "axolotls"))]
    Axolotls,
    #[cfg_attr(feature = "serde", serde(rename = "creature"))]
    Creature,
    #[cfg_attr(feature = "serde", serde(rename = "misc"))]
    Misc,
    #[cfg_attr(feature = "serde", serde(rename = "monster"))]
    Monster,
    #[cfg_attr(feature = "serde", serde(rename = "underground_water_creature"))]
    UndergroundWaterCreature,
    #[cfg_attr(feature = "serde", serde(rename = "water_ambient"))]
    WaterAmbient,
    #[cfg_attr(feature = "serde", serde(rename = "water_creature"))]
    WaterCreature,
}
