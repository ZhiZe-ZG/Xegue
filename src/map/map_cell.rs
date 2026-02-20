use crate::terminal::terminal_symbol::TerminalSymbol;
use crate::thing::Thing;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CellType {
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

impl CellType {
    pub fn to_screen_symbol(self) -> TerminalSymbol {
        match self {
            CellType::Empty => TerminalSymbol::Empty,
            CellType::Passage => TerminalSymbol::Passage,
            CellType::Door => TerminalSymbol::Door,
            CellType::Floor => TerminalSymbol::Floor,
            CellType::Player => TerminalSymbol::Player,
            CellType::Trap => TerminalSymbol::Trap,
            CellType::Stairs => TerminalSymbol::Stairs,
            CellType::Gold => TerminalSymbol::Gold,
            CellType::Potion => TerminalSymbol::Potion,
            CellType::Scroll => TerminalSymbol::Scroll,
            CellType::Magic => TerminalSymbol::Magic,
            CellType::Food => TerminalSymbol::Food,
            CellType::Weapon => TerminalSymbol::Weapon,
            CellType::Armor => TerminalSymbol::Armor,
            CellType::Amulet => TerminalSymbol::Amulet,
            CellType::Ring => TerminalSymbol::Ring,
            CellType::Stick => TerminalSymbol::Stick,
            CellType::WallHorizontal => TerminalSymbol::WallHorizontal,
            CellType::WallVertical => TerminalSymbol::WallVertical,
        }
    }
}

#[derive(Clone)]
pub struct MapCell {
    pub cell_type: CellType,
    pub flags: u8,
    pub monst: Option<Thing>,
}
