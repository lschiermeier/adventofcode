use std::{fs::File, os};

use aoc_2024::*;

fn main() {
    let blubb = file!().rsplit(".").next().expect("Filename not found.");
    let lines = read_lines(["inputs/", blubb, ".txt"].join(""));
}