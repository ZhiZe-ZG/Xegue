use crate::terrain::terrain_cell::{TerrainCell, TerrainCellClass};

pub struct TerrainGrid {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<TerrainCell>,
}

impl TerrainGrid {
    pub fn init(width: usize, height: usize) -> Self {
        let total = width.saturating_mul(height);
        Self {
            width,
            height,
            cells: vec![TerrainCell::default(); total],
        }
    }

    #[inline]
    pub fn index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&TerrainCell> {
        self.cells.get(self.index(x, y))
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut TerrainCell> {
        let index = self.index(x, y);
        self.cells.get_mut(index)
    }

    /// Mark a cell as a corridor / passage.
    pub fn set_passage(&mut self, x: i32, y: i32) {
        if x < 0 || y < 0 {
            return;
        }
        let (ux, uy) = (x as usize, y as usize);
        if ux >= self.width || uy >= self.height {
            return;
        }
        if let Some(cell) = self.get_mut(ux, uy) {
            cell.cell_class = TerrainCellClass::Passage;
        }
    }

    /// Mark a cell as a door.
    pub fn set_door(&mut self, x: i32, y: i32) {
        if x < 0 || y < 0 {
            return;
        }
        let (ux, uy) = (x as usize, y as usize);
        if ux >= self.width || uy >= self.height {
            return;
        }
        if let Some(cell) = self.get_mut(ux, uy) {
            cell.cell_class = TerrainCellClass::Door;
        }
    }
}