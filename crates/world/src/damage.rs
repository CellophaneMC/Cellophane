use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DamageType {
    message_id: String,
    scaling: DamageScaling,
    exhaustion: f32,
    #[serde(default = "Default::default")]
    effects: DamageEffects,
    #[serde(default = "Default::default")]
    death_message_type: DeathMessageType,
}

impl DamageType {
    pub fn message_id(&self) -> &String {
        &self.message_id
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum DamageScaling {
    #[serde(rename = "never")]
    Never,
    #[serde(rename = "when_caused_by_living_non_player")]
    WhenCausedByLivingNonPlayer,
    #[serde(rename = "always")]
    Always,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub enum DamageEffects {
    #[default]
    #[serde(rename = "hurt")]
    Hurt,
    #[serde(rename = "thorns")]
    Thorns,
    #[serde(rename = "drowning")]
    Drowning,
    #[serde(rename = "burning")]
    Burning,
    #[serde(rename = "poking")]
    Poking,
    #[serde(rename = "freezing")]
    Freezing,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub enum DeathMessageType {
    #[default]
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "fall_variants")]
    FallVariants,
    #[serde(rename = "intentional_game_design")]
    IntentionalGameDesign,
}
