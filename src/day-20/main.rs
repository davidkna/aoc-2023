#![feature(test)]
extern crate test;

use std::collections::VecDeque;

use bstr::ByteSlice;
use fnv::FnvHashMap;
use itertools::Itertools;

const INPUT: &[u8] = include_bytes!("input.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pulse {
    High,
    Low,
}

#[derive(Debug)]
enum ModuleKind {
    FlipFlop { off: bool },
    Conjunction { last_pulses: Vec<Pulse> },
}

#[derive(Debug)]
struct Module<'a> {
    kind: ModuleKind,
    outputs: Vec<&'a [u8]>,
    inputs: Vec<&'a [u8]>,
}

fn parse_input(input: &[u8]) -> (Vec<&[u8]>, FnvHashMap<&[u8], Module>) {
    let broadcaster_targets = input
        .lines()
        .find_map(|line| {
            line.strip_prefix(b"broadcaster -> ")
                .map(|targets| targets.split_str(", ").collect_vec())
        })
        .unwrap();

    let mut rules = input
        .lines()
        .filter(|line| !line.starts_with(b"broadcaster"))
        .map(|line| {
            let (name, outputs) = line[1..].split_once_str(" -> ").unwrap();
            let outputs = outputs.split_str(", ").collect_vec();

            let kind = match line[0] {
                b'%' => ModuleKind::FlipFlop { off: true },
                b'&' => ModuleKind::Conjunction {
                    last_pulses: vec![],
                },
                _ => unreachable!("invalid module kind"),
            };

            let module = Module {
                kind,
                outputs,
                inputs: vec![],
            };
            (name, module)
        })
        .collect::<FnvHashMap<_, _>>();

    let input_to_outputs = rules
        .iter()
        .map(|(name, module)| (*name, module.outputs.clone()))
        .collect_vec();

    for (name, outputs) in input_to_outputs {
        outputs
            .iter()
            .filter(|s| s != &b"output")
            .for_each(|output| {
                let Some(module) = rules.get_mut(output) else {
                    return;
                };
                module.inputs.push(name);
            });
    }

    for target in &broadcaster_targets {
        let module = rules.get_mut(target).unwrap();
        module.inputs.push(b"broadcaster");
    }

    for (_name, module) in &mut rules {
        if let ModuleKind::Conjunction { last_pulses } = &mut module.kind {
            last_pulses.resize(module.inputs.len(), Pulse::Low);
        }
    }
    (broadcaster_targets, rules)
}

fn perform_tick<'a>(
    queue: &mut VecDeque<(&'a [u8], &'a [u8], Pulse)>,
    rules: &mut FnvHashMap<&'a [u8], Module<'a>>,
    name: &'a [u8],
    parent_name: &'a [u8],
    pulse: Pulse,
) {
    let Some(module) = rules.get_mut(name) else {
        return;
    };
    let pulse_type = match &mut module.kind {
        ModuleKind::FlipFlop { off } => {
            if pulse == Pulse::High {
                return;
            }
            let pulse_type = if *off { Pulse::High } else { Pulse::Low };
            *off = !*off;
            pulse_type
        }
        ModuleKind::Conjunction { last_pulses } => {
            let input_idx = module
                .inputs
                .iter()
                .position(|input| input == &parent_name)
                .unwrap();
            last_pulses[input_idx] = pulse;

            let all_high = last_pulses.iter().all(|pulse| *pulse == Pulse::High);
            if all_high {
                Pulse::Low
            } else {
                Pulse::High
            }
        }
    };
    for output in &module.outputs {
        queue.push_back((output, name, pulse_type));
    }
}

fn part_1(input: &[u8]) -> u32 {
    let (broadcaster_targets, mut rules) = parse_input(input);
    let (mut output_low, mut output_high) = (1000, 0);
    let mut queue = VecDeque::new();

    for _ in 0..1000 {
        for target in &broadcaster_targets {
            queue.push_back((*target, b"broadcaster".as_slice(), Pulse::Low));
        }

        while let Some((name, parent_name, pulse)) = queue.pop_front() {
            match pulse {
                Pulse::High => output_high += 1,
                Pulse::Low => output_low += 1,
            };
            perform_tick(&mut queue, &mut rules, name, parent_name, pulse);
        }
    }

    output_low * output_high
}

fn greatest_common_divisor(a: u64, b: u64) -> u64 {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

fn lowest_common_multiple(a: u64, b: u64) -> u64 {
    a * b / greatest_common_divisor(a, b)
}

fn part_2(input: &[u8]) -> u64 {
    let (broadcaster_targets, mut rules) = parse_input(input);
    let rx_parent = rules
        .iter()
        .find_map(|(name, module)| module.outputs.contains(&&b"rx"[..]).then_some(*name))
        .unwrap();

    let rx_parent_inputs = rules[rx_parent].inputs.clone();

    let mut loop_idx = rules[rx_parent].inputs.iter().map(|_| None).collect_vec();

    let mut it = 1;
    let mut queue = VecDeque::new();
    while loop_idx.iter().any(Option::is_none) {
        for target in &broadcaster_targets {
            queue.push_back((*target, b"broadcaster".as_slice(), Pulse::Low));
        }

        while let Some((name, parent_name, pulse)) = queue.pop_front() {
            perform_tick(&mut queue, &mut rules, name, parent_name, pulse);
            if name == rx_parent {
                let idx = rx_parent_inputs
                    .iter()
                    .position(|input| input == &parent_name)
                    .unwrap();

                if loop_idx[idx].is_none() && pulse == Pulse::High {
                    loop_idx[idx] = Some(it);
                }
            }
        }

        it += 1;
    }

    loop_idx
        .into_iter()
        .map(Option::unwrap)
        .reduce(lowest_common_multiple)
        .unwrap()
}

fn main() {
    println!("Part 1: {}", part_1(INPUT));
    println!("Part 2: {}", part_2(INPUT));
}

#[cfg(test)]
mod tests {
    use std::hint::black_box;

    use super::*;

    const EXAMPLE_1: &[u8] = b"broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";

    const EXAMPLE_2: &[u8] = b"broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(EXAMPLE_1), 32_000_000);
        assert_eq!(part_1(EXAMPLE_2), 11_687_500);
    }

    #[bench]
    fn bench_part_1(b: &mut test::Bencher) {
        b.iter(|| part_1(black_box(INPUT)));
    }

    #[bench]
    fn bench_part_2(b: &mut test::Bencher) {
        b.iter(|| part_2(black_box(INPUT)));
    }
}