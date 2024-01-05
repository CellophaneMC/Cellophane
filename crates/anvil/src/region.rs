use std::io::{Read, Seek, SeekFrom, Write};

use bit_set::BitSet;

use crate::error::{Error, Result};

pub const SECTOR_SIZE: usize = 4096;
pub const REGION_HEADER_SIZE: usize = SECTOR_SIZE * 2;
pub const CHUNK_HEADER_SIZE: usize = 5;

#[derive(Clone)]
pub struct Region<S> {
    stream: S,
    sectors: BitSet,
}

impl<S> Region<S>
    where
        S: Read + Seek
{
    pub fn from_stream(stream: S) -> Self {
        let mut sectors = BitSet::with_capacity(1024);
        sectors.insert(0);
        sectors.insert(1);

        let mut stream = stream;

        for i in 0..1024 {
            let chunk_location = chunk_location(&mut stream, i);
            if let Some(chunk_location) = chunk_location {
                for j in 0..chunk_location.sectors {
                    sectors.insert(chunk_location.offset + j);
                }
            }
        }

        Self {
            stream,
            sectors,
        }
    }

    fn read_chunk_header(&mut self, chunk_location: ChunkLocation) -> Result<ChunkHeader> {
        let offset = chunk_location.offset * SECTOR_SIZE;
        self.stream.seek(SeekFrom::Start(offset as u64))?;

        let mut buf = [0u8; CHUNK_HEADER_SIZE];
        self.stream.read_exact(&mut buf)?;
        let chunk_header = ChunkHeader::from_bytes(&buf)?;

        Ok(chunk_header)
    }

    fn read_raw_chunk(
        &mut self,
        x: usize,
        z: usize,
        dst: &mut dyn Write,
    ) -> Result<u64> {
        let chunk_location = chunk_location(&mut self.stream, chunk_location_offset(x, z));
        if let Some(chunk_location) = chunk_location {
            let offset = chunk_location.offset * SECTOR_SIZE + CHUNK_HEADER_SIZE;
            let chunk_header = self.read_chunk_header(chunk_location)?;
            self.stream.seek(SeekFrom::Start(offset as u64))?;
            let mut src = (&mut self.stream).take(chunk_header.size as u64);

            let bytes = match chunk_header.compression_type {
                CompressionType::Gzip => {
                    let mut decoder = flate2::read::GzDecoder::new(src);
                    std::io::copy(&mut decoder, dst)?
                }
                CompressionType::Zlib => {
                    let mut decoder = flate2::read::ZlibDecoder::new(src);
                    std::io::copy(&mut decoder, dst)?
                }
                CompressionType::Uncompressed => {
                    std::io::copy(&mut src, dst)?
                }
            };
            Ok(bytes)
        } else {
            Ok(0)
        }
    }
}

fn chunk_location<S>(stream: &mut S, offset: u64) -> Option<ChunkLocation>
    where
        S: Read + Seek
{
    stream.seek(SeekFrom::Start(offset)).ok()?;

    let mut buf = [0u8; 4];
    stream.read_exact(&mut buf).ok()?;

    let mut offset = 0usize;
    offset |= (buf[0] as usize) << 16;
    offset |= (buf[1] as usize) << 8;
    offset |= buf[2] as usize;
    let sectors = buf[3] as usize;

    (offset != 0 || sectors != 0).then_some(ChunkLocation { offset, sectors })
}

const fn chunk_location_offset(chunk_x: usize, chunk_z: usize) -> u64 {
    (chunk_x & 31) as u64 + (chunk_z & 31) as u64 * 32
}

#[derive(Debug)]
struct ChunkLocation {
    // The offset, in units of 4 KiB sectors, into the region file where the chunk is stored.
    // Offset 0 is the start of the file.
    offset: usize,

    // The number of 4 KiB sectors that the chunk occupies in the region file.
    sectors: usize,
}

#[derive(Debug)]
struct ChunkHeader {
    // The length of the chunk data, in bytes, not counting the length or compression type bytes.
    size: usize,

    // The compression type used for the chunk data.
    compression_type: CompressionType,
}

impl ChunkHeader {
    fn from_bytes(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < CHUNK_HEADER_SIZE {
            return Err(Error::InvalidChunkHeaderSize(bytes.len()));
        }

        let size = u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as usize - 1;
        let compression_type = CompressionType::try_from(bytes[4])?;

        Ok(Self {
            size,
            compression_type,
        })
    }
}

#[derive(Debug, Default)]
pub enum CompressionType {
    Gzip = 1,
    #[default]
    Zlib = 2,
    Uncompressed = 3,
}

impl TryFrom<u8> for CompressionType {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self> {
        match value {
            1 => Ok(CompressionType::Gzip),
            2 => Ok(CompressionType::Zlib),
            3 => Ok(CompressionType::Uncompressed),
            _ => Err(Error::UnknownCompression(value))
        }
    }
}

#[cfg(test)]
mod test {
    use std::fs::File;

    use crate::region::Region;

    #[test]
    fn foo() {
        let file = File::open("/Users/andreypfau/IdeaProjects/pfaumc/pfaumc-minigame/run/bedwars_lobby/region/r.0.0.mca").expect("Failed to open region file");
        let mut region = Region::from_stream(file);
        // let mut chunk_data = Vec::new();
        // let read = region.read_chunk(&ChunkPos::new(0, 0), &mut chunk_data).expect("Failed to read chunk");
        // println!("Chunk len: {:?}", chunk_data.len());
        // println!("read len: {:?}", read);
        region.read_raw_chunk(0, 0, &mut std::io::stdout()).expect("Failed to read chunk");
    }
}
