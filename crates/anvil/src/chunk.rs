use serde_derive::Deserialize;

use cellophanemc_nbt::Compound;

#[derive(Debug, Deserialize)]
pub struct Chunk {
    /// Version of the chunk NBT structure.
    #[serde(rename = "DataVersion")]
    pub data_version: i32,
    /// X position of the chunk (in absolute chunks from the world `x, z` origin, not relative to the region).
    #[serde(rename = "xPos")]
    pub x_pos: i32,
    /// Z position of the chunk (in absolute chunks from the world `x, z` origin, not relative to the region).
    #[serde(rename = "zPos")]
    pub z_pos: i32,
    /// Lowest Y section position in the chunk. (e.g -4 in 1.18).
    #[serde(rename = "yPos")]
    pub y_pos: i32,
    /// Defines the world generation status of this chunk.
    /// All status except `minecraft:full` are used for chunks called proto-chunks, in other words, for chunks with incomplete generation.
    #[serde(rename = "Status")]
    pub status: String,
    /// Tick when the chunk was last saved.
    #[serde(rename = "LastUpdate")]
    pub last_update: i64,
    /// List of compound tags, each tag is a section (also known as sub-chunk).
    /// All sections in the world's height are present in this list,
    /// even those who are empty (filled with air).
    pub sections: Vec<Section>,
    /// Each TAG_Compound in this list defines a block entity in the chunk.
    pub block_entities: Option<Vec<Compound>>,
    /// Several different heightmaps corresponding to 256 values compacted at 9 bits per value
    /// (lowest being 0, highest being 384, both values inclusive).
    /// The 9 bit values are stored in an array of 37 longs, each containing 7 values
    /// (long = 64 bits, 7×9 = 63; the last bit is unused).
    #[serde(rename = "Heightmaps")]
    pub heightmaps: Option<Heightmaps>,
    /// List is an "active" liquid in this chunk waiting to be updated.
    pub fluid_ticks: Vec<SavedTick>,
    /// List is an "active" block in this chunk waiting to be updated.
    pub block_ticks: Vec<SavedTick>,
    /// The cumulative number of ticks players have been in this chunk.
    #[serde(rename = "InhabitedTime")]
    pub inhabited_time: i64,
    pub blending_data: Option<BlendingData>,
    #[serde(rename = "PostProcessing")]
    pub post_processing: Vec<Vec<i16>>,
    pub structures: Structures,
}

#[derive(Debug, Deserialize)]
pub struct SavedTick {
    #[serde(rename = "i")]
    pub id: String,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    #[serde(rename = "p")]
    pub priority: i32,
    #[serde(rename = "t")]
    pub delay: i32,
}

#[derive(Debug, Deserialize)]
pub struct Section {
    #[serde(rename = "Y")]
    pub y: i8,
    #[serde(rename = "block_states")]
    pub block_states: PackedPalettedContainer<Compound>,
    #[serde(rename = "biomes")]
    pub biomes: PackedPalettedContainer<String>,
    #[serde(rename = "SkyLight")]
    pub sky_light: Option<Vec<i8>>,
    #[serde(rename = "BlockLight")]
    pub block_light: Option<Vec<i8>>,
}

#[derive(Debug, Deserialize)]
pub struct PackedPalettedContainer<T> {
    pub palette: Vec<T>,
    pub data: Option<Vec<i64>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Heightmaps {
    pub motion_blocking: Option<Vec<i64>>,
    pub motion_blocking_no_leaves: Option<Vec<i64>>,
    pub ocean_floor: Option<Vec<i64>>,
    pub ocean_floor_wg: Option<Vec<i64>>,
    pub world_surface: Option<Vec<i64>>,
    pub world_surface_wg: Option<Vec<i64>>,
}

#[derive(Debug, Deserialize)]
pub struct Structures {
    #[serde(rename = "References")]
    pub references: Compound,
    pub starts: Compound,
}

#[derive(Debug, Deserialize)]
pub struct BlendingData {
    pub min_section: i32,
    pub max_section: i32,
    pub heights: Option<Vec<f64>>,
}
