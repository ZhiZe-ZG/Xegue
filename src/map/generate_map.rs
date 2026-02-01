use glam::IVec2;

use crate::map::map_grid::MapGrid;
use crate::map::map_structure::{put_room, Room};

pub fn generate_map(length: usize, width: usize, _height: usize) -> MapGrid {
	let mut map = MapGrid::init(length, width);
	if length < 3 || width < 3 {
		return map;
	}

	let length_i = length as i32;
	let width_i = width as i32;
	let bsze_x = width_i / 3;
	let bsze_y = length_i / 3;

	for i in 0..9 {
		let top_x = (i % 3) as i32 * bsze_x;
		let top_y = (i / 3) as i32 * bsze_y;
		let mut pos_x = top_x + 1;
		let mut pos_y = top_y + 1;

		if pos_x < 1 {
			pos_x = 1;
		}
		if pos_y < 1 {
			pos_y = 1;
		}

		let mut size_x = (bsze_x - 1).max(0);
		let mut size_y = (bsze_y - 1).max(0);
		let max_x = (width_i - 1 - pos_x).max(0);
		let max_y = (length_i - 1 - pos_y).max(0);
		if size_x > max_x {
			size_x = max_x;
		}
		if size_y > max_y {
			size_y = max_y;
		}

		if size_x < 3 || size_y < 3 {
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
