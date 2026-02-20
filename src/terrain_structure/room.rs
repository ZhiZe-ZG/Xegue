use crate::terrain::terrain_cell::TerrainCellClass;
use crate::terrain::terrain_grid::TerrainGrid;
use glam::IVec2;

#[derive(Copy, Clone, Debug)]
pub struct Room {
    pub pos: IVec2, // let top corner of the room
    pub size: IVec2,
    pub is_maze: bool,
    pub is_gone: bool,  /* room is gone (a corridor) */
    pub is_dark:bool, /* room is dark */
}

//need rewrite and explain

pub fn put_room(room: &Room, map: &mut TerrainGrid) {
    if room.is_maze {
        put_maze(room, map);
    } else {
        put_vert_wall(room, map, room.pos.x);
        put_vert_wall(room, map, room.pos.x + room.size.x - 1);
        put_horiz_wall(room, map, room.pos.y);
        put_horiz_wall(room, map, room.pos.y + room.size.y - 1);

        for y in (room.pos.y + 1)..(room.pos.y + room.size.y - 1) {
            for x in (room.pos.x + 1)..(room.pos.x + room.size.x - 1) {
                set_cell_type(map, x, y, TerrainCellClass::Floor);
            }
        }
    }
}

fn put_vert_wall(room: &Room, map: &mut TerrainGrid, start_x: i32) {
    for y in (room.pos.y + 1)..=(room.pos.y + room.size.y - 1) {
        set_cell_type(map, start_x, y, TerrainCellClass::WallVertical);
    }
}

fn put_horiz_wall(room: &Room, map: &mut TerrainGrid, start_y: i32) {
    for x in room.pos.x..=(room.pos.x + room.size.x - 1) {
        set_cell_type(map, x, start_y, TerrainCellClass::WallHorizontal);
    }
}

fn put_maze(room: &Room, map: &mut TerrainGrid) {
    // This should rewrite
    for y in room.pos.y..(room.pos.y + room.size.y) {
        for x in room.pos.x..(room.pos.x + room.size.x) {
            set_cell_type(map, x, y, TerrainCellClass::Passage);
        }
    }
}

/// Set a cell's class at (x, y) if it is inside the grid.
fn set_cell_type(map: &mut TerrainGrid, x: i32, y: i32, cell_type: TerrainCellClass) {
    if x < 0 || y < 0 {
        return;
    }
    let (x, y) = (x as usize, y as usize);
    if let Some(cell) = map.get_mut(x, y) {
        cell.cell_class = cell_type;
    }
}
