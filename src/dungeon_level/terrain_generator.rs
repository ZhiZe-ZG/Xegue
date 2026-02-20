use glam::IVec2;
use rand::Rng;

use crate::terrain::terrain_grid::TerrainGrid;
use crate::terrain_structure::room::put_room;
use crate::terrain_structure::room::Room;
use crate::terrain_structure::room_grid::RoomAdjacency; // new import

pub struct TerrainGenerator {
    pub terrain_size: IVec2,
    pub max_rooms: usize,
}

impl Default for TerrainGenerator {
    fn default() -> Self {
        TerrainGenerator {
            terrain_size: IVec2::new(80, 24),
            max_rooms: 9,
        }
    }
}

impl TerrainGenerator {
    pub fn generate_rooms(&self) -> Vec<Room> {
        let mut rooms = Vec::new();
        let mut rng = rand::rng();

        let bsze_x = self.terrain_size.x / 3;
        let bsze_y = self.terrain_size.y / 3;

        // up to 4 index for gone room (which is not exist on the terrain)
        let left_out = rng.random_range(0..=4);
        // generate a bool vec contains element of max_rooms
        // left_out of them are true, which means the room is gone
        let mut gone_rooms = vec![false; self.max_rooms];
        for i in 0..left_out {
            let idx = rng.random_range(0..self.max_rooms);
            gone_rooms[idx] = true;
        }

        for i in 0..self.max_rooms {
            // top left corner of each block
            let top_x = (i % 3) as i32 * bsze_x + 1;
            let top_y = (i / 3) as i32 * bsze_y;

            // pos is randomly set based on the top left corner of the block, and the size is randomly set based on the block size
            let pos_x = top_x + rng.random_range(0..=(bsze_x - 4).max(0));
            let pos_y = top_y + rng.random_range(0..=(bsze_y - 4).max(0));

            if gone_rooms[i] {
                // mark gone room
                // this room is lack from the terrain
                // the pos of gone room should be random in the block
                // this should used in put passages to connect the room with other rooms, and make sure the path is not too long
                rooms.push(Room {
                    pos: IVec2::new(pos_x, pos_y),
                    size: IVec2::new(0, 0),
                    is_maze: false,
                    is_dark: false,
                    is_gone: true,
                });
                continue;
            }

            // random set position and size of the room
            // size is randomly choose in the left places
            let size_x = rng.random_range(0..=(bsze_x - (pos_x - top_x) - 1).max(0)).max(4);
            let size_y = rng.random_range(0..=(bsze_y - (pos_y - top_y) - 1).max(0)).max(4);

            rooms.push(Room {
                pos: IVec2::new(pos_x, pos_y),
                size: IVec2::new(size_x, size_y),
                is_maze: false,
                is_dark: false,
                is_gone: false,
            });
        }

        rooms
    }

    /// Generate a random room adjacency matrix for this level.
    ///
    /// This only creates the abstract connection graph; it does not
    /// actually carve corridors on the terrain.
    pub fn generate_room_connections(&self) -> RoomAdjacency {
        let mut rng = rand::rng();
        RoomAdjacency::generate_random_graph(&mut rng)
    }

    /// Put given rooms onto the provided terrain grid.
    pub fn put_rooms_on_grid(&self, grid: &mut TerrainGrid, rooms: Vec<Room>) {
        for room in rooms {
            if room.is_gone || room.size.x <= 0 || room.size.y <= 0 {
                continue;
            }

            // Clamp room rectangle to terrain bounds
            let start_x = room.pos.x.max(0);
            let start_y = room.pos.y.max(0);
            let end_x = (room.pos.x + room.size.x).min(self.terrain_size.x);
            let end_y = (room.pos.y + room.size.y).min(self.terrain_size.y);

            if end_x <= start_x || end_y <= start_y {
                continue;
            }

            // Delegate to room helper if it expects full room info
            put_room(&room, grid);
        }
    }
}
