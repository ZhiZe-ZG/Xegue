extern crate ncurses;

use xegue::coord::Coord;
use xegue::map::map_grid::MapGrid;
use xegue::map::map_structure::{put_room, Room};
use xegue::screen_symbol::ScreenSymbol;
use ncurses::*;

fn main() {
    // Init ncurses
    initscr(); // Initialize the window
    raw(); // Disable line buffering
    keypad(stdscr(), true); // Enable function keys
    noecho(); // Disable echoing of input characters

    // Test Coord struct
    let player_pos = Coord::new(10, 5);
    let treasure_pos = Coord::new(15, 8);

    // Calculate distances
    let manhattan = player_pos.distance(&treasure_pos);
    let euclidean_sq = player_pos.distance_squared(&treasure_pos);

    // Print text
    let _ = addstr("=== xegue Coordinate System Test ===\n\n");
    let _ = addstr(&format!(
        "Player position: ({}, {})\n",
        player_pos.x, player_pos.y
    ));
    let _ = addstr(&format!(
        "Treasure position: ({}, {})\n\n",
        treasure_pos.x, treasure_pos.y
    ));
    let _ = addstr(&format!("Manhattan distance: {}\n", manhattan));
    let _ = addstr(&format!("Euclidean distance²: {}\n\n", euclidean_sq));

    // Test coordinate arithmetic
    let delta = Coord::new(2, -1);
    let new_pos = player_pos + delta;
    let _ = addstr(&format!(
        "Move by ({}, {}): new position ({}, {})\n\n",
        delta.x, delta.y, new_pos.x, new_pos.y
    ));

    // Test ncurses coordinate conversion
    let (y, x) = player_pos.yx();
    let _ = addstr(&format!("For ncurses mvaddch: y={}, x={}\n\n", y, x));

    // Room drawing test (Rust rewrite of draw_room/vert/horiz)
    let mut map = MapGrid::init(12, 40);
    let room = Room {
        pos: Coord::new(2, 2),
        size: Coord::new(16, 8),
        is_maze: false,
        is_dark:false,
        is_gone:false,
    };
    put_room(&room, &mut map);
    let _ = addstr("Room draw test:\n");
    for line in map.to_strings() {
        let _ = addstr(&line);
        let _ = addstr("\n");
    }
    let _ = addstr("\n");

    // Display marker at player position
    let _ = mvaddch(
        player_pos.y,
        player_pos.x,
        ScreenSymbol::Player.as_char() as u32,
    );
    let _ = mvaddstr(player_pos.y + 1, player_pos.x - 1, "YOU");

    // Display marker at treasure position
    let _ = mvaddch(
        treasure_pos.y,
        treasure_pos.x,
        ScreenSymbol::Gold.as_char() as u32,
    );
    let _ = mvaddstr(treasure_pos.y + 1, treasure_pos.x - 1, "GOLD");

    // Eq test
    let _ = addstr(&format!(
        "(2,3) is  equal to (2,3): {}\n",
        Coord::new(2, 3) == Coord::new(2, 3)
    ));

    let _ = mvaddstr(LINES() - 1, 0, "Press any key to exit...");

    // Refresh the screen
    refresh();

    // Wait for user input
    getch();

    // End ncurses mode
    endwin();
}
