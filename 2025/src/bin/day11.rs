use aoc_2025::*;
use itertools::Itertools;
#[macro_use]
extern crate log;
extern crate pretty_env_logger;

#[allow(unused)]
fn main() {
    pretty_env_logger::init();
    info!("Starting {}", file!());
    let args = std::env::args().collect::<Vec<String>>();
    let test_mode = args.len() > 1 && args[1] == "test";
    let src_name = file!();

    let input_path = gen_input_path(src_name, test_mode);
    let starting_table = read_table(input_path, ' ').expect("File could not be opened.");




}