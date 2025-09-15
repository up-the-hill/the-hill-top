#![allow(unused)]

use crate::game::{Pos, TileType, xy_idx};
use crossterm::{
    cursor, execute,
    style::Print,
    terminal::{
        Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, ScrollUp, SetSize, size,
    },
};
use hecs::{Entity, World};
use std::io::{self, Write};

pub const HEIGHT: usize = 40;
pub const WIDTH: usize = 80;
pub const SIZE: usize = WIDTH * HEIGHT;

pub fn draw_map(world: &mut World, player: &Entity) -> io::Result<()> {
    let mut stdout = io::stdout();
    let mut buffer: Vec<char> = Vec::with_capacity(SIZE);

    let mut map_query = world.query::<(&Vec<TileType>,)>();
    let (map,) = map_query.iter().next().unwrap().1;

    // add map
    for tile in map.iter() {
        let ch = match (tile) {
            TileType::Wall => '#',
            TileType::Floor => '.',
        };
        buffer.push(ch);
    }

    // add player
    if let Ok(mut pos) = world.get_mut::<Pos>(*player) {
        buffer[xy_idx(pos.x, pos.y)] = '@';
    } else {
        panic!("cannot find player")
    }

    // add monsters

    // draw map on screen
    for row in 0..HEIGHT {
        let start = row * WIDTH;
        let end = start + WIDTH;
        // Collect row chars into a String
        let row_s: String = buffer[start..end].iter().collect();

        execute!(stdout, cursor::MoveTo(0, row as u16), Print(&row_s))?;
    }

    stdout.flush()?;

    Ok(())
}

pub fn draw_box(x1: i16, y1: i16, x2: i16, y2: i16) -> io::Result<()> {
    let mut stdout = io::stdout();

    for y in y1..y2 {
        execute!(stdout, cursor::MoveTo(x1 as u16, y as u16), Print("│"))?;
        execute!(stdout, cursor::MoveTo(x2 as u16, y as u16), Print("│"))?;
    }

    for x in x1..x2 {
        execute!(stdout, cursor::MoveTo(x as u16, y1 as u16), Print("─"))?;
        execute!(stdout, cursor::MoveTo(x as u16, y2 as u16), Print("─"))?;
    }

    execute!(stdout, cursor::MoveTo(x1 as u16, y1 as u16), Print("┌"))?;
    execute!(stdout, cursor::MoveTo(x2 as u16, y1 as u16), Print("┐"))?;
    execute!(stdout, cursor::MoveTo(x1 as u16, y2 as u16), Print("└"))?;
    execute!(stdout, cursor::MoveTo(x2 as u16, y2 as u16), Print("┘"))?;

    Ok(())
}

pub fn draw_ui() -> io::Result<()> {
    draw_box(0, 40, 79, 45)?;

    Ok(())
}
