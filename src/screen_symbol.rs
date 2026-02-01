#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum ScreenSymbol {
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

impl ScreenSymbol {
    pub const fn as_char(self) -> char {
        match self {
            ScreenSymbol::Empty => ' ',
            ScreenSymbol::Passage => '#',
            ScreenSymbol::Door => '+',
            ScreenSymbol::Floor => '.',
            ScreenSymbol::Player => '@',
            ScreenSymbol::Trap => '^',
            ScreenSymbol::Stairs => '%',
            ScreenSymbol::Gold => '*',
            ScreenSymbol::Potion => '!',
            ScreenSymbol::Scroll => '?',
            ScreenSymbol::Magic => '$',
            ScreenSymbol::Food => ':',
            ScreenSymbol::Weapon => ')',
            ScreenSymbol::Armor => ']',
            ScreenSymbol::Amulet => ',',
            ScreenSymbol::Ring => '=',
            ScreenSymbol::Stick => '/',
            ScreenSymbol::WallHorizontal => '-',
            ScreenSymbol::WallVertical => '|',
        }
    }
}
