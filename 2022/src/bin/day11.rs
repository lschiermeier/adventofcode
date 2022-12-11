#[allow(unused)]
use aoc_2022::*;
use lazy_static::lazy_static;
use regex::Regex;

// Rope Bridge
fn main() {
    let mut lines = read_lines("inputs/day11.txt").expect("file coudn't be read");
    let mut monkeys: Vec<Monkey> = vec![];
    while let Some(Ok(_monkey)) = lines.next() {
        let items = lines.next().unwrap().unwrap();
        let items: Vec<usize> = items[18..]
            .split(", ")
            .map(|x| x.parse().expect("parsing of start items failed."))
            .collect();
        let operation = lines.next().unwrap().unwrap().as_str().to_owned();
        let operation = match (
            operation.split(" ").nth(6).unwrap(),
            operation.split(" ").nth(7).unwrap(),
        ) {
            ("*", "old") => Operation::Sqr,
            ("*", num) => Operation::Mul(num.parse().unwrap()),
            ("+", num) => Operation::Add(num.parse().unwrap()),
            op => panic!("unknown operation {:?}", op),
        };
        let test: usize = get_last_number(lines.next().unwrap().unwrap().as_str());
        let true_target: usize = get_last_number(lines.next().unwrap().unwrap().as_str());
        let false_target: usize = get_last_number(lines.next().unwrap().unwrap().as_str());
        monkeys.push(
            Monkey { 
                items: items, 
                op: operation, 
                test: test, 
                true_target: true_target, 
                false_target: false_target, 
                inspections: 0 
            }
        );
        lines.next();
    }

    for round in 0..20 {
        perform_inspections(&mut monkeys);
        // println!("After Round {:?}: {:?}", round+1, monkeys);
    }
    monkeys.sort_by(|a, b| b.inspections.cmp(&a.inspections));
    let monkey_buisness = monkeys[0].inspections * monkeys[1].inspections;
    println!("Result Day 11 Part 1: {monkey_buisness}");
}

#[derive(Debug)]
struct Monkey {
    items: Vec<usize>,
    op: Operation,
    test: usize,
    true_target: usize,
    false_target: usize,
    inspections: usize
}

fn perform_inspections(monkeys: &mut Vec<Monkey>) {
    
    for i in 0..monkeys.len() {
        let true_target;
        let mut new_true_items: Vec<usize> = Vec::new();
        let false_target;
        let mut new_false_items: Vec<usize> = Vec::new();
        let monkey: &mut Monkey = monkeys.get_mut(i).expect("Monkey not found");
        // println!(" {:?}", monkey);
        true_target = monkey.true_target;
        false_target = monkey.false_target;
        for item in &monkey.items{
            let new_item = match monkey.op {
                Operation::Add(num) => item + num,
                Operation::Mul(num) => item * num,
                Operation::Sqr => item * item
            };
            let new_item = new_item / 3;
            if new_item % monkey.test == 0 {
                new_true_items.push(new_item);
            } else {
                new_false_items.push(new_item);
            }
            monkey.inspections += 1;
        }
        monkey.items.clear();
        let true_target = monkeys.get_mut(true_target).expect("Monkey not found");
        true_target.items.extend(new_true_items);
        let false_target = monkeys.get_mut(false_target).expect("Monkey not found");
        false_target.items.extend(new_false_items);
    }
}



#[derive(Debug)]
enum Operation {
    Mul(usize),
    Add(usize),
    Sqr,
}

fn get_last_number(str: &str) -> usize {
    lazy_static! {
        static ref RE: Regex = Regex::new(r".* (\d+)$").unwrap();
    }
    RE.captures(str).expect("applying Regex failed").get(1).unwrap().as_str().parse().unwrap()
}