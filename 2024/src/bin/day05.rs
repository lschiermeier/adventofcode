use aoc_2024::*;
use regex::Regex;
use std::collections::{HashMap, HashSet};

struct BasicOrdering {
    before: u64,
    after: u64,
}

struct RelationToOrdering {
    number: u64,
    before: HashSet<u64>,
    after: HashSet<u64>,
}

fn main() {
    let src_name = file!();
    let input_path = gen_input_path(src_name, true);
    let lines = read_strings(input_path).expect("File not found.");
    let mut line_iter = lines.into_iter();
    let mut orderings: Vec<BasicOrdering> = vec![];
    let re = Regex::new(r"(\d+)|(\d+)").unwrap();
    while let Some(line) = line_iter.next() {
        let (_full, [before, after]) = re.captures(&line).unwrap().extract();
        orderings.push(BasicOrdering {
            before: before.parse().unwrap(),
            after: after.parse().unwrap(),
        });
        let relation = HashMap::<u64, RelationToOrdering>::new();
    }
    let mut updates: Vec<Vec<u64>> = vec![];
    while let Some(line) = line_iter.next() {
        updates.push(line.split(",").map(|x| x.parse().unwrap()).collect());
    }

    let sum_p1 = todo!();
    println!("{src_name} - Result 1: {sum_p1}");

    let sum_p2 = todo!();
    println!("{src_name} - Result 2: {sum_p2}");
}

pub fn check_update(update: &Vec<u64>, orderings: &Vec<BasicOrdering>) -> bool {
    
}

#[cfg(test)]
mod tests {
    use super::*;
}
