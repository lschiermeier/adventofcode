use aoc_2022::*;
use regex::{Regex, Captures, Match};
use lazy_static::lazy_static;

fn main() {
    let mut lines = read_lines("inputs/day12").expect("File couldn't be read");
    let mut packet_pairs: Vec<(Element, Element)> = Vec::new();
    while let Some(Ok(line)) = lines.next() {
        if line.is_empty() {
            continue;
        }
        let first = parse_packet(line);
    }
}

enum Element {
    Num(u32),
    List(Vec<Element>),
}

fn parse_packet(line: String) -> Element {
    lazy_static!{
        static ref REG: Regex = Regex::new(r"[(\[)(\d+)(\])]").unwrap();
    }
    let caps = REG.captures("line").expect("Regex failed");
    let caps: Vec<_> = caps.iter().map(|x| x.unwrap().to_owned().as_str().to_owned()).collect();
    if let Some(packet) = parse_elem(0, caps) {
        return packet;
    } else {
        panic!("packet parsing failed")
    }
}

fn parse_elem(idx: usize, caps: Vec<String>) -> Option<Element> {
    
    match caps[idx].as_str() {
        "[" => {
            // make vec
            // repeated parsing for elems
            // return might need to include idx...
            todo!()
        },
        "]" => None,
        num => if let Ok(num) = num.parse::<u32>() {
            Some(Element::Num(num))
        } else {
            None
        }
    }
}