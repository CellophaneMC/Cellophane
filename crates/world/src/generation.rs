use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum CarvingSteps {
    #[cfg_attr(feature = "serde", serde(rename = "air"))]
    Air,
    #[cfg_attr(feature = "serde", serde(rename = "liquid"))]
    Liquid,
}
