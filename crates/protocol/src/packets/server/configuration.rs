use cellophanemc_nbt::{Compound, Value};

use super::*;

packets! {
    PluginMessage {
        channel String;
        data LengthInferredVecU8;
    }

    Disconnect {
        reason String;
    }

    FinishConfiguration {
    }

    KeepAlive {
        id i64;
    }

    Ping {
        id i64;
    }

    RegistryData {
        data Compound;
    }

    RemoveResourcePack {
        uuid Option<Uuid>;
    }

    AddResourcePack {
        uuid Uuid;
        url String;
        hash String;
        forced bool;
        promt_message Option<Value>;
    }

    FeatureFlags {
        flags VarIntPrefixedVec<String>
    }

    UpdateTags {
        tags VarIntPrefixedVec<Tags>;
    }

    Tags {
        registry String;
        tags VarIntPrefixedVec<Tag>;
    }

    Tag {
        name String;
        entries VarIntPrefixedVec<VarInt>;
    }
}

// #[derive(Debug, Clone)]
// pub struct RegistryData {
//     pub data: Nbt,
// }
//
// #[allow(clippy::useless_conversion)]
// #[allow(unused_imports)]
// #[allow(unused_variables)]
// impl crate::Decoder for RegistryData {
//     fn read(reader: &mut impl std::io::Read) -> crate::error::Result<Self> {
//         let data = <Nbt
//         >::read(reader)
//             .map_err(|e| crate::error::Error::FieldDecode {
//                 field: stringify!( data ).to_string(),
//                 packet: stringify!( RegistryData ).to_string(),
//                 source: Box::new((e)),
//             })?
//             .into();
//
//         Ok(Self {
//             data,
//         })
//     }
// }
// #[allow(clippy::useless_conversion)]
// #[allow(unused_imports)]
// #[allow(unused_variables)]
// impl crate::Encoder for RegistryData {
//     fn write(&self, writer: &mut impl std::io::Write) -> crate::error::Result<()> {
//         (self.data)
//             .write(writer)
//             .map_err(|e| crate::error::Error::FieldEncode {
//                 field: stringify!( data ).to_string(),
//                 packet: stringify!( RegistryData ).to_string(),
//                 source: Box::new(crate::error::Error::from((e))),
//             })?;
//
//         Ok(())
//     }
// }
