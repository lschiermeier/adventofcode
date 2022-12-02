use std::fs::File;
use std::io::prelude::*;
use std::path::Path;


// Rock Paper Scissors
fn main() {
    let path = Path::new("inputs/day02.txt");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut input = String::new();
    file.read_to_string(&mut input).expect("Could not read file");

    let mut score_acum: i32 = 0;
    for line in input.split("\n") {
        if !line.is_empty() {
            score_acum += score(
                line.chars().nth(0).expect("empty line"),
                line.chars().nth(2).expect("too short line"));
            
        }
    }

    println!("Result Day 2 Part 1: {}", score_acum);

    let mut score_acum = 0;
    for line in input.split("\n") {
        if !line.is_empty() {
            let enemy_shape = line.chars().nth(0).expect("empty line");
            let own_shape = strategy_to_shape(
                enemy_shape,
                line.chars().nth(2).expect("too short line")
            );
            score_acum += score(enemy_shape, own_shape);
        }
    }

    println!("Result Day 2 Part 2: {}", score_acum);
}

fn score(enemy_shape: char, own_shape: char) -> i32 {
    let own_score = match own_shape {
        'X' => 1,
        'Y' => 2,
        'Z' => 3,
         _  => 0
    };
    own_score + match (enemy_shape, own_shape) {
        ('A', 'X') => 3,
        ('B', 'Y') => 3,
        ('C', 'Z') => 3,
        ('A', 'Y') => 6,
        ('B', 'Z') => 6,
        ('C', 'X') => 6,
        ( _ ,  _ ) => 0,
    }
}

fn strategy_to_shape(enemy_shape: char, own_strategy:char) -> char {
    match (enemy_shape, own_strategy) {
        ('A', 'X') => 'Z',
        ('B', 'X') => 'X',
        ('C', 'X') => 'Y',
        ('A', 'Y') => 'X',
        ('B', 'Y') => 'Y',
        ('C', 'Y') => 'Z',
        ('A', 'Z') => 'Y',
        ('B', 'Z') => 'Z',
        ('C', 'Z') => 'X',
        ( _ ,  _ ) => '?',
    }
}