// Calorie Counting
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;


fn main() {
    let path = Path::new("inputs/day01.txt");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut s = String::new();
    file.read_to_string(&mut s).expect("Could not read file");
    let split = s.split("\n");

    let mut elves: Vec<Vec<i32>> = Vec::new();
    elves.push(Vec::new());
    for line in split {
        let str_line = String::from(line);
        let elf: &mut Vec<i32> = elves.last_mut().expect("empty?!");
        let ref this = str_line;
        if this.len() > 0 {
            elf.push(this.parse().expect("bad input file"))
        } else {
            elves.push(Vec::new())
        }
    }
    let mut all_calors: Vec<i32> = Vec::new();
    for elf in elves {
        let calors: i32 = elf.iter().sum();
        all_calors.push(calors)
    }
    println!("Result Day 1 Part 1: {}", all_calors.iter().max().expect("This should have failed before..."));

    all_calors.sort_by(|a,b| b.cmp(a));
    println!("Result Day 1 Part 2: {}", all_calors[0] + all_calors[1] + all_calors[2]);
}