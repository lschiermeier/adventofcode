use std::vec;

use aoc_2025::*;
use itertools::Itertools;
extern crate pretty_env_logger;
#[macro_use]
extern crate log;

fn main() {
    pretty_env_logger::init();
    info!("Starting {}", file!());
    let args = std::env::args().collect::<Vec<String>>();
    let test_mode = args.len() > 1 && args[1] == "test";
    let src_name = file!();

    let input_path = gen_input_path(src_name, test_mode);
    let mut table = read_table(&input_path, ' ').expect("File could not be opened.");
    let operations = table.pop().unwrap();
    let mut table_iters = table.iter().map(|row| row.iter()).collect_vec();
    let mut col_results_p1: Vec<u64> = vec![];
    for (i, operation) in operations.iter().enumerate() {
        let table_strings = table_iters
            .iter_mut()
            .map(|x| x.next().unwrap().to_owned())
            .collect_vec();
        let (init, calc_func) = get_calc_func(operation.chars().next().unwrap());
        let col_result: u64 = table_strings
            .iter()
            .map(|x| x.parse::<u64>().unwrap())
            .fold(init, calc_func);
        debug!("Part 1 Col {i} with Op '{operation}' = {col_result} ");
        col_results_p1.push(col_result);
    }
    let sum = col_results_p1.iter().sum::<u64>();
    info!("Part 1: Sum of all column results: {sum}");

    // start part 2
    let mut num_vecs = read_chars(input_path).unwrap();
    let op_vec = num_vecs
        .pop()
        .unwrap()
        .iter()
        .filter(|x| **x != ' ')
        .map(|c| get_calc_func(*c))
        .collect_vec();
    let mut op_iter = op_vec.iter().rev();
    let vert_nums = parse_chars(transpose(num_vecs).unwrap());
    // debug!("\n{vert_nums:?}");
    let build_nums = vert_nums
        .iter()
        .rev()
        .map(|x| {
            if x.len() == 0 {
                None
            } else {
                Some(x.iter().fold(0, |acc: u64, y| acc * 10 + (*y as u64)))
            }
        })
        .collect_vec();
    debug!("{build_nums:?}");
    let subsequences = build_nums
        .split(|x| *x == None)
        .map(|x| x.iter().map(|x| x.unwrap() as u64).to_owned().collect_vec())
        .collect_vec();
    let col_results_p2 = subsequences
        .iter()
        .map(|x| {
            let (init, func) = op_iter.next().unwrap();
            x.iter().copied().fold(*init, func)
        })
        .collect_vec();
    debug!("{col_results_p2:?}");
    let sum = col_results_p2.iter().sum::<u64>();
    info!("Part 1: Sum of all column results: {sum}");
}

pub fn get_calc_func(c: char) -> (u64, Box<dyn Fn(u64, u64) -> u64>) {
    match c {
        '*' => (1, Box::new(|x: u64, y: u64| x * y)),
        '+' => (0, Box::new(|x: u64, y: u64| x + y)),
        _ => panic!("Unknown operation in calculation"),
    }
}
