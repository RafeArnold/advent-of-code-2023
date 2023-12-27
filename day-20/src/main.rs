// You could definitely solve this problem using bitwise operations.

use crate::ModuleKind::{Broadcast, Conjunction, FlipFlop};
use std::collections::HashMap;

fn main() {
    const INPUT: &str = include_str!("../input.txt");
    println!("{}", run_1(INPUT));
}

fn run_1(input: &str) -> usize {
    let mut modules_config = parse_input(input);
    let mut low_signals_sent = 0;
    let mut high_signals_sent = 0;
    for _ in 0..1000 {
        low_signals_sent += 1;
        let mut modules = HashMap::from([("button", (vec!["broadcaster"], false))]);
        while !modules.is_empty() {
            let mut next_modules = HashMap::new();
            for (sender, (destinations, pulse)) in modules {
                for destination in destinations {
                    if let Some((next_destinations, next_pulse)) =
                        modules_config.get_mut(destination).and_then(|module| {
                            module
                                .receive(sender, pulse)
                                .map(|next_pulse| (module.destinations.clone(), next_pulse))
                        })
                    {
                        if next_pulse {
                            high_signals_sent += next_destinations.len();
                        } else {
                            low_signals_sent += next_destinations.len();
                        }
                        next_modules.insert(destination, (next_destinations, next_pulse));
                    }
                }
            }
            modules = next_modules;
        }
    }
    high_signals_sent * low_signals_sent
}

fn parse_input(input: &str) -> HashMap<&str, Module> {
    let mut destinations_to_inputs = HashMap::<&str, Vec<&str>>::new();
    let mut modules = input
        .lines()
        .map(|line| {
            let (name, destinations) = line.split_once(" -> ").unwrap();
            let (name, kind) = match name.as_bytes()[0] {
                b'%' => (&name[1..], ModuleKind::flip_flop()),
                b'&' => (&name[1..], ModuleKind::conjunction()),
                _ => (name, ModuleKind::broadcast()),
            };
            let destinations = destinations.split(", ").collect();
            for &destination in &destinations {
                destinations_to_inputs
                    .entry(destination)
                    .or_default()
                    .push(name);
            }
            (name, Module { kind, destinations })
        })
        .collect::<HashMap<_, _>>();
    for (destination, inputs) in destinations_to_inputs {
        if let Some(module) = modules.get_mut(destination) {
            if let Conjunction { memory } = &mut module.kind {
                memory.extend(inputs.into_iter().map(|input| (input, false)));
            }
        }
    }
    modules
}

struct Module<'a> {
    kind: ModuleKind<'a>,
    destinations: Vec<&'a str>,
}

impl<'a> Module<'a> {
    fn receive(&mut self, sender: &'a str, pulse: bool) -> Option<bool> {
        match &mut self.kind {
            Broadcast => Some(pulse),
            FlipFlop { on } => {
                if pulse {
                    None
                } else {
                    *on = !*on;
                    Some(*on)
                }
            }
            Conjunction { memory } => {
                memory.insert(sender, pulse);
                Some(!(pulse && memory.values().all(|pulse| *pulse)))
            }
        }
    }
}

enum ModuleKind<'a> {
    Broadcast,
    FlipFlop { on: bool },
    Conjunction { memory: HashMap<&'a str, bool> },
}

impl ModuleKind<'_> {
    fn broadcast() -> Self {
        Broadcast
    }
    fn flip_flop() -> Self {
        FlipFlop { on: false }
    }
    fn conjunction() -> Self {
        Conjunction {
            memory: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_1: &str = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";

    const INPUT_2: &str = "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";

    #[test]
    fn challenge_1() {
        assert_eq!(run_1(INPUT_1), 32000000);
        assert_eq!(run_1(INPUT_2), 11687500);
    }
}
