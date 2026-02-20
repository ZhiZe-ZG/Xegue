use crate::terrain::terrain_cell::TerrainCellClass;
use crate::terrain::terrain_grid::TerrainGrid;
use glam::IVec2;

#[derive(Clone, Debug)]
pub struct Room {
    pub pos: IVec2, // let top corner of the room
    pub size: IVec2,
    pub is_maze: bool,
    pub is_gone: bool,  /* room is gone (a corridor) */
    pub is_dark:bool, /* room is dark */
    pub doors: Vec<IVec2>,
}

pub fn put_room(room: &Room, map: &mut TerrainGrid) {
    if room.is_maze {
        put_maze(room, map);
    } else {
        put_room_walls(room, map);
        put_room_floor(room, map);
    }
}

fn put_vertical_wall(start_y:i32, end_y:i32, x:i32, map:&mut TerrainGrid){
    for y in start_y..=end_y{
        set_cell_type(map, x, y, TerrainCellClass::WallVertical);
    }
}

fn put_horizontal_wall(start_x:i32, end_x:i32, y:i32, map:&mut TerrainGrid){
    for x in start_x..=end_x{
        set_cell_type(map, x, y, TerrainCellClass::WallHorizontal);
    }
}

fn put_room_walls(room: &Room, map: &mut TerrainGrid) {
    put_vertical_wall(room.pos.y+1, room.pos.y + room.size.y - 2, room.pos.x, map);
    put_vertical_wall(room.pos.y+1, room.pos.y + room.size.y - 2, room.pos.x + room.size.x - 1, map);
    put_horizontal_wall(room.pos.x, room.pos.x + room.size.x - 1, room.pos.y, map);
    put_horizontal_wall(room.pos.x, room.pos.x + room.size.x - 1, room.pos.y + room.size.y - 1, map);
}

fn put_room_floor(room: &Room, map: &mut TerrainGrid) {
    for y in (room.pos.y + 1)..(room.pos.y + room.size.y - 1) {
        for x in (room.pos.x + 1)..(room.pos.x + room.size.x - 1) {
            set_cell_type(map, x, y, TerrainCellClass::Floor);
        }
    }
}

/// Set a cell's class at (x, y) if it is inside the grid.
fn set_cell_type(map: &mut TerrainGrid, x: i32, y: i32, cell_class: TerrainCellClass) {
    if x < 0 || y < 0 {
        return;
    }
    let (x, y) = (x as usize, y as usize);
    if let Some(cell) = map.get_mut(x, y) {
        cell.cell_class = cell_class; // <- fix: use `cell_class` field
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
