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
}