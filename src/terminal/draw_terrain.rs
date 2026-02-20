use glam::IVec2;
use crate::terrain::terrain_grid::TerrainGrid;
use crate::terminal::terminal_symbol::TerminalSymbol;

/// Convert a TerrainGrid into lines of text for ncurses to display.
pub fn terrain_to_lines(grid: &TerrainGrid) -> Vec<String> {
    let width = grid.width;
    let height = grid.height;

    let mut lines = Vec::with_capacity(height as usize);

    for y in 0..height {
        let mut line = String::with_capacity(width as usize);
        for x in 0..width {
            let cell = grid.get(x, y) .unwrap_or_else(|| panic!("Invalid grid access at ({}, {})", x, y));
            // use terminal_symbol.rs to get the display symbol
            let symbol = TerminalSymbol::char_from_terrain_cell_class(cell.cell_class);
            line.push(symbol);
        }
        lines.push(line);
    }

    lines
}
