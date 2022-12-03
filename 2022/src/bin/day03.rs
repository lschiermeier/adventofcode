use aoc_2022::*;
use std::collections::HashSet;

// Rucksack Reorganization
#[allow(unused)]
fn main() {
    let lines = read_lines("inputs/day03.txt").expect("coudn't open file");
    let mut rucksacks = Vec::new();
    for line in lines {
        let line = line.expect("parsing wasn't finished");
        assert!(line.len() % 2 == 0, "length not even");
        let half_len = line.len() / 2;
        let splits = line.split_at(half_len);
        let mut first_half: HashSet<char> = HashSet::new();
        for letter in splits.0.chars() {
            first_half.insert(letter);
        }
        let mut second_half: HashSet<char> = HashSet::new();
        for letter in splits.1.chars() {
            second_half.insert(letter);
        }
        rucksacks.push((first_half, second_half));
    }
    let mut all_doubles = Vec::<char>::new();
    for (first_comp, second_comp) in &rucksacks {
        let doubles: Vec<&char> = first_comp.intersection(&second_comp).collect();
        assert!(doubles.len() == 1, "Line didn't contain exactly one double...");
        all_doubles.push(doubles[0].to_owned());
    }

    let result: u32 = all_doubles.iter().map(letter_to_score).sum();
    println!("Result Day 3 Part 1: {}", result);


    let mut badges = Vec::<char>::new();
    for  team  in rucksacks.chunks(3) {
        let elf_1:HashSet<_> = team[0].0.union(&team[0].1).cloned().collect();
        let elf_2:HashSet<_> = team[1].0.union(&team[1].1).cloned().collect();
        let elf_3:HashSet<_> = team[2].0.union(&team[2].1).cloned().collect();
        let team_intersection: HashSet<char> = elf_1.intersection(&elf_2).cloned().collect();
        let team_intersection: HashSet<char> = team_intersection.intersection(&elf_3).cloned().collect();
        assert!(team_intersection.len() == 1, "wrong badge found");
        badges.push(team_intersection.iter().next().expect("intersection went wrong").to_owned())
    }
    let result: u32 = badges.iter().map(letter_to_score).sum();
    println!("Result Day 3 Part 2: {}", result);
}

fn letter_to_score(letter: &char) -> u32 {
    if char::is_lowercase(*letter) {
        letter.to_digit(36).expect("wasn't lower case") - 9
    } else {
        letter.to_ascii_lowercase().to_digit(36).expect("conversion didn't work") + 17
    }
}