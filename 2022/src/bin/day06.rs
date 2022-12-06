use std::cmp::max;

use aoc_2022::*;

// Tuning Trouble
#[allow(unused)]
fn main() {
    let lines = read_lines("inputs/day06.txt").expect("File couldn't be read.");
    for line in lines {
        let line = line.expect("Line not parsed.");
        let package_starts = find_all_marker(&line, 4);
        println!("Result Day 6 Part 1: {}", package_starts[0].0);

        let package_starts = find_all_marker(&line, 14);
        println!("Result Day 6 Part 2: {}", package_starts[0].0);
    }
}

fn check_start_marker(marker: &str) -> bool{
    let mut chars = Vec::<char>::new();
    for letter in marker.chars() {
        if chars.contains(&letter) {
            return false;
        } else {
            chars.push(letter);
        }
    }
    true
}

fn find_all_marker(line: &String, marker_len: usize) -> Vec<(usize,char)>{
    let mut package_starts: Vec<(usize,char)> = Vec::new();
    for (index, letter) in line.char_indices() {
        let end_index = index + marker_len;
        if end_index >= line.len() {
            break;
        }
        if check_start_marker(&line[index..end_index]) {
            package_starts.push((index+marker_len,letter));
        }
    }
    package_starts
}