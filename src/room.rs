use crate::coord::Coord;

#[derive(Copy, Clone, Debug)]
pub struct Room {
    pub pos: Coord,
    pub size: Coord,
    pub is_maze: bool,
}
