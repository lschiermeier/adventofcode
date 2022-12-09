use aoc_2022::*;
use regex::Regex;

// Treetop Tree House
#[allow(unused)]
fn main() {
    let reg = Regex::new(r"^([$\w\d]+) ([\w\d\.]+)(.*)$").unwrap();

    let lines = read_lines("inputs/day07_test.txt").expect("file couldn't be read");
    let lines:Vec<String> = lines.map(|x| x.unwrap()).collect();
    let mut trees = vec![vec![0u32;lines.len()];lines.len()];
    let mut first = true;
    for (y_idx, line) in lines.iter().enumerate() {
        for (x_idx, letter) in line.chars().enumerate() {
            trees[x_idx][y_idx] = letter.to_digit(10).unwrap();
        }
    }
}

