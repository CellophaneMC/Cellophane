use bevy_ecs::prelude::Component;
use bevy_ecs::prelude::ReflectComponent;
use bevy_reflect::Reflect;

use cellophanemc_profile::GameProfile;
use cellophanemc_protocol::packets::client::ClientInformation;

#[derive(Debug, Component, Default, Reflect)]
#[reflect(Component)]
pub struct Player {
    pub profile: GameProfile,
}

impl Player {
    pub fn new(profile: GameProfile) -> Self {
        Self {
            profile
        }
    }
}

#[derive(Debug, Component, Reflect)]
pub struct PlayerSettings {
    pub locale: String,
    pub view_distance: u8,
    pub chat_mode: i32,
    pub chat_colors: bool,
    pub displayed_skin_parts: u8,
    pub main_hand: i32,
    pub enable_text_filtering: bool,
    pub allow_server_listings: bool,
}

impl From<&ClientInformation> for PlayerSettings {
    fn from(value: &ClientInformation) -> Self {
        PlayerSettings {
            locale: value.locale.clone(),
            view_distance: value.view_distance,
            chat_mode: value.chat_mode,
            chat_colors: value.chat_colors,
            displayed_skin_parts: value.displayed_skin_parts,
            main_hand: value.main_hand,
            enable_text_filtering: value.enable_text_filtering,
            allow_server_listings: value.allow_server_listings,
        }
    }
}
