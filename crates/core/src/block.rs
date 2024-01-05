use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::block_state::BlockState;
use crate::state::State;

#[derive(Debug, Default, Clone, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Block {
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "BTreeMap::is_empty"))]
    properties: BTreeMap<String, Vec<String>>,
    states: Vec<BlockState>,
}

impl Block {
    pub fn default_block_state(&self) -> Option<&BlockState> {
        self.states.iter().find(|&x| {
            x.is_default()
        })
    }
}

#[cfg(test)]
mod test {
    use std::collections::BTreeMap;
    use std::fs::File;
    use std::io;

    use crate::block::Block;

    #[test]
    fn foo() {
        let file = File::open("/Users/andreypfau/CLionProjects/karbon-rs/assets/minecraft/reports/blocks.json").unwrap();
        let reader = io::BufReader::new(file);

        let result: BTreeMap<String, Block> = serde_json::from_reader(reader).unwrap();
        // println!("blocks: {:?}", result);

        println!("bedrock: {:?}", result.get("minecraft:bedrock"))
    }
}
