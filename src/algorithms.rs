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