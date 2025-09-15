use crate::terminal;
use crossterm::event::{self, Event, KeyCode};
use crossterm::style::Color;
use hecs::{Entity, EntityBuilder, World};

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

pub fn read_map(path: &str) -> Vec<TileType> {
    let file_content = std::fs::read_to_string(path).expect("Could not read map file");
    let mut map = Vec::new();
    for c in file_content.chars() {
        match c {
            '#' => map.push(TileType::Wall),
            '.' => map.push(TileType::Floor),
            _ => (),
        }
    }
    map
}

pub fn run() {
    let mut world = World::new();
    let mut builder = EntityBuilder::new();
    let map = read_map("assets/home.txt");
    world.spawn((map,));

    let player = world.spawn(
        builder
            .add(Pos { x: 40, y: 20 })
            .add(Renderable {
                glyph: '@',
                fg: Color::White,
                bg: Color::Black,
            })
            .build(),
    );

    'game_loop: loop {
        terminal::draw_map(&mut world, &player);
        terminal::draw_ui();
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
    let mut map_query = world.query::<(&Vec<TileType>,)>();
    let (map,) = map_query.iter().next().unwrap().1;
    if let Ok(mut pos) = world.get_mut::<Pos>(*entity) {
        let target_pos = (pos.x + x as i16, pos.y + y as i16);
        if (target_pos.0 >= 0
            && target_pos.0 < terminal::WIDTH as i16
            && target_pos.1 >= 0
            && target_pos.1 < terminal::HEIGHT as i16)
        {
            match map[xy_idx(target_pos.0, target_pos.1)] {
                TileType::Floor => {
                    pos.x += x as i16;
                    pos.y += y as i16;
                }
                TileType::Wall => (),
            }
        }
    }
}
