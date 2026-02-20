use glam::IVec2;
use rand::Rng;

use crate::terrain::terrain_grid::TerrainGrid;
use crate::terrain_structure::room::put_room;
use crate::terrain_structure::room::Room;
use crate::terrain_structure::room_grid::RoomAdjacency;
use crate::terrain_structure::passage::Passage;

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
                    doors: Vec::new(),
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
                doors: Vec::new(),
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

    /// Carve doors and passages between rooms based on the given adjacency
    /// matrix. This only touches the terrain grid; the caller must have
    /// already placed rooms on the grid.
    ///
    /// Returns the list of carved passages.
    pub fn carve_passages(
        &self,
        grid: &mut TerrainGrid,
        rooms: &mut [Room],
        adjacency: &RoomAdjacency,
    ) -> Vec<Passage> {
        let room_count = rooms.len();
        let mut passages = Vec::new();

        // Iterate over upper triangle of adjacency matrix to avoid duplicates
        for i in 0..room_count {
            for j in (i + 1)..room_count {
                if adjacency.is_connected(i, j) {
                    let (r1, r2) = {
                        let (left, right) = rooms.split_at_mut(j);
                        (&mut left[i], &mut right[0])
                    };
                    if r1.is_gone || r2.is_gone {
                        continue;
                    }
                    if let Some(p) =
                        self.carve_corridor_between_rooms(grid, r1, r2, i, j)
                    {
                        passages.push(p);
                    }
                }
            }
        }

        passages
    }

    /// Carve a corridor between two rooms using a port of Rogue's `conn`
    /// algorithm: corridors are either horizontal or vertical primary
    /// with a single turn.
    fn carve_corridor_between_rooms(
        &self,
        grid: &mut TerrainGrid,
        r1: &mut Room,
        r2: &mut Room,
        idx1: usize,
        idx2: usize,
    ) -> Option<Passage> {
        // Map Rogue's 3x3 layout: decide which pair index is "rm",
        // and whether we are going right ('r') or down ('d').
        let (rm, direc) = if idx1 < idx2 {
            if idx1 + 1 == idx2 {
                (idx1, 'r')
            } else {
                (idx1, 'd')
            }
        } else {
            if idx2 + 1 == idx1 {
                (idx2, 'r')
            } else {
                (idx2, 'd')
            }
        };

        // Determine which Room is "from" and which is "to" following Rogue.
        let (from, to, from_idx, to_idx) = if rm == idx1 {
            (r1, r2, idx1, idx2)
        } else {
            (r2, r1, idx2, idx1)
        };

        // Early-out if any room has zero size (invalid).
        // NOTE: gone rooms have size (0,0), but should still be connectable
        // by passages; they simply won't get doors. So only reject truly
        // invalid (negative) sizes.
        if from.size.x < 0
            || from.size.y < 0
            || to.size.x < 0
            || to.size.y < 0
        {
            return None;
        }

        let mut rng = rand::rng();

        let mut del = IVec2::ZERO;
        let mut spos = from.pos;
        let mut epos = to.pos;
        let mut turn_delta = IVec2::ZERO;
        let mut distance: i32 = 0;
        let mut turn_distance: i32 = 0;

        if direc == 'd' {
            // Vertical adjacency: from is above to.
            del = IVec2::new(0, 1);

            // Compute horizontal overlap between the two rooms.
            let from_left = from.pos.x;
            let from_right = from.pos.x + from.size.x - 1;
            let to_left = to.pos.x;
            let to_right = to.pos.x + to.size.x - 1;

            let overlap_left = from_left.max(to_left);
            let overlap_right = from_right.min(to_right);

            // Use the middle of overlap as the door column if there is overlap.
            // Otherwise, fall back to original random columns, still aligned.
            let door_x = if overlap_left <= overlap_right {
                (overlap_left + overlap_right) / 2
            } else {
                // no overlap: pick random columns but still "aligned" by choice
                if from.size.x > 2 {
                    from.pos.x + rng.random_range(1..from.size.x - 1)
                } else {
                    from.pos.x
                }
            };

            // from door: bottom wall
            spos.x = door_x.clamp(from.pos.x + 1, from.pos.x + from.size.x - 2);
            spos.y = from.pos.y + from.size.y - 1;

            // to door: top wall, same x
            epos.x = door_x.clamp(to.pos.x + 1, to.pos.x + to.size.x - 2);
            epos.y = to.pos.y;

            distance = (spos.y - epos.y).abs() - 1;
            turn_delta = IVec2::new(if spos.x < epos.x { 1 } else { -1 }, 0);
            turn_distance = (spos.x - epos.x).abs();
        } else if direc == 'r' {
            // Horizontal adjacency: from is left of to.
            del = IVec2::new(1, 0);

            // Compute vertical overlap between the two rooms.
            let from_top = from.pos.y;
            let from_bottom = from.pos.y + from.size.y - 1;
            let to_top = to.pos.y;
            let to_bottom = to.pos.y + to.size.y - 1;

            let overlap_top = from_top.max(to_top);
            let overlap_bottom = from_bottom.min(to_bottom);

            let door_y = if overlap_top <= overlap_bottom {
                (overlap_top + overlap_bottom) / 2
            } else {
                if from.size.y > 2 {
                    from.pos.y + rng.random_range(1..from.size.y - 1)
                } else {
                    from.pos.y
                }
            };

            // from door: right wall
            spos.x = from.pos.x + from.size.x - 1;
            spos.y = door_y.clamp(from.pos.y + 1, from.pos.y + from.size.y - 2);

            // to door: left wall, same y
            epos.x = to.pos.x;
            epos.y = door_y.clamp(to.pos.y + 1, to.pos.y + to.size.y - 2);

            distance = (spos.x - epos.x).abs() - 1;
            turn_delta = IVec2::new(0, if spos.y < epos.y { 1 } else { -1 });
            turn_distance = (spos.y - epos.y).abs();
        } else {
            return None;
        }

        if distance < 0 {
            // Rooms overlap / are too close: still ensure a passage exists.
            // Use passage tiles, and only put doors for non-gone rooms.
            if from.is_gone {
                grid.set_passage(spos.x, spos.y);
            } else {
                grid.set_door(spos.x, spos.y);
                from.doors.push(spos);
            }

            if to.is_gone {
                grid.set_passage(epos.x, epos.y);
            } else {
                grid.set_door(epos.x, epos.y);
                to.doors.push(epos);
            }

            let tiles = vec![spos, epos];
            return Some(Passage::new(from_idx, to_idx, spos, epos, tiles));
        }

        // Rogue: turn_spot = rnd(distance - 1) + 1;
        // If distance <= 1, there is effectively no space to turn; just go straight.
        let turn_spot = if distance > 1 {
            rng.random_range(1..distance)
        } else {
            1
        };

        // Endpoints: gone rooms get passage tiles, normal rooms get doors.
        if from.is_gone {
            grid.set_passage(spos.x, spos.y);
        } else {
            grid.set_door(spos.x, spos.y);
            from.doors.push(spos);
        }

        if to.is_gone {
            grid.set_passage(epos.x, epos.y);
        } else {
            grid.set_door(epos.x, epos.y);
            to.doors.push(epos);
        }

        // Now carve the corridor between spos and epos.
        let mut tiles: Vec<IVec2> = Vec::new();
        tiles.push(spos);

        let mut curr = spos;
        let mut dist_left = distance;

        while dist_left > 0 {
            // Move one step along primary direction.
            curr += del;

            // If we are at the turn spot, walk all turn_distance along turn_delta.
            if dist_left == turn_spot && turn_distance > 0 {
                let mut td = turn_distance;
                while td > 0 {
                    grid.set_passage(curr.x, curr.y);
                    tiles.push(curr);
                    curr += turn_delta;
                    td -= 1;
                }
            }

            // Continue digging along primary direction.
            grid.set_passage(curr.x, curr.y);
            tiles.push(curr);
            dist_left -= 1;
        }

        // After the loop Rogue steps one more time and expects to arrive at epos.
        curr += del;
        if curr != epos {
            // If this happens, something is inconsistent; still force a straight fill.
            // But this should not in the typical 3x3 layout.
            let step_x = (epos.x - curr.x).signum();
            let step_y = (epos.y - curr.y).signum();
            while curr != epos {
                grid.set_passage(curr.x, curr.y);
                tiles.push(curr);
                curr.x += step_x;
                curr.y += step_y;
            }
        }

        // Ensure epos is represented (as a door or passage). It may already
        // be a door in the grid; we only record its position if missing.
        if tiles.last().copied() != Some(epos) {
            tiles.push(epos);
        }

        Some(Passage::new(
            from_idx,
            to_idx,
            spos,
            epos,
            tiles,
        ))
    }
}
