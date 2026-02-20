use crate::thing::Thing;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TerrainCellClass {
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

#[derive(Clone)]
pub struct TerrainCell {
    pub cell_class: TerrainCellClass,
    pub flags: u8,
    pub monst: Option<Thing>,  // This should reconsider.
}

impl Default for TerrainCell {
    fn default() -> Self {
        Self {
            cell_class: TerrainCellClass::Empty,
            flags: 0,
            monst: None,
        }
    }
}

// Basic initialization helpers for TerrainCell.
impl TerrainCell {
    /// Create a new terrain cell with given class, flags and monster.
    pub fn new(cell_class: TerrainCellClass, flags: u8, monst: Option<Thing>) -> Self {
        Self {
            cell_class,
            flags,
            monst,
        }
    }
}
