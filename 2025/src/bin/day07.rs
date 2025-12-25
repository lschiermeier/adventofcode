use aoc_2025::*;
#[macro_use]
extern crate log;
extern crate pretty_env_logger;

use Tile::*;

fn main() {
    pretty_env_logger::init();
    info!("Starting {}", file!());
    let args = std::env::args().collect::<Vec<String>>();
    let test_mode = args.len() > 1 && args[1] == "test";
    let src_name = file!();

    let input_path = gen_input_path(src_name, test_mode);
    let starting_map: Map2D<Tile> =
        Map2D::new_from(read_ascii(&input_path).expect("File could not be opened."))
            .expect("Map could not be built.");
    debug!("\n{starting_map}");
    let mut beam_map = starting_map.clone();
    let mut split_count = 0;
    for (starting_tile, point) in starting_map
        .iter_elem()
        .skip(starting_map.outer_bound.x as usize)
    {
        let above = beam_map
            .get(point + Direction::North.as_offset())
            .expect("North should exist");
        match (starting_tile, above) {
            // simple do nothing cases
            (_, Splitter) | (Splitter, Empty) | (Empty, Empty) => continue,

            (Start, _) | (Beam(_), _) => {
                panic!("Start should not be below. Starting map should be empty of beams.")
            }

            // first beam no decisions
            (_, Start) => beam_map
                .set(point, Beam(1))
                .expect("Below Start should exist."),

            (Empty, Beam(above_num)) => {
                if let Some(Beam(current_num)) = beam_map.get(point) {
                    beam_map
                        .set(point, Beam(current_num + above_num))
                        .expect("The point was just read...")
                } else {
                    beam_map
                        .set(point, Beam(above_num))
                        .expect("The point was just read...")
                }
            }
            (Splitter, Beam(above_num)) => {
                split_count += 1;
                let blubb = beam_map.get(point + Direction::West.as_offset());
                trace!("Beam Map \n{beam_map}");
                trace!("West of {point:?} is {blubb:?}");
                if let Some(Beam(west_num)) = beam_map.get(point + Direction::West.as_offset()) {
                    beam_map
                        .set(
                            point + Direction::West.as_offset(),
                            Beam(west_num + above_num),
                        )
                        .expect("The point was just read..")
                } else {
                    let _ = beam_map.set(point + Direction::West.as_offset(), Beam(above_num));
                }
                let _ = beam_map.set(point + Direction::East.as_offset(), Beam(above_num));
            }
        }
    }
    debug!("\n{beam_map}");
    info!("Result Part 1: {split_count}");
    let last_row = beam_map.get_row((beam_map.outer_bound.y -1) as usize).unwrap();
    let beam_sum: u64 = last_row.iter().filter_map(|x| match x {
        Splitter| Empty| Start => None,
        Beam(num) => Some(num),
    }).sum();
    info!("Result Part 2: {beam_sum}");


}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Tile {
    Empty,
    Start,
    Splitter,
    Beam(u64),
}

impl From<u8> for Tile {
    fn from(b: u8) -> Self {
        match b {
            b'S' => Tile::Start,
            b'^' => Tile::Splitter,
            b'|' => Tile::Beam(0),
            _ => Tile::Empty,
        }
    }
}

impl From<Tile> for u8 {
    fn from(t: Tile) -> u8 {
        match t {
            Tile::Empty => b'.',
            Tile::Start => b'S',
            Tile::Splitter => b'^',
            Tile::Beam(0) => b'|',
            Tile::Beam(num) if num <= 9 => num.to_string().bytes().next().unwrap(),
            Tile::Beam(_) => b'X',
        }
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let b: u8 = (*self).into();
        write!(f, "{}", b as char)
    }
}
