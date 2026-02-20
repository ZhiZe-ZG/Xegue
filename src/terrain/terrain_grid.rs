use crate::terrain::terrain_cell::{TerrainCell, TerrainCellClass};

pub struct TerrainGrid {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<TerrainCell>,
}