use aoc_2025::*;
use itertools::Itertools;
extern crate pretty_env_logger;
#[macro_use]
extern crate log;

fn main() {
    pretty_env_logger::formatted_timed_builder()
        .filter_level(log::LevelFilter::Info)
        .init();
    info!("Starting {}", file!());
    let args = std::env::args().collect::<Vec<String>>();
    let test_mode = args.len() > 1 && args[1] == "test";
    let src_name = file!();

    let input_path = gen_input_path(src_name, test_mode);
    let lines = read_strings(input_path).expect("File could not be opened.");

    let line = &lines[0];
    let ranges: Vec<_> = line
        .split(',')
        .map(|pair| {
            let mut parts = pair.split('-');
            let left = parts.next().unwrap().parse::<i64>().unwrap();
            let right = parts.next().unwrap().parse::<i64>().unwrap();
            (left, right)
        })
        .collect();
    let invalid_ids: Vec<Vec<i64>> = ranges
        .iter()
        .map(|x| find_invalid_ids(*x, is_valid_id_2_reps))
        .collect();
    let sum_p1: i64 = invalid_ids.iter().map(|x| x.iter().sum::<i64>()).sum();
    debug!("invalid_ids: {:?}", invalid_ids);
    info!("{src_name} - Result 1: {sum_p1}");
    let invalid_ids: Vec<Vec<i64>> = ranges
        .iter()
        .map(|x| find_invalid_ids(*x, is_valid_id_any_reps))
        .collect();
    let sum_p2: i64 = invalid_ids.iter().map(|x| x.iter().sum::<i64>()).sum();
    info!("{src_name} - Result 2: {sum_p2}");
}

fn find_invalid_ids(range: (i64, i64), is_valid_func: fn(i64) -> bool) -> Vec<i64> {
    let mut invalid_ids = Vec::new();
    for id in range.0..=range.1 {
        if !is_valid_func(id) {
            invalid_ids.push(id);
        }
    }
    invalid_ids
}

fn is_valid_id_2_reps(id: i64) -> bool {
    let digits: Vec<char> = id.to_string().chars().collect();
    if digits.len() % 2 != 0 {
        return true;
    }
    let (first, second) = digits.split_at(digits.len() / 2);
    first != second
}

fn is_valid_id_any_reps(id: i64) -> bool {
    let digits: Vec<char> = id.to_string().chars().collect();
    for i in 1..digits.len() {
        if digits.len() % i != 0 {
            continue;
        }
        let chunks = digits.chunks(i);
        if chunks.clone().all_equal() {
            return false;
        }
    }
    true
}
