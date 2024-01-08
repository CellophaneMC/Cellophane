use std::hash::Hash;
use std::io::{Read, Seek, SeekFrom, Write};

use bitfield_struct::bitfield;
use byteorder::{BigEndian, ReadBytesExt};

use cellophanemc_nbt::binary::decode::FromModifiedUtf8;
use cellophanemc_nbt::Compound;

use crate::error::{Error, Result};

pub const SECTOR_SIZE: usize = 4096;
pub const REGION_HEADER_SIZE: usize = SECTOR_SIZE * 2;
pub const CHUNK_HEADER_SIZE: usize = 5;

#[derive(Clone)]
pub struct Region<S> {
    stream: S,
    locations: [Location; 1024],
    timestamps: [u32; 1024],
    used_sectors: bitvec::vec::BitVec,
}

impl<F> Region<F>
where
    F: Read + Seek,
{
    pub fn from_stream(stream: F) -> Result<Self> {
        let mut stream = stream;
        let mut header = [0u8; REGION_HEADER_SIZE];
        stream.read_exact(&mut header)?;

        let locations = std::array::from_fn(|i| {
            Location(u32::from_be_bytes(
                header[i * 4..(i + 1) * 4].try_into().unwrap(),
            ))
        });
        let timestamps = std::array::from_fn(|i| {
            u32::from_be_bytes(
                header[SECTOR_SIZE + i * 4..SECTOR_SIZE + (i + 1) * 4]
                    .try_into()
                    .unwrap(),
            )
        });

        let mut used_sectors = bitvec::vec::BitVec::repeat(true, 2);
        for location in locations {
            if location.is_empty() {
                continue;
            }
            let (sector_offset, sector_count) = location.offset_and_count();
            if sector_offset < 2 {
                continue;
            }
            if sector_count == 0 {
                continue;
            }

            Self::reserve_sectors(&mut used_sectors, sector_offset, sector_count);
        }

        Ok(Self {
            stream,
            locations,
            timestamps,
            used_sectors,
        })
    }

    fn reserve_sectors(
        used_sectors: &mut bitvec::vec::BitVec,
        sector_offset: u64,
        sector_count: usize,
    ) {
        let start_index = sector_offset as usize;
        let end_index = sector_offset as usize + sector_count;
        if used_sectors.len() < end_index {
            used_sectors.resize(start_index, false);
            used_sectors.resize(end_index, true);
        } else {
            used_sectors[start_index..end_index].fill(true);
        }
    }

    fn read_raw_chunk<S>(&mut self, x: i32, z: i32) -> Result<Option<RawChunk<S>>>
    where
        S: FromModifiedUtf8 + Hash + Ord,
    {
        let chunk_idx = chunk_idx(x, z);
        let location = self.locations[chunk_idx];

        if location.is_empty() {
            return Ok(None);
        }

        let (sector_offset, sector_count) = location.offset_and_count();

        self.stream
            .seek(SeekFrom::Start(sector_offset * SECTOR_SIZE as u64))?;

        let exact_chunk_size = self.stream.read_u32::<BigEndian>()? as usize;
        let compression_type = CompressionType::try_from(self.stream.read_u8()?)?;

        let mut src = (&mut self.stream).take((exact_chunk_size - 1) as u64);
        let mut nbt_data = Vec::new();

        match compression_type {
            CompressionType::Gzip => {
                let mut decoder = flate2::read::GzDecoder::new(src);
                std::io::copy(&mut decoder, &mut nbt_data)?
            }
            CompressionType::Zlib => {
                let mut decoder = flate2::read::ZlibDecoder::new(src);
                std::io::copy(&mut decoder, &mut nbt_data)?
            }
            CompressionType::Uncompressed => std::io::copy(&mut src, &mut nbt_data)?,
        };

        let (nbt, _) = cellophanemc_nbt::binary::decode::from_binary(&mut nbt_data.as_slice())?;

        Ok(Some(RawChunk {
            data: nbt,
            timestamp: self.timestamps[chunk_idx],
        }))
    }
}

#[derive(Debug)]
pub struct RawChunk<S = String> {
    pub data: Compound<S>,
    pub timestamp: u32,
}

const fn chunk_idx(chunk_x: i32, chunk_z: i32) -> usize {
    (chunk_x & 31) as usize + (chunk_z & 31) as usize * 32
}

#[bitfield(u32)]
struct Location {
    count: u8,
    #[bits(24)]
    offset: u32,
}

impl Location {
    fn is_empty(self) -> bool {
        self.0 == 0
    }

    fn offset_and_count(self) -> (u64, usize) {
        (self.offset() as u64, self.count() as usize)
    }
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
            _ => Err(Error::UnknownCompression(value)),
        }
    }
}

#[cfg(test)]
mod test {
    use std::fs::File;

    use serde::Deserialize;

    use crate::chunk::Chunk;
    use crate::region::Region;

    #[test]
    fn foo() {
        let file = File::open("/Users/andreypfau/IdeaProjects/pfaumc/pfaumc-minigame/run/bedwars_lobby/region/r.0.0.mca").expect("Failed to open region file");
        let mut region = Region::from_stream(file).unwrap();
        // let mut chunk_data = Vec::new();
        // let read = region.read_chunk(&ChunkPos::new(0, 0), &mut chunk_data).expect("Failed to read chunk");
        // println!("Chunk len: {:?}", chunk_data.len());
        // println!("read len: {:?}", read);
        let raw_chunk = region
            .read_raw_chunk::<String>(0, 0)
            .expect("Failed to read raw chunk")
            .unwrap();
        let data = raw_chunk.data;

        let chunk = Chunk::deserialize(data.clone()).unwrap();
        println!("chunk: {:#?}", chunk);
        // aaa(data)
    }
}
