use serde::{Deserialize, Deserializer, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", content = "aa")]
pub enum IntSampler {
    #[serde(rename = "minecraft:constant")]
    Constant {
        value: i32
    },
    #[serde(rename = "minecraft:uniform")]
    Uniform {
        min_inclusive: i32,
        max_inclusive: i32,
    },
    #[serde(rename = "minecraft:biased_to_bottom")]
    BiasedToBottom {
        min_inclusive: i32,
        max_inclusive: i32,
    },
    #[serde(rename = "minecraft:clamped")]
    Clamped {
        source: Box<IntSampler>,
        min_inclusive: i32,
        max_inclusive: i32,
    },
    #[serde(rename = "minecraft:clamped_normal")]
    ClampedNormal {
        mean: f32,
        deviation: f32,
        min_inclusive: i32,
        max_inclusive: i32,
    },
}
