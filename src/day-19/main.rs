#![feature(test)]
extern crate test;

use std::iter;

use bstr::ByteSlice;
use fnv::FnvHashMap;
use itertools::Itertools;
const INPUT: &[u8] = include_bytes!("input.txt");

#[derive(Clone, Copy, Default)]
enum Op {
    #[default]
    Gt,
    Lt,
}

#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
enum Target {
    #[default]
    Reject,
    Accept,
    Rule([u8; 3]),
}

#[derive(Default, Clone)]
struct Ruleset {
    rules: (Rule, Option<Rule>, Option<Rule>),
    default_target: Target,
}

#[derive(Default, Clone, Copy)]
struct Rule {
    attr: u8,
    threshold: u16,
    target: Target,
    op: Op,
}

#[derive(Clone, Copy)]
struct ValueRange {
    min: u16,
    max: u16,
}

impl ValueRange {
    fn new(min: u16, max: u16) -> Self {
        Self { min, max }
    }

    fn combination_count(&self) -> u64 {
        (self.max + 1 - self.min) as u64
    }

    fn split(&self, threshold: u16, op: Op) -> (ValueRange, ValueRange) {
        match op {
            Op::Gt => (
                ValueRange::new(self.min, threshold),
                ValueRange::new(threshold + 1, self.max),
            ),
            Op::Lt => (
                ValueRange::new(threshold, self.max),
                ValueRange::new(self.min, threshold - 1),
            ),
        }
    }

    fn is_empty(&self) -> bool {
        self.min > self.max
    }
}

fn pad_rule(rule: &[u8]) -> [u8; 3] {
    let mut buf = [b' '; 3];

    buf[3 - rule.len()..].copy_from_slice(rule);
    buf
}

fn parse_rules(rule_desc: &[u8]) -> FnvHashMap<[u8; 3], Ruleset> {
    rule_desc
        .lines()
        .fold(FnvHashMap::default(), |mut rules, line| {
            let line = &line[..line.len() - 1];
            let (name, instructions) = line.split_once_str("{").unwrap();
            let name = pad_rule(name.as_bytes());

            let mut rs = Ruleset::default();
            for (i, ins) in instructions.split_str(",").enumerate() {
                let Some((cond, target)) = ins.split_once_str(":") else {
                    rs.default_target = match ins {
                        b"R" => Target::Reject,
                        b"A" => Target::Accept,
                        target => Target::Rule(pad_rule(target)),
                    };
                    break;
                };

                let attr = match cond[0] {
                    b'x' => 0,
                    b'm' => 1,
                    b'a' => 2,
                    b's' => 3,
                    _ => unreachable!(),
                };

                let op = match cond[1] {
                    b'<' => Op::Lt,
                    b'>' => Op::Gt,
                    _ => unreachable!(),
                };

                let threshold = unsafe { cond[2..].to_str_unchecked() }
                    .parse::<u16>()
                    .unwrap();
                let target = match target {
                    b"R" => Target::Reject,
                    b"A" => Target::Accept,
                    _ => Target::Rule(pad_rule(target)),
                };

                let r = Rule {
                    attr,
                    op,
                    threshold,
                    target,
                };

                match i {
                    0 => rs.rules.0 = r,
                    1 => rs.rules.1 = Some(r),
                    2 => rs.rules.2 = Some(r),
                    _ => unreachable!(),
                }
            }

            rules.insert(name, rs);
            rules
        })
}

fn part_1(input: &[u8]) -> u32 {
    let (rules, part) = input.split_once_str("\n\n").unwrap();
    let parts = part.lines().map(|line| {
        let line = &line[1..line.len() - 1];
        let (x, m, a, s) = line
            .split_str(",")
            .map(|part| {
                unsafe { part[2..].to_str_unchecked() }
                    .parse::<u16>()
                    .unwrap()
            })
            .collect_tuple()
            .unwrap();

        (x, m, a, s)
    });

    let rules = parse_rules(rules);

    let mut valid = 0;
    'nextpart: for (x, m, a, s) in parts {
        let mut cursor = &rules[b" in"];
        let xmas = [x, m, a, s];

        'nextrule: loop {
            for rule in iter::once(&cursor.rules.0)
                .chain(cursor.rules.1.iter())
                .chain(cursor.rules.2.iter())
            {
                let rule_matches = match rule.op {
                    Op::Gt => xmas[rule.attr as usize] > rule.threshold,
                    Op::Lt => xmas[rule.attr as usize] < rule.threshold,
                };
                if rule_matches {
                    match rule.target {
                        Target::Reject => continue 'nextpart,
                        Target::Accept => {
                            valid += xmas.iter().sum::<u16>() as u32;
                            continue 'nextpart;
                        }
                        Target::Rule(target) => {
                            cursor = &rules[&target];
                            continue 'nextrule;
                        }
                    }
                }
            }
            match cursor.default_target {
                Target::Reject => continue 'nextpart,
                Target::Accept => {
                    valid += xmas.iter().sum::<u16>() as u32;
                    continue 'nextpart;
                }
                Target::Rule(target) => {
                    cursor = &rules[&target];
                    continue 'nextrule;
                }
            }
        }
    }

    valid
}

fn part_2(input: &[u8]) -> u64 {
    let (rules, _) = input.split_once_str("\n\n").unwrap();
    let rules = parse_rules(rules);

    let mut queue = vec![([b' ', b'i', b'n'], [
        ValueRange::new(1, 4000),
        ValueRange::new(1, 4000),
        ValueRange::new(1, 4000),
        ValueRange::new(1, 4000),
    ])];
    let mut out = 0;
    while let Some((id, bounds)) = queue.pop() {
        let cursor = &rules[&id];
        let mut bounds = bounds;

        for rule in iter::once(&cursor.rules.0)
            .chain(cursor.rules.1.iter())
            .chain(cursor.rules.2.iter())
        {
            let mut rule_match_bounds = bounds;
            (
                bounds[rule.attr as usize],
                rule_match_bounds[rule.attr as usize],
            ) = bounds[rule.attr as usize].split(rule.threshold, rule.op);

            if !rule_match_bounds[rule.attr as usize].is_empty() {
                match rule.target {
                    Target::Reject => (),
                    Target::Accept => {
                        out += rule_match_bounds
                            .iter()
                            .map(ValueRange::combination_count)
                            .product::<u64>();
                        continue;
                    }
                    Target::Rule(target) => queue.push((target, rule_match_bounds)),
                }
            }

            if bounds[rule.attr as usize].is_empty() {
                continue;
            }
        }

        match cursor.default_target {
            Target::Reject => continue,
            Target::Accept => {
                out += bounds
                    .iter()
                    .map(ValueRange::combination_count)
                    .product::<u64>();
            }
            Target::Rule(target) => {
                queue.push((target, bounds));
            }
        }
    }

    out
}

fn main() {
    println!("Part 1: {}", part_1(INPUT));
    println!("Part 2: {}", part_2(INPUT));
}

#[cfg(test)]
mod tests {
    use std::hint::black_box;

    use super::*;

    const EXAMPLE: &[u8] = b"px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(EXAMPLE), 19114);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(EXAMPLE), 167409079868000);
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
