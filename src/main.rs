extern crate ncurses;

use glam::IVec2;
use ncurses::*;
use xegue::map::generate_map::generate_map;
use xegue::screen_symbol::ScreenSymbol;

fn manhattan(a: IVec2, b: IVec2) -> i32 {
    (a.x - b.x).abs() + (a.y - b.y).abs()
}

fn distance_squared(a: IVec2, b: IVec2) -> i32 {
    let dx = a.x - b.x;
    let dy = a.y - b.y;
    dx * dx + dy * dy
}

fn main() {
    // Init ncurses
    initscr(); // Initialize the window
    raw(); // Disable line buffering
    keypad(stdscr(), true); // Enable function keys
    noecho(); // Disable echoing of input characters

    // Test Coord struct
    let player_pos = IVec2::new(10, 5);
    let treasure_pos = IVec2::new(15, 8);

    // Calculate distances
    let manhattan = manhattan(player_pos, treasure_pos);
    let euclidean_sq = distance_squared(player_pos, treasure_pos);

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
    let delta = IVec2::new(2, -1);
    let new_pos = player_pos + delta;
    let _ = addstr(&format!(
        "Move by ({}, {}): new position ({}, {})\n\n",
        delta.x, delta.y, new_pos.x, new_pos.y
    ));

    // Test ncurses coordinate conversion
    let (y, x) = (player_pos.y, player_pos.x);
    let _ = addstr(&format!("For ncurses mvaddch: y={}, x={}\n\n", y, x));

    // Map generation test (no monsters, items, or maze)
    let map = generate_map(24, 80, 0);
    let _ = addstr("Map generation test:\n");
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
        IVec2::new(2, 3) == IVec2::new(2, 3)
    ));

    let _ = mvaddstr(LINES() - 1, 0, "Press any key to exit...");

    // Refresh the screen
    refresh();

    // Wait for user input
    getch();

    // End ncurses mode
    endwin();
}
