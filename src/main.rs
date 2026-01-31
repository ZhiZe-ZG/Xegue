extern crate ncurses;

use ncurses::*;

fn main() {
    // Init ncurses
    initscr();              // Initialize the window
    raw();                  // Disable line buffering
    keypad(stdscr(), true); // Enable function keys
    noecho();               // Disable echoing of input characters

    // Print text
    let _ = addstr("Hello, Xegue!\n");
    let _ = addstr("Press any key to exit...");

    // Refresh the screen
    refresh();

    // Wait for user input
    getch();

    // End ncurses mode
    endwin();
}
