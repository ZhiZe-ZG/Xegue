use crate::screen_symbol::ScreenSymbol;
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
    pub fn to_screen_symbol(self) -> ScreenSymbol {
        match self {
            CellType::Empty => ScreenSymbol::Empty,
            CellType::Passage => ScreenSymbol::Passage,
            CellType::Door => ScreenSymbol::Door,
            CellType::Floor => ScreenSymbol::Floor,
            CellType::Player => ScreenSymbol::Player,
            CellType::Trap => ScreenSymbol::Trap,
            CellType::Stairs => ScreenSymbol::Stairs,
            CellType::Gold => ScreenSymbol::Gold,
            CellType::Potion => ScreenSymbol::Potion,
            CellType::Scroll => ScreenSymbol::Scroll,
            CellType::Magic => ScreenSymbol::Magic,
            CellType::Food => ScreenSymbol::Food,
            CellType::Weapon => ScreenSymbol::Weapon,
            CellType::Armor => ScreenSymbol::Armor,
            CellType::Amulet => ScreenSymbol::Amulet,
            CellType::Ring => ScreenSymbol::Ring,
            CellType::Stick => ScreenSymbol::Stick,
            CellType::WallHorizontal => ScreenSymbol::WallHorizontal,
            CellType::WallVertical => ScreenSymbol::WallVertical,
        }
    }
}

#[derive(Clone)]
pub struct MapCell {
    pub cell_type: CellType,
    pub flags: u8,
    pub monst: Option<Thing>,
}
