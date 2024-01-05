#[cfg(feature = "compression")]
use libdeflater::Decompressor;
use tokio::io::{AsyncRead, AsyncReadExt};

use cellophanemc_protocol::VarInt;

pub struct PacketDecoder {
    #[cfg(feature = "compression")]
    threshold: i32,
}

impl Default for PacketDecoder {
    fn default() -> Self {
        Self {
            #[cfg(feature = "compression")]
            threshold: -1,
        }
    }
}

impl PacketDecoder {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn read_frame(&mut self, reader: &mut (impl AsyncRead + Unpin + Send)) -> anyhow::Result<bytes::Bytes> {
        let len = VarInt::read_async(reader).await?;

        let data: Vec<u8>;

        #[cfg(feature = "compression")]
        {
            data = if self.threshold >= 0 {
                let data_len = VarInt::read_async(reader).await?;
                let mut compressed = vec![0; (len.0 - data_len.written_size()) as usize];
                reader.read_exact(&mut compressed).await?;

                if data_len.0 > 0 {
                    let mut decompressed = vec![0; data_len.0 as usize];

                    let mut decompressor = Decompressor::new();
                    decompressor.zlib_decompress(&compressed, &mut decompressed)?;

                    decompressed
                } else {
                    compressed
                }
            } else {
                let mut buf = vec![0; len.0 as usize];
                reader.read_exact(&mut buf).await?;
                buf
            };
        }

        #[cfg(not(feature = "compression"))]
        {
            let mut buf = vec![0; len.0 as usize];
            reader.read_exact(&mut buf).await?;
            data = buf;
        };

        Ok(data.into())
    }
}
