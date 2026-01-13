use std::collections::{HashMap, VecDeque};

use aoc_2025::*;
#[macro_use]
extern crate log;
extern crate pretty_env_logger;

#[allow(unused)]
fn main() {
    pretty_env_logger::init();
    info!("Starting {}", file!());
    let args = std::env::args().collect::<Vec<String>>();
    let test_mode = args.len() > 1 && args[1] == "test";
    let src_name = file!();

    let input_path = gen_input_path(src_name, test_mode);
    let starting_table = read_table(input_path, ' ').expect("File could not be opened.");

    let mut edges_from: HashMap<String, Vec<String>> = HashMap::new();

    for line in starting_table.iter() {
        let mut iter = line.iter().cloned();
        let source = iter.next().unwrap().chars().filter(|x| *x != ':').collect();
        let mut edges = vec![];
        for after in iter {
            edges.push(after);
        }
        edges_from.insert(source, edges);
    }

    let mut completed_paths: Vec<Vec<String>> = vec![];
    let mut started_paths: VecDeque<Vec<String>> = VecDeque::new();
    started_paths.push_back(vec!["you".to_owned()]);
    while started_paths.len() > 0 {
        let current_path = started_paths.pop_front().expect("just checked length");
        let afters = edges_from
            .get(current_path.last().unwrap())
            .expect("node not found");
        for after in afters {
            if let Some(_) = current_path.iter().find(|x| *x == after) {
                // Node already in skip
                continue;
            } else {
                let mut new_path = current_path.clone();
                new_path.push(after.clone());
                if after == "out" {
                    trace!("Path completed: {:?}", new_path);
                    completed_paths.push(new_path);
                } else {
                    started_paths.push_back(new_path);
                }
            }
        }
    }
    debug!("Result P1: {}", completed_paths.len());

    let starting_table = if test_mode {
        let input_path = "inputs/day11_test_2.txt";
        read_table(input_path, ' ').expect("File could not be opened.")
    } else {
        starting_table
    };

    let mut edges_from: HashMap<String, Vec<String>> = HashMap::new();

    for line in starting_table.iter() {
        let mut iter = line.iter().cloned();
        let source = iter.next().unwrap().chars().filter(|x| *x != ':').collect();
        let mut edges = vec![];
        for after in iter {
            edges.push(after);
        }
        edges_from.insert(source, edges);
    }

    let mut completed_paths: Vec<Vec<String>> = vec![];
    let mut started_paths: VecDeque<Vec<String>> = VecDeque::new();
    started_paths.push_back(vec!["svr".to_owned()]);
    while started_paths.len() > 0 {
        let current_path = started_paths.pop_front().expect("just checked length");
        trace!("len of started: {}", started_paths.len());
        let afters = edges_from
            .get(current_path.last().unwrap())
            .expect("node not found");
        for after in afters {
            if let Some(_) = current_path.iter().find(|x| *x == after) {
                // Node already in skip
                continue;
            } else {
                let mut new_path = current_path.clone();
                new_path.push(after.clone());
                if after == "out" {
                    if new_path.contains(&"dac".to_owned()) && new_path.contains(&"fft".to_owned())
                    {
                        trace!("Path completed: {:?}", new_path);
                        completed_paths.push(new_path);
                    } else {
                        trace!("Path dismissed: {:?}", new_path);
                    }
                } else {
                    started_paths.push_back(new_path);
                }
            }
        }
    }
    debug!("Result P2: {}", completed_paths.len());
}
