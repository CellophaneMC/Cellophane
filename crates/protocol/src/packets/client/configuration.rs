use super::*;

packets!(
    PluginMessageConfiguration {
        channel String;
        data LengthInferredVecU8;
    }

    FinishConfiguration {}

    Pong {
        id i64;
    }

    ResourcePack {
        uuid Uuid;
        result VarInt;
    }
);
