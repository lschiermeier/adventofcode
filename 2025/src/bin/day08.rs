use aoc_2025::*;
#[macro_use]
extern crate log;
extern crate pretty_env_logger;

fn main() {
    pretty_env_logger::init();
    info!("Starting {}", file!());
    let args = std::env::args().collect::<Vec<String>>();
    let test_mode = args.len() > 1 && args[1] == "test";
    let src_name = file!();

    let input_path = gen_input_path(src_name, test_mode);
    let starting_cords =
        parse_table::<i64>(read_table(input_path, ',').expect("File could not be opened."));
    let junction_cords: Vec<Point3d> = starting_cords
        .iter()
        .map(|x| x.try_into().unwrap())
        .collect();
    let junction_dists: Vec<Vec<i64>> = junction_cords
        .iter()
        .copied()
        .map(|y| junction_cords.iter().copied().map(|x| y ^ x).collect())
        .collect();
    
}
