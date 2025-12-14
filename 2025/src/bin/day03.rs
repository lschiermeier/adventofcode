use aoc_2025::*;
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
    let lines = read_strings(input_path).expect("File could not be opened.");
    let mut jolts_p1: Vec<u128> = Vec::new();
    let mut jolts_p2: Vec<u128> = Vec::new();
    for line in lines {
        let max_jolt_2 = calc_max_jolt_recursive(&line, 0, 1);
        let max_jolt_12 = calc_max_jolt_recursive(&line, 0, 11);
        debug!("Line: {line} => Max bank 2: {max_jolt_2:?}, Max bank 12: {max_jolt_12:?}");
        jolts_p1.push(max_jolt_2);
        jolts_p2.push(max_jolt_12);
    }
    let sum_p1 = jolts_p1.iter().sum::<u128>();
    info!("Part 1: Sum of jolts is {sum_p1}");
    let sum_p2 = jolts_p2.iter().sum::<u128>();
    info!("Part 2: Sum of jolts is {sum_p2}");
}

fn calc_max_jolt_recursive(s: &str, start_pos: usize, depth_left: usize) -> u128 {
    let max_char = s[..s.len() - depth_left]
        .chars()
        .skip(start_pos)
        .max()
        .unwrap();
    let pos = s[..s.len() - depth_left]
        .chars()
        .skip(start_pos)
        .position(|c| c == max_char)
        .unwrap()
        + start_pos;
    let max_num: u128 = max_char.to_digit(10).unwrap() as u128;
    if depth_left == 0 {
        max_num
    } else {
        calc_max_jolt_recursive(s, pos + 1, depth_left - 1)
            + max_num * 10u128.pow(depth_left as u32)
    }
}
