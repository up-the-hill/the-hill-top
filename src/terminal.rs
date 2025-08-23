#![allow(unused)]

use crossterm::{
    cursor, execute,
    style::{Color, Print},
    terminal::{
        Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, ScrollUp, SetSize, size,
    },
};
use hecs::{Entity, World};
use std::io::{self, Write};

const HEIGHT: usize = 40;
const WIDTH: usize = 80;
const SIZE: usize = WIDTH * HEIGHT;

pub struct Pos {
    pub x: i16,
    pub y: i16,
}

pub struct Renderable {
    pub glyph: char,
    pub fg: Color,
    pub bg: Color,
}

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall,
    Floor,
}

pub fn xy_idx(x: i16, y: i16) -> usize {
    (y as usize * 80) + x as usize
}

pub fn idx_xy(idx: usize) -> (i16, i16) {
    let y = (idx / 80) as i16;
    let x = (idx % 80) as i16;
    (x, y)
}

fn read_map() -> Vec<TileType> {
    let mut map = vec![TileType::Floor; 80 * 40];
    // TODO:
    map
}

pub fn test_map() -> Vec<TileType> {
    let mut map = vec![TileType::Floor; 80 * 40];

    // Make the boundaries walls
    for x in 0..80 {
        map[xy_idx(x, 0)] = TileType::Wall;
        map[xy_idx(x, 39)] = TileType::Wall;
    }
    for y in 0..40 {
        map[xy_idx(0, y)] = TileType::Wall;
        map[xy_idx(79, y)] = TileType::Wall;
    }

    // TODO: read a map

    map
}

pub fn draw(tilemap: Vec<TileType>, world: &mut World, player: &Entity) -> io::Result<()> {
    let mut stdout = io::stdout();
    let mut buffer: Vec<char> = Vec::with_capacity(SIZE);

    // clear
    execute!(
        stdout,
        EnterAlternateScreen,
        Clear(ClearType::All),
        cursor::Hide
    )?;

    // draw map
    for tile in tilemap {
        let ch = match (tile) {
            TileType::Wall => '#',
            TileType::Floor => '.',
        };
        buffer.push(ch);
    }

    // draw player
    if let Ok(mut pos) = world.get_mut::<Pos>(*player) {
        buffer[xy_idx(pos.x, pos.y)] = '@';
    } else {
        panic!("cannot find player")
    }

    // draw monsters

    for row in 0..HEIGHT {
        let start = row * WIDTH;
        let end = start + WIDTH;
        // Collect row chars into a String
        let row_s: String = buffer[start..end].iter().collect();

        execute!(stdout, cursor::MoveTo(0, row as u16), Print(&row_s))?;
    }

    // Ensure everything is flushed
    stdout.flush()?;

    Ok(())
}
