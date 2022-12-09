use std::collections::HashSet;

#[allow(unused)]
use aoc_2022::*;
use regex::Regex;

// Rope Bridge
fn main() {
    let lines = read_lines("inputs/day09.txt").expect("file coudn't be read");
    let reg = Regex::new(r"^(.) (\d+)$").expect("Regex compile failed.");
    let mut instructions: Vec<(Direction, i32)> = Vec::new();
    for line in lines.map(|x| x.unwrap()){
        let caps = reg.captures(&line).expect("Regex execution failed");
        let dir = match caps.get(1).unwrap().as_str() {
            "R" => Some(Direction::Right),
            "L" => Some(Direction::Left),
            "U" => Some(Direction::Up),
            "D" => Some(Direction::Down),
            _ => None
        }.expect("Unknown direction");
        let dist: i32 = caps.get(2).unwrap().as_str().parse().expect("number parsing failed");
        instructions.push((dir,dist));
    }
    let mut head_location: Location = Location(0,0);
    let mut tail_location: Location = Location(0,0);
    let mut tail_history = HashSet::new();
    tail_history.insert(tail_location);
    for (dir, dist) in &instructions {
        for _i in 0..*dist {
            let former_head_location = head_location;
            head_location = move_knot(head_location, dir, 1);
            if (tail_location.0 - head_location.0).abs() > 1 || (tail_location.1 - head_location.1).abs() > 1 {
                tail_location = former_head_location;
                tail_history.insert(tail_location);
            }
        }
    }
    println!("Result Day 9 Part 1: {}", tail_history.len());
}

enum Direction {
    Up,
    Down,
    Right,
    Left
}

#[derive(Eq, Hash, PartialEq, Debug, Copy, Clone)]
struct Location(i32,i32);

fn move_knot(start_location: Location, dir: &Direction, dist: i32) -> Location {
    match dir {
        Direction::Up    => Location(start_location.0       , start_location.1 - dist),
        Direction::Down  => Location(start_location.0       , start_location.1 + dist),
        Direction::Right => Location(start_location.0 + dist, start_location.1),
        Direction::Left  => Location(start_location.0 - dist, start_location.1)
    }
}
