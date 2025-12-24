use aoc_2025::*;
use core::convert::From;
use std::usize;
extern crate pretty_env_logger;
#[macro_use]
extern crate log;
// #[allow(unused)]

fn main() {
    pretty_env_logger::init();
    info!("Starting {}", file!());
    let args = std::env::args().collect::<Vec<String>>();
    let test_mode = args.len() > 1 && args[1] == "test";
    let src_name = file!();

    let input_path = gen_input_path(src_name, test_mode);
    let lines = read_ascii(input_path).expect("File could not be opened.");
    let starting_map = Map2D::<Tile>::new_from(lines);
    debug!("Starting_map:\n{starting_map}");
    let (mut movable_map, p1_movable_count) = calc_movable(starting_map);
    debug!("Map after iteration 1\n{movable_map}");
    info!("Part 1: movable rolls: {}", p1_movable_count);
    let mut movable_sum: usize = p1_movable_count;
    let mut counter = 1;
    loop {
        let (new_movable_map, p2_movable_count) = calc_movable(movable_map);
        movable_map = new_movable_map;
        counter += 1;
        debug!("Map after iteration {counter}\n{movable_map}");
        debug!("New movable rolls: {}", p2_movable_count);
        if p2_movable_count == movable_sum {
            break;
        } else {
            movable_sum = p2_movable_count;
        }
    }
    info!("Part 2: Total movable rolls: {movable_sum}");
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum Tile {
    Empty,
    Roll(bool),
}

impl From<Tile> for u8 {
    fn from(t: Tile) -> u8 {
        match t {
            Tile::Empty => b'.',
            Tile::Roll(false) => b'@',
            Tile::Roll(true) => b'X',
        }
    }
}

impl From<u8> for Tile {
    fn from(b: u8) -> Self {
        match b {
            b'@' => Tile::Roll(false),
            b'X' => Tile::Roll(true),
            _ => Tile::Empty,
        }
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let b: u8 = (*self).into();
        write!(f, "{}", b as char)
    }
}

fn calc_movable(starting_map: Map2D<Tile>) -> (Map2D<Tile>, usize) {
    let mut movable_map = starting_map.clone();
    let mut count_map = Map2D::<u8>::from_elem_with_bound(starting_map.outer_bound, 0);
    for (tile, point) in starting_map.iter_elem() {
        if tile != Tile::Empty {
            let surrounding_roll_count = Direction::iterator()
                .filter(|dir| {
                    (dir.as_offset() + point).is_in_bounds(starting_map.outer_bound)
                        && starting_map.get(point + dir.as_offset()).unwrap() == Tile::Roll(false)
                })
                .count();
            count_map
                .set(point, surrounding_roll_count as u8)
                .expect("Should not happen, iterator delivered point.");
            if surrounding_roll_count < 4 {
                movable_map
                    .set(point, Tile::Roll(true))
                    .expect("Should not happen, iterator delivered point.");
            }
        }
    }
    let movable_count = movable_map
        .iter_elem()
        .filter(|(tile, _point)| *tile == Tile::Roll(true))
        .count();
    trace!("Count map:\n{count_map}");
    (movable_map, movable_count)
}
