#![allow(unused)]
use crossterm::{
    cursor, execute,
    terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;

mod game;
mod terminal;

fn main() {
    crossterm::terminal::enable_raw_mode().unwrap();
    // clear
    execute!(
        io::stdout(),
        EnterAlternateScreen,
        Clear(ClearType::All),
        cursor::Hide
    )
    .unwrap();

    game::run();

    execute!(io::stdout(), cursor::Show, LeaveAlternateScreen).unwrap();
}
