use crate::map::map_cell::{CellType, MapCell};

pub struct MapGrid {
	pub length: usize,
	pub width: usize,
	pub cells: Vec<MapCell>,
}

impl MapGrid {
	pub fn init(length: usize, width: usize) -> Self {
		let total = length.saturating_mul(width);
		Self {
			length,
			width,
			cells: vec![
				MapCell {
					cell_type: CellType::Empty,
					flags: 0,
					monst: None,
				};
				total
			],
		}
	}

	pub fn to_strings(&self) -> Vec<String> {
		let mut lines = Vec::with_capacity(self.length);
		for y in 0..self.length {
			let mut line = String::with_capacity(self.width);
			for x in 0..self.width {
				if let Some(cell) = self.get(y, x) {
					line.push(cell.cell_type.to_screen_symbol().as_char());
				}
			}
			lines.push(line);
		}
		lines
	}

	pub fn index(&self, y: usize, x: usize) -> usize {
		y * self.width + x
	}

	pub fn get(&self, y: usize, x: usize) -> Option<&MapCell> {
		self.cells.get(self.index(y, x))
	}

	pub fn get_mut(&mut self, y: usize, x: usize) -> Option<&mut MapCell> {
		let index = self.index(y, x);
		self.cells.get_mut(index)
	}
}
