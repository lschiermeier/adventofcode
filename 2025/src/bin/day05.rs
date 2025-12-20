use aoc_2025::*;
use itertools::{Itertools, MinMaxResult};
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
    let lines = read_table(input_path, '-').expect("File could not be opened.");
    let mut ranges: Vec<Vec<i64>> = lines
        .iter()
        .map(|row| row.iter().map(|num| num.parse().unwrap()).collect())
        .collect();
    let ids = ranges
        .split_off(ranges.partition_point(|x| x.len() == 2))
        .concat();
    // debug!("in IDs: {ids:?}");

    let fresh_id_count = ids
        .iter()
        .filter(|id| check_freshness(&ranges, **id))
        .count();
    info!("Part 1: num of fresh ids: {}", fresh_id_count);
    if let MinMaxResult::MinMax(min, max) = ranges.iter().flatten().minmax() {
        info!("Range number: {}", ranges.len());
        info!("ID range: {min} - {max}");
    }
    ranges.sort_by(|x, y| x[0].cmp(&y[0]).then(x[1].cmp(&y[1])));
    trace!(
        "range sizes: {:?}",
        ranges.iter().map(|x| x[1] - x[0]).collect_vec()
    );
    let mut max_range_member: i64 = 0;
    let mut num_range_members: i64 = 0;
    for (i, (left,right)) in ranges.iter().flatten().tuples().enumerate() {
        debug!("range {i}: {left} -- {right}");
        if *right > max_range_member {
            if *left <= max_range_member {
                num_range_members += right - max_range_member
            } else {
                num_range_members += right - left + 1
            }
            max_range_member = *right
        }
    }
    info!("Part 2: total num of fresh ids: {num_range_members}");
}

fn check_freshness(ranges: &Vec<Vec<i64>>, id: i64) -> bool {
    for range in ranges {
        if id >= range[0] && id <= range[1] {
            return true;
        }
    }
    false
}
