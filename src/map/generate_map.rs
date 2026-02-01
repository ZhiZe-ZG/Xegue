use glam::IVec2;
use rand::Rng;

use crate::map::map_grid::MapGrid;
use crate::map::map_structure::{put_room, Room};

pub fn generate_map(length: usize, width: usize, _height: usize) -> MapGrid {
	let mut map = MapGrid::init(length, width);
	if length < 3 || width < 3 {
		return map;
	}
	let mut rng = rand::rng();

	let length_i = length as i32;
	let width_i = width as i32;
	let bsze_x = width_i / 3;
	let bsze_y = length_i / 3;

	for i in 0..9 {
		let top_x = (i % 3) as i32 * bsze_x + 1;
		let top_y = (i / 3) as i32 * bsze_y;

		let max_room_x = (bsze_x - 4).max(0);
		let max_room_y = (bsze_y - 4).max(0);
		if max_room_x == 0 || max_room_y == 0 {
			continue;
		}

		let size_x = rng.random_range(0..=max_room_x) + 4;
		let size_y = rng.random_range(0..=max_room_y) + 4;
		let max_pos_x = (bsze_x - size_x).max(0);
		let max_pos_y = (bsze_y - size_y).max(0);
		let pos_x = top_x + rng.random_range(0..=max_pos_x);
		let pos_y = top_y + rng.random_range(0..=max_pos_y);

		if pos_y <= 0 || pos_y >= length_i - 1 {
			continue;
		}

		let room = Room {
			pos: IVec2::new(pos_x, pos_y),
			size: IVec2::new(size_x, size_y),
			is_maze: false,
			is_dark: false,
			is_gone: false,
		};

		put_room(&room, &mut map);
	}

	map
}
