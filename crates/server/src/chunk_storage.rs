use std::fs::File;
use std::io::{Error, Read, Seek, SeekFrom};
use std::path::Path;

use bevy_reflect::TypeInfo::Value;
use bit_set::BitSet;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use flate2::bufread::{GzDecoder, ZlibDecoder};
use tokio::io::AsyncWriteExt;

use cellophanemc_core::chunk_pos::ChunkPos;
use cellophanemc_nbt::aa::Value;

pub struct RegionFile {
    file: File,
    sector_data: [u32; 1024],
    timestamps: [u32; 1024],
    used_sectors: BitSet,
}

impl RegionFile {
    fn open<P: AsRef<Path>>(path: P) -> Result<Self, std::io::Error> {
        let mut file = File::open(path)?;
        let mut sector_data = [0u32; 1024];
        file.read_u32_into::<BigEndian>(&mut sector_data)?;
        let mut timestamps = [0u32; 1024];
        file.read_u32_into::<BigEndian>(&mut timestamps)?;
        let mut used_sectors = BitSet::with_capacity(1024);
        for i in 0..1024 {
            let sector = ChunkSector(sector_data[i]);
            if sector.0 != 0 {
                let offset = sector.get_offset();
                let size = sector.get_length();
                for j in 0..size {
                    used_sectors.insert((offset + j) as usize);
                }
            }
        }

        Ok(Self {
            file,
            sector_data,
            timestamps,
            used_sectors,
        })
    }

    fn set_sectors(&mut self, start: usize, size: usize) {
        for i in start..start + size {
            self.used_sectors.insert(i);
        }
    }

    fn allocate_sectors(&mut self, size: usize) -> usize {
        let mut start = 0;
        let mut count = 0;
        loop {
            if !self.used_sectors.contains(start) {
                count += 1;
                if count == size {
                    break;
                }
            } else {
                start += 1;
                count = 0;
            }
        }
        self.set_sectors(start, size);
        start
    }

    fn free_sectors(&mut self, start: usize, size: usize) {
        for i in start..start + size {
            self.used_sectors.remove(i);
        }
    }

    const fn chunk_pos_index(pos: &ChunkPos) -> usize {
        (pos.x as usize & 31) + (pos.z as usize & 31) * 32
    }

    const fn get_chunk_sector(&self, pos: &ChunkPos) -> ChunkSector {
        let index = Self::chunk_pos_index(pos);
        ChunkSector(self.sector_data[index])
    }

    const fn get_chunk_timestamp(&self, pos: &ChunkPos) -> u32 {
        let index = Self::chunk_pos_index(pos);
        let timestamp = self.timestamps[index];
        timestamp
    }

    fn read_chunk_nbt(&mut self, pos: &ChunkPos) -> Result<Value::Compound, Error> {
        let mut data = Vec::new();
        self.read_chunk(pos, &mut data)?;
        let length = u32::from_be_bytes([data[0], data[1], data[2], data[3]]) as usize - 1;
        let compression_type = data[4];
        let compressed_data = &data[5..5 + length];
        let decompressed_data = match compression_type {
            0 => {
                // uncompressed
                compressed_data
            }
            1 => {
                let mut decoder = GzDecoder::new(compressed_data);
                let mut decompressed_data = Vec::new();
                decoder.read_to_end(&mut decompressed_data)?;
                decompressed_data
            }
            2 => {
                let mut decoder = ZlibDecoder::new(compressed_data);
                let mut decompressed_data = Vec::new();
                decoder.read_to_end(&mut decompressed_data)?;
                decompressed_data
            }
            _ => return Err(Error::new(std::io::ErrorKind::Other, "Unsupported compression type"))
        };
    }

    fn read_chunk(&mut self, pos: &ChunkPos, dst: &mut Vec<u8>) -> Result<usize, std::io::Error> {
        let sector = self.get_chunk_sector(pos);
        if sector.0 == 0 {
            return Ok(0);
        }
        let offset = sector.get_offset();
        let size = sector.get_length();
        if size == 255 {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Chunk is too big"));
        }
        let bytes_size = size as usize * 4096;
        dst.resize(bytes_size, 0);
        self.file.seek(SeekFrom::Start((offset as u64) * 4096))?;
        self.file.read_exact(&mut dst[..bytes_size])?;
        return Ok(bytes_size);


        // self.reader.seek(SeekFrom::Start((offset as u64) * 4096))?;
        // self.reader.read_exact(&mut data)?;
        //
        // let compressed_length = u32::from_be_bytes([data[0], data[1], data[2], data[3]]) as usize - 1;
        // let compression_type = data[4];
        // let compressed_data = &data[5..5 + compressed_length];
        //
        // let current_dst_len = dst.len();
        // match compression_type {
        //     0 => {
        //         dst.extend_from_slice(compressed_data);
        //     }
        //     1 => {
        //         let mut decoder = GzDecoder::new(compressed_data);
        //         decoder.read_to_end(dst)?;
        //     }
        //     2 => {
        //         let mut decoder = ZlibDecoder::new(compressed_data);
        //         decoder.read_to_end(dst)?;
        //     }
        //     _ => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Unsupported compression type"))
        // }
        //
        // Ok(dst.len() - current_dst_len)
    }

    fn write_chunk(&mut self, pos: &ChunkPos, data: &[u8]) -> Result<(), std::io::Error> {
        let index = Self::chunk_pos_index(pos);

        let sector = ChunkSector(self.sector_data[index]);
        let current_length = sector.get_length() as usize;
        let new_length = ChunkSector::size_to_sectors(data.len());
        let current_offset = sector.get_offset() as usize;

        let new_offset = if new_length < current_length {
            current_offset
        } else if current_length < new_length {
            self.allocate_sectors(new_length)
        } else {
            current_offset
        };

        self.file.seek(SeekFrom::Start((new_offset as u64) * 4096))?;
        self.file.write(data)?;

        self.sector_data[index] = ChunkSector::new(new_offset, new_length).0;
        self.timestamps[index] = 0;

        self.write_header()?;

        if new_length < current_length {
            self.free_sectors(new_offset + new_length, current_length - new_length);
        } else if current_length < new_length {
            self.free_sectors(current_offset, current_length);
        }
        Ok(())
    }

    fn write_header(&mut self) -> Result<(), std::io::Error> {
        self.file.seek(SeekFrom::Start(0))?;
        for i in 0..1024 {
            self.file.write_u32::<BigEndian>(self.sector_data[i])?;
        }
        for i in 0..1024 {
            self.file.write_u32::<BigEndian>(self.timestamps[i])?;
        }
        Ok(())
    }
}

struct ChunkSector(u32);

impl ChunkSector {
    fn new(offset: usize, length: usize) -> Self {
        Self(((offset << 8) & 0xFFFFFF) as u32 | (length & 0xFF) as u32)
    }

    const fn get_length(&self) -> u32 {
        self.0 & 0xFF
    }

    const fn get_offset(&self) -> u32 {
        (self.0 >> 8) & 0xFFFFFF
    }

    const fn size_to_sectors(size: usize) -> usize {
        (size + 4095) / 4096
    }
}

#[cfg(test)]
mod test {
    use cellophanemc_core::chunk_pos::ChunkPos;

    use crate::chunk_storage::RegionFile;

    #[test]
    fn foo() {
        let mut region = RegionFile::open("/Users/andreypfau/IdeaProjects/pfaumc/pfaumc-minigame/run/bedwars_lobby/region/r.0.0.mca").expect("Failed to open region file");
        let mut chunk_data = Vec::new();
        let read = region.read_chunk(&ChunkPos::new(0, 0), &mut chunk_data).expect("Failed to read chunk");
        println!("Chunk len: {:?}", chunk_data.len());
        println!("read len: {:?}", read);
    }
}
