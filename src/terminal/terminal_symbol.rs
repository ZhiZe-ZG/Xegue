use crate::terrain::terrain_cell::TerrainCellClass;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum TerminalSymbol {
    Empty,
    Passage,
    Door,
    Floor,
    Player,
    Trap,
    Stairs,
    Gold,
    Potion,
    Scroll,
    Magic,
    Food,
    Weapon,
    Armor,
    Amulet,
    Ring,
    Stick,
    WallHorizontal,
    WallVertical,
}

impl TerminalSymbol {
    pub const fn as_char(self) -> char {
        match self {
            TerminalSymbol::Empty => ' ',
            TerminalSymbol::Passage => '#',
            TerminalSymbol::Door => '+',
            TerminalSymbol::Floor => '.',
            TerminalSymbol::Player => '@',
            TerminalSymbol::Trap => '^',
            TerminalSymbol::Stairs => '%',
            TerminalSymbol::Gold => '*',
            TerminalSymbol::Potion => '!',
            TerminalSymbol::Scroll => '?',
            TerminalSymbol::Magic => '$',
            TerminalSymbol::Food => ':',
            TerminalSymbol::Weapon => ')',
            TerminalSymbol::Armor => ']',
            TerminalSymbol::Amulet => ',',
            TerminalSymbol::Ring => '=',
            TerminalSymbol::Stick => '/',
            TerminalSymbol::WallHorizontal => '-',
            TerminalSymbol::WallVertical => '|',
        }
    }

    /// Transfer a terrain cell class to a terminal symbol.
    pub const fn from_terrain_cell_class(class: TerrainCellClass) -> Self {
        match class {
            TerrainCellClass::Empty => TerminalSymbol::Empty,
            TerrainCellClass::Passage => TerminalSymbol::Passage,
            TerrainCellClass::Door => TerminalSymbol::Door,
            TerrainCellClass::Floor => TerminalSymbol::Floor,
            TerrainCellClass::Player => TerminalSymbol::Player,
            TerrainCellClass::Trap => TerminalSymbol::Trap,
            TerrainCellClass::Stairs => TerminalSymbol::Stairs,
            TerrainCellClass::Gold => TerminalSymbol::Gold,
            TerrainCellClass::Potion => TerminalSymbol::Potion,
            TerrainCellClass::Scroll => TerminalSymbol::Scroll,
            TerrainCellClass::Magic => TerminalSymbol::Magic,
            TerrainCellClass::Food => TerminalSymbol::Food,
            TerrainCellClass::Weapon => TerminalSymbol::Weapon,
            TerrainCellClass::Armor => TerminalSymbol::Armor,
            TerrainCellClass::Amulet => TerminalSymbol::Amulet,
            TerrainCellClass::Ring => TerminalSymbol::Ring,
            TerrainCellClass::Stick => TerminalSymbol::Stick,
            TerrainCellClass::WallHorizontal => TerminalSymbol::WallHorizontal,
            TerrainCellClass::WallVertical => TerminalSymbol::WallVertical,
            // add mappings here if you introduce new terrain kinds
        }
    }

    /// Accept a terrain cell class and return the corresponding display character.
    pub const fn char_from_terrain_cell_class(class: TerrainCellClass) -> char {
        Self::from_terrain_cell_class(class).as_char()
    }
}
