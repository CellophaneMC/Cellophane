use serde::{Deserialize, Serialize};

pub enum Sound {}

#[derive(Debug, Clone, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Music {
    sound: String,
    min_delay: i32,
    max_delay: i32,
    replace_current_music: bool,
}
