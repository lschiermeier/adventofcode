use aoc_2024::*;
use regex::Regex;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct BasicOrdering {
    before: u64,
    after: u64,
}

#[derive(Debug)]
struct PageOrder {
    number: u64,
    before: HashSet<u64>,
    after: HashSet<u64>,
}

impl Ord for PageOrder {
    fn cmp(&self, other: &Self) -> Ordering {
        if self == other {
            Ordering::Equal
        } else if self.before.contains(&other.number) && other.after.contains(&self.number) {
            Ordering::Less
        } else if self.after.contains(&other.number) && other.before.contains(&self.number) {
            Ordering::Greater
        } else {
            panic!("Ordering failed {self:?}.cmp({other:?})")
        }
    }
}

impl PartialOrd for PageOrder {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for PageOrder {
    fn eq(&self, other: &Self) -> bool {
        self.number == other.number
    }
}

impl Eq for PageOrder {}

fn main() {
    let src_name = file!();
    let input_path = gen_input_path(src_name, false);
    let lines = read_strings(input_path).expect("File not found.");
    let mut line_iter = lines.into_iter();
    let mut orderings: Vec<BasicOrdering> = vec![];
    let re = Regex::new(r"(\d+)\|(\d+)").unwrap();
    while let Some(line) = line_iter.next() {
        if line == "" {
            break;
        }
        let (_full, [before, after]) = re.captures(&line).unwrap().extract();
        orderings.push(BasicOrdering {
            before: before.parse().unwrap(),
            after: after.parse().unwrap(),
        });
    }
    let relations = build_relation_orderings(orderings);
    // println!("{relations:#?}");

    let mut updates: Vec<Vec<u64>> = vec![];
    while let Some(line) = line_iter.next() {
        updates.push(line.split(",").map(|x| x.parse().unwrap()).collect());
    }
    let (correct_updates, mut incorrect_updates): (Vec<_>, Vec<_>) =
        updates.into_iter().partition(|x| {
            x.is_sorted_by(|a, b| relations.get(a).unwrap() > relations.get(b).unwrap())
        });
    // println!("{correct_updates:?}");
    let sum_p1: u64 = correct_updates.iter().map(|x| x[x.len() / 2]).sum();
    println!("{src_name} - Result 1: {sum_p1}");

    for update in incorrect_updates.iter_mut() {
        update.sort_by(|a, b| relations.get(a).unwrap().cmp(relations.get(b).unwrap()));
    }
    let sum_p2: u64 = incorrect_updates.iter().map(|x| x[x.len() / 2]).sum();
    println!("{src_name} - Result 2: {sum_p2}");
}

fn build_relation_orderings(orderings: Vec<BasicOrdering>) -> HashMap<u64, PageOrder> {
    let mut relations: HashMap<u64, PageOrder> = HashMap::new();
    for BasicOrdering { before, after } in orderings {
        let _before_entry = relations
            .entry(before)
            .and_modify(|x| {
                x.after.insert(after);
            })
            .or_insert(PageOrder {
                number: before,
                before: HashSet::new(),
                after: HashSet::from([after]),
            });
        let _after_entry = relations
            .entry(after)
            .and_modify(|x| {
                x.before.insert(before);
            })
            .or_insert(PageOrder {
                number: after,
                before: HashSet::from([before]),
                after: HashSet::new(),
            });
    }
    relations
}
