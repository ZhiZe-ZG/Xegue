use crate::coord::Coord;
use crate::screen_symbol::ScreenSymbol;
use crate::room::Room;

pub fn new_map(rows: usize, cols: usize) -> Vec<Vec<char>> {
    vec![vec![' '; cols]; rows]
}

pub fn map_to_strings(map: &[Vec<char>]) -> Vec<String> {
    map.iter().map(|row| row.iter().collect()).collect()
}

pub fn draw_room(room: &Room, map: &mut [Vec<char>]) {
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

fn vert(room: &Room, map: &mut [Vec<char>], start_x: i32) {
    for y in (room.pos.y + 1)..=(room.pos.y + room.size.y - 1) {
        set_char(map, y, start_x, '|');
    }
}

fn horiz(room: &Room, map: &mut [Vec<char>], start_y: i32) {
    for x in room.pos.x..=(room.pos.x + room.size.x - 1) {
        set_char(map, start_y, x, '-');
    }
}

fn draw_maze(room: &Room, map: &mut [Vec<char>]) {
    for y in room.pos.y..(room.pos.y + room.size.y) {
        for x in room.pos.x..(room.pos.x + room.size.x) {
            set_char(map, y, x, ScreenSymbol::Passage.as_char());
        }
    }
}

fn set_char(map: &mut [Vec<char>], y: i32, x: i32, ch: char) {
    if y < 0 || x < 0 {
        return;
    }
    let (y, x) = (y as usize, x as usize);
    if let Some(row) = map.get_mut(y) {
        if let Some(cell) = row.get_mut(x) {
            *cell = ch;
        }
    }
}
