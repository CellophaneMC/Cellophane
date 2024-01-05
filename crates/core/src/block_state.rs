use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::state::State;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BlockStateId(pub u16);

impl From<u16> for BlockStateId {
    #[inline(always)]
    fn from(value: u16) -> Self {
        Self(value)
    }
}

impl From<BlockStateId> for u16 {
    #[inline(always)]
    fn from(value: BlockStateId) -> Self {
        value.0
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BlockState {
    pub id: BlockStateId,
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "BlockState::skip_serializing_if_default_false"))]
    pub default: bool,
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "BTreeMap::is_empty"))]
    pub properties: BTreeMap<String, String>,
}

impl BlockState {
    pub const fn new(id: BlockStateId, default: bool, properties: BTreeMap<String, String>) -> Self {
        Self {
            id,
            default,
            properties,
        }
    }

    pub fn skip_serializing_if_default_false(value: &bool) -> bool {
        !*value
    }
}

impl Default for BlockState {
    fn default() -> Self {
        Self {
            id: 0.into(),
            default: false,
            properties: BTreeMap::default(),
        }
    }
}

impl State for BlockState {
    fn id(&self) -> u16 {
        self.id.0
    }

    fn state_property(&self, name: &str) -> Option<&str> {
        self.properties.get(name).map(|a| a.as_str())
    }

    fn is_default(&self) -> bool {
        self.default
    }
}

impl Into<u16> for BlockState {
    fn into(self) -> u16 {
        self.id.0
    }
}

#[cfg(test)]
mod test {
    use crate::block_state::BlockState;

    #[test]
    fn foo() {
        let a = "{\"id\":8707,\"properties\":{\"face\":\"floor\",\"facing\":\"north\",\"powered\":\"true\"}}";
        let result = serde_json::from_str::<BlockState>(a);
        println!("{:?}", result)
    }
}
