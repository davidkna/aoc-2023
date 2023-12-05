#![feature(test)]
extern crate test;

use aho_corasick::{AhoCorasick, PatternID};
use bstr::ByteSlice;

const INPUT: &[u8] = include_bytes!("input.txt");

fn part_1(input: &[u8]) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut first = 0;
            let mut last = 0;

            for c in line {
                if c.is_ascii_digit() {
                    first = u32::from(c - b'0');
                    break;
                }
            }

            for c in line.iter().rev() {
                if c.is_ascii_digit() {
                    last = u32::from(c - b'0');
                    break;
                }
            }

            first * 10 + last
        })
        .sum()
}

fn part_2(input: &[u8]) -> u32 {
    let patterns = [
        "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six",
        "seven", "eight", "nine",
    ];

    let ac = AhoCorasick::new(patterns).unwrap();

    fn pattern_id_to_num(id: PatternID) -> u32 {
        let id_u = id.as_u32();
        match id_u {
            0..=9 => id_u + 1,
            10..=19 => id_u - 9,
            _ => unreachable!(),
        }
    }

    input
        .lines()
        .map(|line| {
            let mut matcher = ac.find_overlapping_iter(line);
            let first = matcher.next().unwrap();
            let last = matcher.last().unwrap_or(first);

            pattern_id_to_num(first.pattern()) * 10 + pattern_id_to_num(last.pattern())
        })
        .sum()
}

fn main() {
    println!("Part 1: {}", part_1(INPUT));
    println!("Part 2: {}", part_2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_01: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    const EXAMPLE_02: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(EXAMPLE_01.as_bytes()), 142);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(EXAMPLE_02.as_bytes()), 281);
    }

    #[bench]
    fn bench_part_1(b: &mut test::Bencher) {
        b.iter(|| part_1(INPUT));
    }

    #[bench]
    fn bench_part_2(b: &mut test::Bencher) {
        b.iter(|| part_2(INPUT));
    }
}
