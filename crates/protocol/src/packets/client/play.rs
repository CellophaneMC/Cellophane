use std::io::{Read, Write};

use cellophanemc_core::block_pos::PackedBlockPos;

use crate::Encoder;
use crate::types::{Hand, Slot};

use super::*;

packets!(

    ConfirmTeleportation {
        id VarInt;
    }

    BlockEntityTagQuery {
        transaction_id VarInt;
        location PackedBlockPos;
    }

    ChangeDifficulty {
        difficulty u8;
    }

    ChatAck {
        offset VarInt;
    }

    ChatCommand {
        command String;
        timestamp u64;
        salt u64;
        argument_signatures VarIntPrefixedVec<ArgumentSignature>;
        last_seen_messages LastSeenMessagesUpdate;
    }

    ChatMessage {
        message String;
        timestamp u64;
        salt u64;
        signature Option<MessageSignature>;
        last_seen_messages LastSeenMessagesUpdate;
    }

    LastSeenMessagesUpdate {
        offset VarInt;
        ack FixedBitSet<20>;
    }

    ArgumentSignature {
        name String;
        signature MessageSignature;
    }

    MessageSignature {
        signature Fixed256VecU8;
    }

    ChunkBatchReceived {
        desired_chunks_per_tick f32;
    }

    ClientAction {
        action VarInt;
    }

    CommandSuggestion {
        id VarInt;
        command String;
    }

    ConfigurationAck {
    }

    ContainerButtonClick {
        container_id u8;
        button_id u8;
    }

    ContainerClick {
        container_id u8;
        state_id VarInt;
        slot i16;
        bytton u8;
        click_type VarInt;
        changed_slots VarIntPrefixedVec<ContainerChangedSlot>;
        carried_item Slot;
    }

    ContainerChangedSlot {
        slot i16;
        item Slot;
    }

    ContainerClose {
        container_id u8;
    }

    ContainerSlotStateChanged {
        slot_id VarInt;
        container_id VarInt;
        new_state bool;
    }

    EditBook {
        slot VarInt;
        pages VarIntPrefixedVec<String>;
        title Option<String>;
    }

    EntityTagQuery {
        transaction_id VarInt;
        entity_id VarInt;
    }

    Interact {
        entity_id VarInt;
        action InteractAction;
        sneaking bool;
    }

    JigsawGenerate {
        position PackedBlockPos;
        levels VarInt;
        keep_jigsaws bool;
    }

    LockDifficulty {
        locked bool;
    }

    PlayerUpdatePosition {
        x f64;
        y f64;
        z f64;
        on_ground bool;
    }

    PlayerUpdatePositionRotation {
        x f64;
        y f64;
        z f64;
        yaw f32;
        pitch f32;
        on_ground bool;
    }

    PlayerUpdateRotation {
        yaw f32;
        pitch f32;
        on_ground bool;
    }

    PlayerUpdateOnGround {
        on_ground bool;
    }
);

#[derive(Debug, PartialEq, Clone)]
pub enum InteractAction {
    Interact {
        hand: Hand,
    },
    Attack,
    InteractAt {
        x: f64,
        y: f64,
        z: f64,
        hand: Hand,
    },
}

impl Decoder for InteractAction {
    fn read(reader: &mut impl Read) -> crate::error::Result<Self> {
        let action = VarInt::read(reader)?.0;
        match action {
            0 => Ok(InteractAction::Interact {
                hand: Hand::read(reader)?,
            }),
            1 => Ok(InteractAction::Attack),
            2 => Ok(InteractAction::InteractAt {
                x: f64::read(reader)?,
                y: f64::read(reader)?,
                z: f64::read(reader)?,
                hand: Hand::read(reader)?,
            }),
            _ => Err(crate::error::Error::InvalidDiscriminant(action)),
        }
    }
}

impl Encoder for InteractAction {
    fn write(&self, writer: &mut impl Write) -> crate::error::Result<()> {
        match self {
            InteractAction::Interact { hand } => {
                VarInt(0).write(writer)?;
                hand.write(writer)?;
            }
            InteractAction::Attack => {
                VarInt(1).write(writer)?;
            }
            InteractAction::InteractAt { x, y, z, hand } => {
                VarInt(2).write(writer)?;
                x.write(writer)?;
                y.write(writer)?;
                z.write(writer)?;
                hand.write(writer)?;
            }
        }
        Ok(())
    }
}
