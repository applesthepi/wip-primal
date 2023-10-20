mod positions;
mod bounds;

pub use positions::*;
pub use bounds::*;

pub const LOD_SAMPLE_WIDTH: u8 = 3;
pub const CHUNK_SIZE_FACTOR: u32 = 3;

pub const CHUNK_WIDTH: u8 = LOD_SAMPLE_WIDTH.pow(CHUNK_SIZE_FACTOR);
pub const CHUNK_WIDTH_SQ: usize = CHUNK_WIDTH as usize * CHUNK_WIDTH as usize;
pub const SECTOR_WIDTH: u16 = 3; // TODO: will be over 16 after stack issues are fixed.