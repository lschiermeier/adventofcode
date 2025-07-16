use std::{fs::read_to_string, vec};

use aoc_2024::*;
use regex::{Regex};

fn main() {
    let src_name = file!();
    let input_path = gen_input_path(src_name, false);
    let text = read_to_string(input_path).unwrap();

    let re_mul = regex::Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let mut list_just_mul: Vec<i64> = vec![];

    for cap in re_mul.captures_iter(&text) {
        let (_, [x_str, y_str]) = cap.extract();
        list_just_mul.push(x_str.parse::<i64>().unwrap() * y_str.parse::<i64>().unwrap());
    }

    let sum_p1: i64 = list_just_mul.iter().sum();
    println!("{src_name} - Result 1: {sum_p1}");

    let re_do = Regex::new(r"(mul|do|don't)\((?:(\d*),(\d*))?\)").unwrap();

    let mut list_do_dont_mul: Vec<i64> = vec![];

    let mut enable = true;
    for cap in re_do.captures_iter(&text) {
        let _full = cap.get(0).unwrap().as_str();
        let command = cap.get(1).unwrap().as_str();
        match command {
            "do" => enable = true,
            "don't" => enable = false,
            "mul" => {
                if enable {
                    list_do_dont_mul.push(
                        cap.get(2).unwrap().as_str().parse::<i64>().unwrap()
                            * cap.get(3).unwrap().as_str().parse::<i64>().unwrap(),
                    );
                }
            }
            x => panic!("unknown command matched {x}"),
        }
    }

    let sum_p2:i64 = list_do_dont_mul.iter().sum();
    println!("{src_name} - Result 2: {sum_p2}");
}

#[cfg(test)]
mod tests {
    use super::*;
}
