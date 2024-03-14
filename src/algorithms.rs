use bevy::prelude::Vec2;
use crate::TilePositionAbs;

pub fn spiral<F: FnMut(&TilePositionAbs) -> bool>(
	mut tile_position_abs: TilePositionAbs,
	mut f: F,
) {
	let mut tile_vector = TilePositionAbs::new(1, 0);
	let mut segment_length = 1;
	let mut segment_passed = 0;
	loop {
		if !f(&tile_position_abs) {
			return;
		}
		tile_position_abs.x += tile_vector.x;
		tile_position_abs.y += tile_vector.y;
		segment_passed += 1;
		if segment_passed != segment_length { continue; }
		segment_passed = 0;
		let oldvx = tile_vector.x;
		tile_vector.x = -tile_vector.y;
		tile_vector.y = oldvx;
		if tile_vector.y == 0 {
			segment_length += 1;
		}
	}
}

pub fn inside_circle(
	tile_position_abs: TilePositionAbs,
	center: TilePositionAbs,
	radius: i64,
) -> bool {
	let dx = center.x - tile_position_abs.x;
	let dy = center.y - tile_position_abs.y;
	let distance_squared = dx.pow(2) + dy.pow(2);
	4 * distance_squared <= (radius * 2 + 1).pow(2)
}

pub fn line_intersection(
	a0: Vec2,
	a1: Vec2,
	b0: Vec2,
	b1: Vec2,
) -> bool {
	let b = a1 - a0;
	let d = b1 - b0;
	let dot_d_perp = (b.x * d.y) - (b.y * d.x);
	if dot_d_perp == 0.0 {
		return false;
	}

	let c = b0 - a0;
	let t = ((c.x * d.y) - (c.y * d.x)) / dot_d_perp;
	if t < 0.0 || t > 1.0 {
		return false;
	}

	let u = ((c.x * b.y) - (c.y * b.x)) / dot_d_perp;
	if u < 0.0 || u > 1.0 {
		return false;
	}

	// let intersection = a0 + (t * b);

	true
}