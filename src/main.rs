#![allow(unused)]
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use crossterm::style::Color;
use crossterm::terminal::enable_raw_mode;
use crossterm::{
    cursor, execute,
    style::Print,
    terminal::{
        Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, ScrollUp, SetSize, size,
    },
};
use hecs::{BuiltEntity, Entity, EntityBuilder, World};
use std::io;

mod terminal;

use crate::terminal::{Pos, Renderable};

fn run_game() {
    let mut world = World::new();
    let mut builder = EntityBuilder::new();

    let player = world.spawn(
        builder
            .add(Pos { x: 1, y: 1 })
            .add(Renderable {
                glyph: '@',
                fg: Color::White,
                bg: Color::Black,
            })
            .build(),
    );

    'game_loop: loop {
        terminal::draw(terminal::test_map(), &mut world, &player);
        match event::read().unwrap() {
            Event::FocusGained => println!("FocusGained"),
            Event::FocusLost => println!("FocusLost"),
            Event::Key(e) => match e.code {
                KeyCode::Up => try_move(&mut world, &player, 0, -1),
                KeyCode::Down => try_move(&mut world, &player, 0, 1),
                KeyCode::Left => try_move(&mut world, &player, -1, 0),
                KeyCode::Right => try_move(&mut world, &player, 1, 0),
                KeyCode::Char('q') => break 'game_loop, // quit
                KeyCode::Esc => break 'game_loop,
                _ => (),
            },
            _ => (),
        }
    }
}

fn try_move(world: &mut World, entity: &Entity, x: i8, y: i8) {
    if let Ok(mut pos) = world.get_mut::<Pos>(*entity) {
        let target_pos = (pos.x + x as i16, pos.y + y as i16);
        match terminal::test_map()[terminal::xy_idx(target_pos.0, target_pos.1)] {
            terminal::TileType::Floor => {
                pos.x += x as i16;
                pos.y += y as i16;
            }
            terminal::TileType::Wall => (),
        }
    }
}

fn main() {
    enable_raw_mode().unwrap();
    // clear
    execute!(
        io::stdout(),
        EnterAlternateScreen,
        Clear(ClearType::All),
        cursor::Hide
    )
    .unwrap();

    run_game();

    execute!(io::stdout(), cursor::Show, LeaveAlternateScreen).unwrap();
}
