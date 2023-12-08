#![feature(test)]
extern crate test;

use bstr::ByteSlice;
use fnv::FnvHashMap;

const INPUT: &[u8] = include_bytes!("input.txt");

fn greatest_common_divisor(a: u64, b: u64) -> u64 {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn lowest_common_multiple(a: u64, b: u64) -> u64 {
    a * b / greatest_common_divisor(a, b)
}

fn part_1(input: &[u8]) -> u64 {
    let (instructions, map) = input.split_once_str("\n\n").unwrap();
    let map = map
        .lines()
        .map(|line| {
            let key = TryInto::<[u8; 3]>::try_into(&line[..3]).unwrap();
            let left = TryInto::<[u8; 3]>::try_into(&line[7..10]).unwrap();
            let right = TryInto::<[u8; 3]>::try_into(&line[12..15]).unwrap();
            (key, (left, right))
        })
        .collect::<FnvHashMap<_, _>>();

    let mut cursor = [b'A'; 3];

    for (i, instruction) in instructions.iter().cycle().enumerate() {
        let current_item = map[&cursor];
        match instruction {
            b'L' => cursor = current_item.0,
            b'R' => cursor = current_item.1,
            _ => unreachable!(),
        }
        if cursor == [b'Z'; 3] {
            return 1 + i as u64;
        }
    }
    unreachable!()
}

fn part_2(input: &[u8]) -> u64 {
    let (instructions, map) = input.split_once_str("\n\n").unwrap();
    let map = map
        .lines()
        .map(|line| {
            let key = TryInto::<[u8; 3]>::try_into(&line[..3]).unwrap();
            let left = TryInto::<[u8; 3]>::try_into(&line[7..10]).unwrap();
            let right = TryInto::<[u8; 3]>::try_into(&line[12..15]).unwrap();
            (key, (left, right))
        })
        .collect::<FnvHashMap<_, _>>();

    let cursors = map
        .keys()
        .filter(|key| matches!(key, [_, _, b'A']))
        .copied();
    cursors
        .map(|start| {
            let mut cursor = start;
            for (i, instruction) in instructions.iter().cycle().enumerate() {
                let current_item = map[&cursor];
                match instruction {
                    b'L' => cursor = current_item.0,
                    b'R' => cursor = current_item.1,
                    _ => unreachable!(),
                }
                if matches!(cursor, [_, _, b'Z']) {
                    return 1 + i as u64;
                }
            }
            unreachable!()
        })
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

    const EXAMPLE_01A: &[u8] = b"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

    const EXAMPLE_01B: &[u8] = b"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    const EXAMPLE_02: &[u8] = b"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(EXAMPLE_01A), 2);
        assert_eq!(part_1(EXAMPLE_01B), 6);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(EXAMPLE_02), 6);
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
