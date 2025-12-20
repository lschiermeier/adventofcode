use core::num;
use std::{iter, ops, thread::yield_now, vec};

use aoc_2025::*;
use itertools::{Itertools, Zip, rev};
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
        let calc_func = match operation.as_str() {
            "+" => |x: u64, y: u64| x + y,
            "*" => |x: u64, y: u64| x * y,
            _ => panic!("Unknown operation in calculation"),
        };
        let init = match operation.as_str() {
            "+" => 0,
            "*" => 1,
            _ => panic!("Unknown operation in calculation"),
        };
        let col_result: u64 = table_strings
            .iter()
            .map(|x| x.parse().unwrap())
            .fold(init, calc_func);
        debug!("Part 1 Col {i} with Op '{operation}' = {col_result} ");
        col_results_p1.push(col_result);
    }
    let sum = col_results_p1.iter().sum::<u64>();
    info!("Part 1: Sum of all column results: {sum}");

    // start part 2
    let mut num_vecs = read_chars(input_path).unwrap();
    let op_vec = num_vecs.pop().unwrap();
    let vert_nums = transpose(num_vecs).unwrap();
    debug!("\n{vert_nums:?}");

    // let sum = col_results_p2.iter().sum::<u64>();
    // info!("Part 1: Sum of all column results: {sum}");
}
