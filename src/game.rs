use crossterm::event::{self, Event, KeyCode};
use crossterm::style::Color;
use hecs::{Entity, EntityBuilder, World};

use crate::terminal::{self, Pos, Renderable};

pub fn run() {
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
                KeyCode::Up | KeyCode::Char('k') => try_move(&mut world, &player, 0, -1),
                KeyCode::Down | KeyCode::Char('j') => try_move(&mut world, &player, 0, 1),
                KeyCode::Left | KeyCode::Char('h') => try_move(&mut world, &player, -1, 0),
                KeyCode::Right | KeyCode::Char('l') => try_move(&mut world, &player, 1, 0),
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
