use aoc_2022::*;
use regex::Regex;

// Camp Cleanup
fn main() {
    let lines = read_lines("inputs/day04.txt").expect("could not read file.");
    let reg = Regex::new(r"^(\d+)-(\d+),(\d+)-(\d+)$").unwrap();
    let mut self_contained_counter: u32 = 0;
    let mut overlap_sum = 0;
    for line in lines {
        let str = line.unwrap();
        let caps = reg.captures(str.as_str()).expect("Capturing failed.");
        let section_a = (
            caps[1].parse::<u32>().unwrap(),
            caps[2].parse::<u32>().unwrap(),
        );
        let section_b = (
            caps[3].parse::<u32>().unwrap(),
            caps[4].parse::<u32>().unwrap(),
        );
        let (is_selfcontained, overlap) = check_overlap(section_a, section_b);
        if is_selfcontained {
            self_contained_counter += 1;
        }
        if overlap >= 1 {
            overlap_sum += 1;
        }
    }

    println!("Result Day 4 Part 1: {}", self_contained_counter);
    println!("Result Day 4 Part 2: {}", overlap_sum);
}

fn check_overlap(section_a: (u32,u32), section_b: (u32,u32)) -> (bool, usize) {
    let range_a = section_a.0..section_a.1+1;
    let range_b = section_b.0..section_b.1+1;
    // println!("{},{} - {},{}", section_a.0, section_a.1, section_b.0, section_b.1);

    let shorter;
    let longer;
    let len_of_shorter;
    if range_a.len() > range_b.len() {
        longer = range_a;
        shorter = range_b;
        len_of_shorter = shorter.len();
    } else {
        longer = range_b;
        shorter = range_a;
        len_of_shorter = shorter.len();
    }
    let mut overlap = 0;
    for i in shorter {
        if longer.contains(&i) {
            overlap += 1;
        }
    }
    let full_overlap = overlap >= len_of_shorter;
    // println!("{}, {} ({})", full_overlap, overlap, len_of_shorter);
    (full_overlap, overlap)
}
