use aoc_2025::*;
use itertools::Itertools;
extern crate pretty_env_logger;
#[macro_use]
extern crate log;

fn main() {
    pretty_env_logger::formatted_timed_builder()
        .init();
    info!("Starting {}", file!());
    let args = std::env::args().collect::<Vec<String>>();
    let test_mode = args.len() > 1 && args[1] == "test";
    let src_name = file!();

    let input_path = gen_input_path(src_name, test_mode);
    let lines = read_strings(input_path).expect("File could not be opened.");
    let mut jolts: Vec<u64> = Vec::new();
    for line in lines {
        let first_max_pos = line[..line.len()-1].chars().rev().position_max().unwrap();
        let second_max_pos = line[line.len()-first_max_pos..].chars().rev().position_max().unwrap();

        let jolt_str = format!("{:?}{:?}",line.chars().rev().nth(first_max_pos).unwrap(), line.chars().rev().nth(second_max_pos).unwrap());
        debug!("bank: {line}\n    batteries: ({first_max_pos},{second_max_pos}): Jolts {jolt_str}");
        jolts.push(jolt_str.parse::<u64>().unwrap());
    }
    let sum_p1 = jolts.iter().sum::<u64>();
    println!("Part 1: Sum of jolts is {sum_p1}");
}