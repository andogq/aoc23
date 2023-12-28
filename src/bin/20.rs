use std::collections::{HashMap, VecDeque};

advent_of_code::solution!(20);

struct Module {
    ty: ModuleType,
    output: Pulse,
    connections: Vec<String>,
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Pulse {
    High,
    Low,
}

enum ModuleType {
    Broadcaster,
    FlipFlop { state: bool },
    Conjunction,
}

impl ModuleType {
    fn broadcaster() -> Self {
        Self::Broadcaster
    }

    fn flip_flop() -> Self {
        Self::FlipFlop { state: false }
    }

    fn conjunction() -> Self {
        Self::Conjunction
    }

    fn handle_pulse(&mut self, last_pulse: Pulse, inputs: &[Pulse]) -> Option<Pulse> {
        match self {
            ModuleType::Broadcaster => Some(last_pulse),
            ModuleType::FlipFlop { state } => {
                if matches!(last_pulse, Pulse::Low) {
                    *state = !*state;

                    Some(if *state { Pulse::High } else { Pulse::Low })
                } else {
                    None
                }
            }
            ModuleType::Conjunction => {
                Some(if inputs.iter().all(|pulse| matches!(pulse, Pulse::High)) {
                    Pulse::Low
                } else {
                    Pulse::High
                })
            }
        }
    }
}

impl TryFrom<char> for ModuleType {
    type Error = ();

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '%' => Ok(Self::flip_flop()),
            '&' => Ok(Self::conjunction()),
            _ => Err(()),
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut module_inputs: HashMap<String, Vec<String>> = HashMap::new();

    let mut modules = input
        .lines()
        .map(|line| {
            let (module, connections) = line.split_once(" -> ").unwrap();

            let (name, module_type) = if module == "broadcaster" {
                ("broadcaster".to_string(), ModuleType::broadcaster())
            } else {
                let mut module = module.chars();

                let module_c = module.next().unwrap();

                (
                    module.collect::<String>(),
                    ModuleType::try_from(module_c).unwrap(),
                )
            };

            (
                name,
                (
                    module_type,
                    connections
                        .split(", ")
                        .map(|s| s.to_string())
                        .collect::<Vec<_>>(),
                ),
            )
        })
        .inspect(|(name, (_, connections))| {
            if name != "broadcaster" {
                connections.iter().for_each(|connection| {
                    module_inputs
                        .entry(connection.to_string())
                        .or_default()
                        .push(name.to_string());
                });
            }
        })
        .map(|(name, (ty, connections))| {
            (
                name,
                Module {
                    ty,
                    output: Pulse::Low,
                    connections,
                },
            )
        })
        .collect::<HashMap<_, _>>();

    let mut input_values = module_inputs
        .iter()
        .map(|(name, input_count)| (name, vec![Pulse::Low; input_count.len()]))
        .collect::<HashMap<_, _>>();

    let mut totals = [0, 0];

    for _ in 0..1000 {
        let mut queue = VecDeque::from_iter(
            modules
                .get("broadcaster")
                .expect("broadcast module")
                .connections
                .iter()
                .cloned()
                .map(|module| {
                    let input_count = module_inputs.get(&module).into_iter().len();

                    (module, Pulse::Low, vec![Pulse::Low; input_count])
                }),
        );

        totals[Pulse::Low as usize] += 1 + queue.len() as u32;

        while let Some((module_name, last_pulse, inputs)) = queue.pop_front() {
            let Some(module) = modules.get_mut(&module_name) else {
                continue;
            };

            if let Some(output) = module.ty.handle_pulse(last_pulse, &inputs) {
                module.output = output;

                for connection in &module.connections {
                    let input_id = module_inputs
                        .get(connection)
                        .unwrap()
                        .iter()
                        .enumerate()
                        .find(|(_, input)| input == &&module_name)
                        .map(|(i, _)| i)
                        .unwrap();

                    // Update input to reflect new output value
                    input_values.get_mut(connection).unwrap()[input_id] = output;

                    // Add connection to the end of the queue
                    queue.push_back((
                        connection.to_string(),
                        output,
                        input_values.get(connection).unwrap().clone(),
                    ));

                    totals[output as usize] += 1;
                }
            }
        }
    }

    Some(totals.into_iter().product())
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut module_inputs: HashMap<String, Vec<String>> = HashMap::new();

    let mut modules = input
        .lines()
        .map(|line| {
            let (module, connections) = line.split_once(" -> ").unwrap();

            let (name, module_type) = if module == "broadcaster" {
                ("broadcaster".to_string(), ModuleType::broadcaster())
            } else {
                let mut module = module.chars();

                let module_c = module.next().unwrap();

                (
                    module.collect::<String>(),
                    ModuleType::try_from(module_c).unwrap(),
                )
            };

            (
                name,
                (
                    module_type,
                    connections
                        .split(", ")
                        .map(|s| s.to_string())
                        .collect::<Vec<_>>(),
                ),
            )
        })
        .inspect(|(name, (_, connections))| {
            if name != "broadcaster" {
                connections.iter().for_each(|connection| {
                    module_inputs
                        .entry(connection.to_string())
                        .or_default()
                        .push(name.to_string());
                });
            }
        })
        .map(|(name, (ty, connections))| {
            (
                name,
                Module {
                    ty,
                    output: Pulse::Low,
                    connections,
                },
            )
        })
        .collect::<HashMap<_, _>>();

    let mut input_values = module_inputs
        .iter()
        .map(|(name, input_count)| (name, vec![Pulse::Low; input_count.len()]))
        .collect::<HashMap<_, _>>();

    let mut iterations = 0;

    // All of target's children need to be high
    let target = modules
        .iter()
        .find(|(_, module)| module.connections.contains(&"rx".to_string()))
        .map(|(name, _)| name)
        .cloned()
        .unwrap();

    let mut high_timings = vec![0; module_inputs.get(&target).unwrap().len()];

    loop {
        iterations += 1;

        let mut queue = VecDeque::from_iter(
            modules
                .get("broadcaster")
                .expect("broadcast module")
                .connections
                .iter()
                .cloned()
                .map(|module| {
                    let input_count = module_inputs.get(&module).into_iter().len();

                    (module, Pulse::Low, vec![Pulse::Low; input_count])
                }),
        );

        while let Some((module_name, last_pulse, inputs)) = queue.pop_front() {
            let Some(module) = modules.get_mut(&module_name) else {
                continue;
            };

            if let Some(output) = module.ty.handle_pulse(last_pulse, &inputs) {
                module.output = output;

                for connection in &module.connections {
                    let input_id = module_inputs
                        .get(connection)
                        .unwrap()
                        .iter()
                        .enumerate()
                        .find(|(_, input)| input == &&module_name)
                        .map(|(i, _)| i)
                        .unwrap();

                    if connection == &target
                        && matches!(output, Pulse::High)
                        && high_timings[input_id] == 0
                    {
                        high_timings[input_id] = iterations;
                    }

                    if high_timings.iter().all(|&timing| timing > 0) {
                        // Find the LCM of all the timings
                        return Some(high_timings.into_iter().fold(1u64, num::integer::lcm));
                    }

                    // Update input to reflect new output value
                    input_values.get_mut(connection).unwrap()[input_id] = output;

                    // Add connection to the end of the queue
                    queue.push_back((
                        connection.to_string(),
                        output,
                        input_values.get(connection).unwrap().clone(),
                    ));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11687500));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
