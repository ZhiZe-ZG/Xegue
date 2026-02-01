use crate::map::map_grid::MapGrid;
use crate::map::map_structure::Room;
use crate::screen_symbol::ScreenSymbol;

pub fn draw_room(room: &Room, map: &mut MapGrid) {
    if room.is_maze {
        draw_maze(room, map);
    } else {
        vert(room, map, room.pos.x);
        vert(room, map, room.pos.x + room.size.x - 1);
        horiz(room, map, room.pos.y);
        horiz(room, map, room.pos.y + room.size.y - 1);

        for y in (room.pos.y + 1)..(room.pos.y + room.size.y - 1) {
            for x in (room.pos.x + 1)..(room.pos.x + room.size.x - 1) {
                set_char(map, y, x, ScreenSymbol::Floor.as_char());
            }
        }
    }
}

fn vert(room: &Room, map: &mut MapGrid, start_x: i32) {
    for y in (room.pos.y + 1)..=(room.pos.y + room.size.y - 1) {
        set_char(map, y, start_x, '|');
    }
}

fn horiz(room: &Room, map: &mut MapGrid, start_y: i32) {
    for x in room.pos.x..=(room.pos.x + room.size.x - 1) {
        set_char(map, start_y, x, '-');
    }
}

fn draw_maze(room: &Room, map: &mut MapGrid) {
    for y in room.pos.y..(room.pos.y + room.size.y) {
        for x in room.pos.x..(room.pos.x + room.size.x) {
            set_char(map, y, x, ScreenSymbol::Passage.as_char());
        }
    }
}

fn set_char(map: &mut MapGrid, y: i32, x: i32, ch: char) {
    if y < 0 || x < 0 {
        return;
    }
    let (y, x) = (y as usize, x as usize);
    if let Some(cell) = map.get_mut(y, x) {
        cell.ch = ch;
    }
}
