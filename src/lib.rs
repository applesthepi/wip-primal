extern crate wip_pm;

mod positions;
mod bounds;

pub use positions::*;
pub use bounds::*;

pub const LOD_SAMPLE_WIDTH: usize = 3;
pub const CHUNK_SIZE_FACTOR: u32 = 3;

pub const CHUNK_WIDTH: usize = LOD_SAMPLE_WIDTH.pow(CHUNK_SIZE_FACTOR);
pub const SECTOR_WIDTH: u16 = 16;