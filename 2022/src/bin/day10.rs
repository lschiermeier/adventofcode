#[allow(unused)]
use aoc_2022::*;
use regex::Regex;

// Rope Bridge
fn main() {
    let lines = read_lines("inputs/day10.txt").expect("file coudn't be read");
    let reg = Regex::new(r"^(\w+) *(.*)$").expect("Regex compile failed.");
    let mut instructions: Vec<Instruction> = Vec::new();
    for line in lines.map(|x| x.unwrap()) {
        let caps = reg.captures(&line).expect("Regex execution failed");
        instructions.push(match caps.get(1).expect("capture 1 empty").as_str() {
            "noop" => Instruction::Noop,
            "addx" => Instruction::Addx(
                caps.get(2)
                    .expect("capture 2 empty")
                    .as_str()
                    .parse()
                    .unwrap(),
            ),
            inst => panic!("unknown instruction: {}", inst),
        });
    }

    let mut current_state = State { cycle: 1, reg_x: 1 };
    let mut state_history = vec![current_state];

    for inst in instructions {
        match inst {
            Instruction::Noop => {
                current_state = State {
                    cycle: current_state.cycle + 1,
                    reg_x: current_state.reg_x,
                };
                state_history.push(current_state);
            },
            Instruction::Addx(num) => {
                current_state = State {
                    cycle: current_state.cycle + 1,
                    reg_x: current_state.reg_x
                };
                state_history.push(current_state);
                current_state = State {
                    cycle: current_state.cycle + 1,
                    reg_x: current_state.reg_x + num
                };
                state_history.push(current_state);
            }
        }
    }
    let interesting_cycles = [20,60,100,140,180,220];
    let signal_strength: i64 = state_history
        .iter()
        .filter(|x| interesting_cycles.contains(&x.cycle))
        .map(|x| x.cycle*x.reg_x)
        .sum();

    println!("Result Day 10 Part 1: {:?}", signal_strength);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Instruction {
    Noop,
    Addx(i64),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct State {
    cycle: i64,
    reg_x: i64,
}

impl State {}
