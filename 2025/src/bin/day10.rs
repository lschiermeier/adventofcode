use std::{collections::BTreeMap, ops::Not};

use aoc_2025::*;
use itertools::Itertools;
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
    let mut machines = starting_table
        .iter()
        .map(|x| Machine::new_from(x).unwrap())
        .collect_vec();

    // for (i,machine) in machines.iter().enumerate() {
    //     debug!("Machine {}: {:?}",i , machine);
    // }

    for machine in machines.iter_mut() {
        let mut sequence_pool: BTreeMap<Lights, Buttons> = BTreeMap::new();
        sequence_pool.insert(vec![false; machine.target_lights.len()], vec![]);
        let mut cycle_count = 0;
        while sequence_pool.keys().all(|x| *x != machine.target_lights) {
            let mut additional_sequences = BTreeMap::new();
            for sequence in sequence_pool
                .iter()
                .filter(|(cur_lights, prev_buttons)| prev_buttons.len() == cycle_count)
            {
                for button in &machine.buttons {
                    let new_lights = press_button_for_lights(sequence.0, button);
                    if !sequence_pool.contains_key(&new_lights) {
                        let mut new_buttons = sequence.1.clone();
                        new_buttons.push(button.clone());
                        additional_sequences.insert(new_lights, new_buttons);
                    }
                }
            }
            trace!("sequence_pool {sequence_pool:?}");
            additional_sequences.into_iter().for_each(|(key, val)| {
                sequence_pool.insert(key, val);
            });
            cycle_count += 1;
        }
        machine.quickest_light_sequence = sequence_pool.get(&machine.target_lights).cloned();
        debug!("Found quickest sequence for {:?}", machine.target_lights);
        debug!("{:?}", machine.quickest_light_sequence);
    }
    let result_p1: usize = machines
        .iter()
        .map(|x| x.quickest_light_sequence.as_ref().unwrap().len())
        .sum();
    info!("Result part 1: {result_p1}");

    for machine in machines.iter_mut() {
        machine.joltage_cycle_count = depth_search(
            vec![0; machine.joltage_requirements.len()],
            &machine.joltage_requirements,
            &machine.buttons,
            None,
            0,
        );
        debug!(
            "found joltage cycle count {} for {:?}",
            machine.joltage_cycle_count.unwrap(),
            machine.joltage_requirements
        );
    }
    let result_p1: i64 = machines
        .iter()
        .map(|x| x.joltage_cycle_count.unwrap())
        .sum();
    info!("Result part 2: {result_p1}");
}

#[derive(Debug)]
#[allow(dead_code)]
struct Machine {
    target_lights: Lights,
    buttons: Buttons,
    joltage_requirements: Joltages,
    quickest_light_sequence: Option<Buttons>,
    joltage_cycle_count: Option<i64>,
}

#[allow(dead_code)]
impl Machine {
    pub fn new_from(machine_strings: &Vec<String>) -> Option<Self> {
        let mut string_iter = machine_strings.iter();
        let target_lights = string_iter
            .next()?
            .chars()
            .skip(1)
            .map_while(|x| match x {
                '[' | ']' => None,
                '#' => Some(true),
                '.' => Some(false),
                _ => None,
            })
            .collect_vec();
        let mut buttons = vec![];
        while string_iter.len() > 1 {
            buttons.push(
                string_iter
                    .next()?
                    .split(&[',', '(', ')'])
                    .filter_map(|x| x.parse::<usize>().ok())
                    .collect_vec(),
            )
        }
        buttons.sort_by_key(|x| 10 - x.len());

        let joltage_requirements = string_iter
            .next()?
            .split(&[',', '{', '}'])
            .filter_map(|x| x.parse::<i64>().ok())
            .collect_vec();

        Some(Machine {
            target_lights,
            buttons,
            joltage_requirements,
            quickest_light_sequence: None,
            joltage_cycle_count: None,
        })
    }
}

type Button = Vec<usize>;
type Buttons = Vec<Button>;
type Lights = Vec<bool>;
type Joltages = Vec<i64>;

pub fn press_button_for_lights(lights: &Lights, button: &Button) -> Lights {
    lights
        .iter()
        .enumerate()
        .map(|(idx, light)| {
            if button.contains(&idx) {
                light.not()
            } else {
                *light
            }
        })
        .collect()
}

pub fn press_button_for_joltages(joltages: &Joltages, button: &Button) -> Joltages {
    joltages
        .iter()
        .enumerate()
        .map(|(idx, joltage)| {
            if button.contains(&idx) {
                joltage + 1
            } else {
                *joltage
            }
        })
        .collect()
}

pub fn depth_search(
    input_joltages: Joltages,
    target_joltages: &Joltages,
    buttons: &Buttons,
    current_best: Option<i64>,
    current_depth: i64,
) -> Option<i64> {
    if let Some(best) = current_best {
        if best <= current_depth {
            trace!(
                "Stopping depth search as better was found. At Depth {}",
                current_depth
            );
            return None;
        }
    }
    let mut maybe_best = current_best;
    for button in buttons {
        let new_joltages = press_button_for_joltages(&input_joltages, &button);
        if new_joltages == *target_joltages {
            maybe_best = Some(1);
            trace!(
                "found {target_joltages:?}, current_depth {current_depth}, best_depth: {current_best:?}"
            );
        } else if target_joltages
            .iter()
            .zip_eq(&new_joltages)
            .map(|(t, n)| t - n)
            .any(|x| x < 0)
        {
            continue;
        } else {
            trace!(
                "searching deeper for {target_joltages:?} from {new_joltages:?}, current_depth {current_depth}, best_depth: {current_best:?}"
            );
            maybe_best = match (
                maybe_best,
                depth_search(
                    new_joltages,
                    target_joltages,
                    buttons,
                    maybe_best,
                    current_depth + 1,
                ),
            ) {
                (None, None) => None,
                (Some(cur), None) => Some(cur),
                (None, Some(new)) => Some(new + 1),
                (Some(cur), Some(new)) if new < cur => Some(new + 1),
                (Some(cur), _) => Some(cur),
            };
        }
    }
    match (maybe_best, current_best) {
        (Some(maybe), Some(current)) if maybe < current => maybe_best,
        (_, None) => maybe_best,
        (_, _) => None,
    }
}
