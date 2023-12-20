#![deny(clippy::pedantic)]
#![allow(clippy::must_use_candidate)]

use std::collections::{HashMap, VecDeque};

use lazy_static::lazy_static;

use num::integer::lcm;

lazy_static! {
    pub static ref INPUT: &'static str = include_str!("../../input");
}

#[derive(Debug, Clone)]
enum Module {
    Broadcast,
    Conjunction(Vec<bool>),
    FlipFlop(bool),
}

/// Parse input
///
/// # Panics
/// Panic if invalid input
fn parse(input: &str) -> HashMap<&str, (Module, Vec<&str>, Vec<&str>)> {
    let mut modules = input
        .lines()
        .map(|line| {
            let (module, outputs) = line.split_once(" -> ").expect("invalid module definition");

            let outputs = outputs.split(", ").collect::<Vec<_>>();

            let mut module_chars = module.chars();
            let (module_type, module) = match module_chars.next() {
                Some('%') => ('%', module_chars.as_str()),
                Some('&') => ('&', module_chars.as_str()),
                Some(_) => (' ', module),
                None => panic!("invalid module"),
            };

            (module, (module_type, vec![], outputs))
        })
        .collect::<HashMap<_, _>>();

    let mo = modules
        .iter()
        .map(|(&module, (_, _, outputs))| (module, outputs.clone()))
        .collect::<Vec<(&str, Vec<&str>)>>();
    for (module, outputs) in mo {
        for output in outputs {
            if let Some(target_module) = modules.get_mut(output) {
                target_module.1.push(module);
            }
        }
    }

    modules
        .drain()
        .map(|(module, (module_type, inputs, outputs))| {
            let module_state = match module_type {
                '%' => Module::FlipFlop(false),
                '&' => Module::Conjunction(vec![false; inputs.len()]),
                ' ' => Module::Broadcast,
                _ => unreachable!(),
            };

            (module, (module_state, inputs, outputs))
        })
        .collect::<HashMap<_, _>>()
}

/// Solve part 1
///
/// # Panics
/// Panic if invalid input
pub fn solve_1(input: &str) -> u64 {
    let mut modules = parse(input);

    let mut high = 0;
    let mut low = 0;
    for _ in 0..1000 {
        let mut pulses = VecDeque::new();

        pulses.push_back(("button", false, "broadcaster"));

        while let Some((from, level, module)) = pulses.pop_front() {
            if level {
                high += 1;
            } else {
                low += 1;
            }

            if let Some((module_state, inputs, outputs)) = modules.get_mut(module) {
                match module_state {
                    Module::Broadcast => {
                        for output in outputs {
                            pulses.push_back((module, level, output));
                        }
                    }
                    Module::Conjunction(state) => {
                        state[inputs.iter().position(|v| v == &from).unwrap()] = level;
                        let level = !state.iter().all(|&v| v);
                        for output in outputs {
                            pulses.push_back((module, level, output));
                        }
                    }
                    Module::FlipFlop(state) => {
                        if !level {
                            *state = !*state;
                            let level = *state;
                            for output in outputs {
                                pulses.push_back((module, level, output));
                            }
                        }
                    }
                }
            }
        }
    }

    high * low
}

/// Solve part 2
///
/// # Panics
/// Panic if invalid input
pub fn solve_2(input: &str) -> u64 {
    let mut modules = parse(input);

    let rx_in_module = modules
        .iter()
        .find_map(|(&module, (_, _, outputs))| {
            if outputs.contains(&"rx") {
                Some(module)
            } else {
                None
            }
        })
        .expect("cannot find rx in module");

    let rx_modules = modules
        .iter()
        .filter_map(|(&module, (_, _, outputs))| {
            if outputs.contains(&rx_in_module) {
                Some(module)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let mut rx_modules_data = HashMap::with_capacity(4);

    for i in 1.. {
        let mut pulses = VecDeque::new();

        pulses.push_back(("button", false, "broadcaster"));

        while let Some((from, level, module)) = pulses.pop_front() {
            if !level && rx_modules.contains(&module) {
                rx_modules_data.insert(module, i);
                if rx_modules_data.len() == rx_modules.len() {
                    return rx_modules_data
                        .values()
                        .fold(1, |acc, value| lcm(acc, *value));
                }
            }

            if let Some((module_state, inputs, outputs)) = modules.get_mut(module) {
                match module_state {
                    Module::Broadcast => {
                        for output in outputs {
                            pulses.push_back((module, level, output));
                        }
                    }
                    Module::Conjunction(state) => {
                        state[inputs.iter().position(|v| v == &from).unwrap()] = level;
                        let level = !state.iter().all(|&v| v);
                        for output in outputs {
                            pulses.push_back((module, level, output));
                        }
                    }
                    Module::FlipFlop(state) => {
                        if !level {
                            *state = !*state;
                            let level = *state;
                            for output in outputs {
                                pulses.push_back((module, level, output));
                            }
                        }
                    }
                }
            }
        }
    }

    unreachable!()
}

pub fn part_1() -> u64 {
    solve_1(&INPUT)
}

pub fn part_2() -> u64 {
    solve_2(&INPUT)
}

#[cfg(test)]
mod tests {
    use super::*;

    lazy_static! {
        static ref EXAMPLE_1: &'static str = include_str!("../../example1");
        static ref EXAMPLE_2: &'static str = include_str!("../../example2");
    }

    #[test]
    fn same_results_1_1() {
        assert_eq!(solve_1(&EXAMPLE_1), 32000000);
    }

    #[test]
    fn same_results_1_2() {
        assert_eq!(solve_1(&EXAMPLE_2), 11687500);
    }

    #[test]
    fn same_results_2() {
        assert_eq!(solve_2(&INPUT), 217317393039529);
    }
}
