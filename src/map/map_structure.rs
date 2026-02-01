use crate::map::map_cell::CellType;
use crate::map::map_grid::MapGrid;
use glam::IVec2;

#[derive(Copy, Clone, Debug)]
pub struct Room {
    pub pos: IVec2,
    pub size: IVec2,
    pub is_maze: bool,
    pub is_gone: bool,  /* room is gone (a corridor) */
    pub is_dark:bool, /* room is dark */
}

pub fn put_room(room: &Room, map: &mut MapGrid) {
    if room.is_maze {
        put_maze(room, map);
    } else {
        put_vert_wall(room, map, room.pos.x);
        put_vert_wall(room, map, room.pos.x + room.size.x - 1);
        put_horiz_wall(room, map, room.pos.y);
        put_horiz_wall(room, map, room.pos.y + room.size.y - 1);

        for y in (room.pos.y + 1)..(room.pos.y + room.size.y - 1) {
            for x in (room.pos.x + 1)..(room.pos.x + room.size.x - 1) {
                set_cell_type(map, y, x, CellType::Floor);
            }
        }
    }
}

fn put_vert_wall(room: &Room, map: &mut MapGrid, start_x: i32) {
    for y in (room.pos.y + 1)..=(room.pos.y + room.size.y - 1) {
        set_cell_type(map, y, start_x, CellType::WallVertical);
    }
}

fn put_horiz_wall(room: &Room, map: &mut MapGrid, start_y: i32) {
    for x in room.pos.x..=(room.pos.x + room.size.x - 1) {
        set_cell_type(map, start_y, x, CellType::WallHorizontal);
    }
}

fn put_maze(room: &Room, map: &mut MapGrid) {
    for y in room.pos.y..(room.pos.y + room.size.y) {
        for x in room.pos.x..(room.pos.x + room.size.x) {
            set_cell_type(map, y, x, CellType::Passage);
        }
    }
}

fn set_cell_type(map: &mut MapGrid, y: i32, x: i32, cell_type: CellType) {
    if y < 0 || x < 0 {
        return;
    }
    let (y, x) = (y as usize, x as usize);
    if let Some(cell) = map.get_mut(y, x) {
        cell.cell_type = cell_type;
    }
}
