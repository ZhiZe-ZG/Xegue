/// Coordinate data type representing a position in the game world
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
}

impl Coord {
    /// Creates a new coordinate
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    
    /// Calculates Manhattan distance to another coordinate
    pub fn distance(&self, other: &Coord) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
    
    /// Calculates Euclidean distance squared (avoids sqrt for performance)
    pub fn distance_squared(&self, other: &Coord) -> i32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        dx * dx + dy * dy
    }
    
    /// Returns coordinate as (y, x) tuple (for ncurses compatibility)
    pub fn yx(&self) -> (i32, i32) {
        (self.y, self.x)
    }
}

impl std::ops::Add for Coord {
    type Output = Self;
    
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::Sub for Coord {
    type Output = Self;
    
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}