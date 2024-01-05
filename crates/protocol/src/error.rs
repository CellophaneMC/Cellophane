use std::str::Utf8Error;

use crate::var_int::VarIntDecodeError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("I/O error")]
    Io(#[from] std::io::Error),
    #[error("VarInt error")]
    VarInt(#[from] VarIntDecodeError),
    #[error("Invalid discriminant {0}")]
    InvalidDiscriminant(i32),
    #[error("Invalid UTF-8")]
    Utf8(#[from] Utf8Error),
    #[error("BlockPos out of range")]
    BlockPosOutOfRange(#[from] cellophanemc_core::block_pos::Error),
    #[error("failed to decode field `{field}` of packet `{packet}`")]
    FieldDecode {
        field: String,
        packet: String,
        #[source]
        source: Box<Error>,
    },
    #[error("failed to encode field `{field}` of packet `{packet}`")]
    FieldEncode {
        field: String,
        packet: String,
        #[source]
        source: Box<Error>,
    },
    #[error("BitSet larger than expected: {0} > {1}")]
    BitSetLargerThanExpected(usize, usize),
}

pub type Result<T> = core::result::Result<T, Error>;
