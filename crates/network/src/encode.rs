#[cfg(feature = "compression")]
use libdeflater::{CompressionLvl, Compressor};
use tokio::io;
use tokio::io::AsyncWriteExt;

use cellophanemc_protocol::VarInt;

pub struct PacketEncoder {
    #[cfg(feature = "compression")]
    threshold: i32,
}

impl Default for PacketEncoder {
    fn default() -> Self {
        Self {
            #[cfg(feature = "compression")]
            threshold: -1,
        }
    }
}

impl PacketEncoder {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn write_frame(
        &mut self,
        dest: &mut (impl io::AsyncWrite + Unpin + Send),
        data: &[u8],
    ) -> anyhow::Result<()> {
        let data_len = VarInt(data.len() as i32);

        #[cfg(feature = "compression")]
        if self.threshold >= 0 {
            if data_len.0 > self.threshold {
                let mut compressor = Compressor::new(CompressionLvl(4));
                let max_size = compressor.gzip_compress_bound(data_len.0 as usize);
                let mut compressed = vec![0; max_size];
                let actual_size = VarInt(compressor.gzip_compress(data, &mut compressed)? as i32);

                VarInt((actual_size.written_size() + actual_size.0) as i32).write_async(dest).await?;
                actual_size.write_async(dest).await?;
                dest.write_all(&compressed).await?;
            } else {
                let data_len_size = 1;
                VarInt((data_len_size + data_len.0)).write_async(dest).await?;
                dest.write_u8(0).await?;
                dest.write_all(data).await?
            }
        } else {
            data_len.write_async(dest).await?;
            dest.write_all(data).await?
        }

        #[cfg(not(feature = "compression"))]
        {
            data_len.write_async(dest).await?;
            dest.write_all(data).await?
        }

        Ok(())
    }
}
