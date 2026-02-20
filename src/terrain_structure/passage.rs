use glam::IVec2;

/// Represents a passage connecting two rooms by index in the room array.
#[derive(Debug, Clone)]
pub struct Passage {
    /// Index of the first connected room in the rooms slice.
    pub room_a: usize,
    /// Index of the second connected room in the rooms slice.
    pub room_b: usize,

    /// Door position in `room_a` (grid coordinates).
    pub door_a: IVec2,
    /// Door position in `room_b` (grid coordinates).
    pub door_b: IVec2,

    /// All tile positions that make up this passage (world/grid coordinates),
    /// including the doors.
    pub tiles: Vec<IVec2>,
}

impl Passage {
    pub fn new(
        room_a: usize,
        room_b: usize,
        door_a: IVec2,
        door_b: IVec2,
        tiles: Vec<IVec2>,
    ) -> Self {
        Self {
            room_a,
            room_b,
            door_a,
            door_b,
            tiles,
        }
    }
}