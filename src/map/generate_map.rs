use glam::IVec2;

use crate::map::map_grid::MapGrid;
use crate::map::map_structure::{put_room, Room};

pub fn generate_map(length: usize, width: usize, height: usize) -> MapGrid {
	let mut map = MapGrid::init(length, width, height);
	if length < 3 || width < 3 {
		return map;
	}

	let size = IVec2::new(
		(width as i32).saturating_sub(2),
		(length as i32).saturating_sub(2),
	);
	let room = Room {
		pos: IVec2::new(1, 1),
		size,
		is_maze: false,
		is_dark: false,
		is_gone: false,
	};

	put_room(&room, &mut map);
	map
}
