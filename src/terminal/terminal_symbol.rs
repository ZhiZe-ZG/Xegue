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
}
