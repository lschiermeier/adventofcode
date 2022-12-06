use aoc_2022::*;
use regex::Regex;

// Supply Stacks
fn main() {
    let inputs = read_lines("inputs/day05.txt").expect("coudn't open file");
    let mut stack_definition: Vec<String> = Vec::new();
    let mut instruction_set: Vec<(usize, usize, usize)> = Vec::new();
    let instruction_regex = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();

    let mut stacks_defined = false;
    for line in inputs {
        let line = line.expect("line isn't readable");
        if line.is_empty() {
            stacks_defined = true;
            continue;
        }
        if !stacks_defined {
            stack_definition.push(line);
        } else {
            let caps = instruction_regex.captures(&line).expect("no match found");
            let instruction: (usize, usize, usize) = (
                caps.get(1).unwrap().as_str().parse().unwrap(),
                caps.get(2).unwrap().as_str().parse().unwrap(),
                caps.get(3).unwrap().as_str().parse().unwrap(),
            );
            instruction_set.push(instruction);
        }
    }
    println!("max stack height = {}, instructions parsed = {}", stack_definition.len() - 1, instruction_set.len());

    let mut slow_stacks: Vec<Vec<char>> = vec![Vec::new(); (stack_definition[0].len() + 1) / 4];
    
    for layer in stack_definition.iter().rev() {
        for index in 0..slow_stacks.len() {
            let letter = layer.chars().nth(1+index*4).unwrap();
            if letter.is_alphabetic() {
                slow_stacks[index].push(letter);
            }
        }
    }

    let mut fast_stacks = slow_stacks.clone();

    for (num, src, dest) in &instruction_set {
        move_boxes_slow(&mut slow_stacks, num, src, dest);
    }
    print!("Result Day 05 Part 1: ");
    print_stack_tops(&slow_stacks);

    for (num, src, dest) in &instruction_set {
        move_boxes_fast(&mut fast_stacks, num, src, dest)
    }
    print!("Result Day 05 Part 2: ");
    print_stack_tops(&fast_stacks);
}


fn move_boxes_slow(stacks: &mut Vec<Vec<char>>, num: &usize, src: &usize, dest: &usize) {
    if *num > 1 {
        move_boxes_slow(stacks, &(num - 1), src, dest);
    }
    let moved_box = stacks[src-1].pop().expect("Stack was empty...");
    stacks[dest-1].push(moved_box);
}

fn move_boxes_fast(stacks: &mut Vec<Vec<char>>, num: &usize, src: &usize, dest: &usize) {
    let move_idx = stacks[src-1].len()-num;
    let moved_boxes: Vec<char> = stacks[src-1].drain(move_idx..).collect();
    stacks[dest-1].extend(moved_boxes);
}

fn print_stack_tops(stacks: &Vec<Vec<char>>) {
    for stack in stacks {
        print!("{}", stack.last().unwrap())
    }
    println!("");
}