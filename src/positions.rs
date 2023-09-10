use wip_pm::WorldPosition;

use crate::CHUNK_WIDTH;

#[derive(WorldPosition, Debug, Default, Clone, Copy)]
pub struct TilePositionAbs {
	pub x: i64,
	pub y: i64,
}

impl TilePositionAbs {
	pub fn new(
		x: i64,
		y: i64,
	) -> Self {
		Self { x, y }
	}

	pub fn splat(
		v: i64,
	) -> Self {
		Self { x: v, y: v, }
	}

	pub fn into_chunk_abs(
		self,
	) -> ChunkPositionAbs {
		ChunkPositionAbs::new(
			(self.x as f64 / CHUNK_WIDTH as f64).floor() as i64,
			(self.y as f64 / CHUNK_WIDTH as f64).floor() as i64,
		)
	}
}

#[derive(WorldPosition, Debug, Default, Clone, Copy)]
pub struct TilePositionRel {
	pub x: u8,
	pub y: u8,
}

impl TilePositionRel {
	pub fn new(
		x: u8,
		y: u8,
	) -> Self {
		Self { x, y }
	}

	/// Takes a tile relative to a src chunk and converts
	/// 	it to an absolute tile position in the world.
	/// 
	/// # Arguments
	/// 
	/// * `chunk_position_abs` - chunk absolute position
	/// 	that this tile is relative to.
	pub fn into_abs(
		self,
		chunk_position_abs: ChunkPositionAbs,
	) -> TilePositionAbs {
		TilePositionAbs::new(
			chunk_position_abs.x * CHUNK_WIDTH as i64 + self.x as i64,
			chunk_position_abs.y * CHUNK_WIDTH as i64 + self.y as i64,
		)
	}
}

#[derive(WorldPosition, Debug, Default, Clone, Copy)]
pub struct ChunkPositionAbs {
	pub x: i64,
	pub y: i64,
}

impl ChunkPositionAbs {
	pub const NULL: ChunkPositionAbs = ChunkPositionAbs { x: i64::MAX, y: i64::MAX };

	pub fn new(
		x: i64,
		y: i64,
	) -> Self {
		Self { x, y }
	}
}

#[derive(WorldPosition, Debug, Default, Clone, Copy)]
pub struct ChunkPositionRel {
	pub x: i64,
	pub y: i64,
}

impl ChunkPositionRel {
	pub fn new(
		x: i64,
		y: i64,
	) -> Self {
		Self { x, y }
	}

	/// Takes a chunk relative to another chunk and converts
	/// 	it to an absolute chunk position in the world.
	/// 
	/// # Arguments
	/// 
	/// * `chunk_position_abs` - chunk absolute position
	/// 	that this chunk is relative to.
	pub fn into_abs(
		self,
		chunk_position_abs: ChunkPositionAbs,
	) -> ChunkPositionAbs {
		ChunkPositionAbs::new(
			chunk_position_abs.x + self.x,
			chunk_position_abs.y + self.y,
		)
	}
}