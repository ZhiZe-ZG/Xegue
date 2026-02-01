use crate::map::map_cell::MapCell;

pub struct MapGrid {
	pub rows: usize,
	pub cols: usize,
	pub cells: Vec<MapCell>,
}

impl MapGrid {
	pub fn init(rows: usize, cols: usize) -> Self {
		let total = rows.saturating_mul(cols);
		Self {
			rows,
			cols,
			cells: vec![
				MapCell {
					ch: ' ',
					flags: 0,
					monst: None,
				};
				total
			],
		}
	}

	pub fn to_strings(&self) -> Vec<String> {
		let mut lines = Vec::with_capacity(self.rows);
		for row in 0..self.rows {
			let mut line = String::with_capacity(self.cols);
			for col in 0..self.cols {
				if let Some(cell) = self.get(row, col) {
					line.push(cell.ch);
				}
			}
			lines.push(line);
		}
		lines
	}

	pub fn index(&self, row: usize, col: usize) -> usize {
		row * self.cols + col
	}

	pub fn get(&self, row: usize, col: usize) -> Option<&MapCell> {
		self.cells.get(self.index(row, col))
	}

	pub fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut MapCell> {
		let index = self.index(row, col);
		self.cells.get_mut(index)
	}
}
