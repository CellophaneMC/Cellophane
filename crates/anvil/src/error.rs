#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("I/O error")]
    Io(#[from] std::io::Error),
    #[error("Unknown compression scheme: {0}")]
    UnknownCompression(u8),
    #[error("Invalid chunk header size: {0}")]
    InvalidChunkHeaderSize(usize),
    #[error("Failed to parse NBT: {0}")]
    Nbt(#[from] cellophanemc_nbt::error::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
