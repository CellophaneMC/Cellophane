pub use common::*;
pub use configuration::*;
pub use handshake::*;
pub use login::*;
pub use play::*;
pub use status::*;

use super::*;

mod configuration;
mod handshake;
mod login;
mod play;
mod status;
mod common;

enum_packets!(
    ClientHandshakePacket {
        0x00 = Handshake,
    }
);

enum_packets!(
    ClientStatusPacket {
        0x00 = Request,
        0x01 = Ping,
    }
);

enum_packets!(
    ClientLoginPacket {
        0x00 = LoginStart,
        0x01 = EncryptionResponse,
        0x02 = LoginPluginResponse,
        0x03 = LoginAck,
    }
);

enum_packets!(
    ClientConfigurationPacket {
        0x00 = ClientInformation,
        0x01 = PluginMessageConfiguration,
        0x02 = FinishConfiguration,
        0x03 = KeepAliveResponse,
        0x04 = Pong,
        0x05 = ResourcePack,
    }
);

enum_packets!(
    ClientPlayPacket {

        // 0x00 = ConfirmTeleportation,
        0x01 = BlockEntityTagQuery,
        0x02 = ChangeDifficulty,

        0x03 = ChatAck,
        0x04 = ChatCommand,
        0x05 = ChatMessage,
        // 0x06 = ChatSessionUpdate

        0x07 = ChunkBatchReceived,
        0x08 = ClientAction,
        0x09 = ClientInformation,
        0x0A = CommandSuggestion,
        0x0B = ConfigurationAck,

        0x0C = ContainerButtonClick,
        0x0D = ContainerClick,
        0x0E = ContainerClose,
        0x0F = ContainerSlotStateChanged,

        //0x10 = CUSTOM PAYLOAD
        0x11 = EditBook,
        0x12 = EntityTagQuery,
        0x13 = Interact,
        0x14 = JigsawGenerate,
        0x15 = KeepAliveResponse,
        0x16 = LockDifficulty,
        0x17 = PlayerUpdatePosition,
        0x18 = PlayerUpdatePositionRotation,
        0x19 = PlayerUpdateRotation,
        0x1A = PlayerUpdateOnGround,


    }
);

#[derive(Debug, Clone)]
pub enum ClientPacket {
    Handshake(ClientHandshakePacket),
    Status(ClientStatusPacket),
    Login(ClientLoginPacket),
    Configuration(ClientConfigurationPacket),
    Play(ClientPlayPacket),
}
