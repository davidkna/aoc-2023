#![feature(array_windows)]
#![feature(test)]
extern crate test;

use bstr::ByteSlice;
use itertools::Itertools;

const INPUT: &[u8] = include_bytes!("input.txt");

fn part_1(input: &[u8]) -> i64 {
    input
        .lines()
        .map(|line| {
            let mut observations = line
                .split_str(" ")
                .map(|s| unsafe { s.to_str_unchecked() }.parse::<i64>().unwrap())
                .collect_vec();
            let mut result = 0;
            for i in 1..observations.len() {
                result += observations.last().unwrap();
                for j in (i..observations.len()).rev() {
                    observations[j] -= observations[j - 1];
                }
                if observations[i..].iter().all(|&n| n == 0) {
                    break;
                }
            }
            result
        })
        .sum()
}

fn part_2(input: &[u8]) -> i64 {
    input
        .lines()
        .map(|line| {
            let _result = 0;
            let mut is_even = true;

            let mut observations = line
                .split_str(" ")
                .map(|s| unsafe { s.to_str_unchecked() }.parse::<i64>().unwrap())
                .collect_vec();
            let mut result = 0;
            for i in 1..observations.len() {
                if is_even {
                    result += observations[i - 1];
                } else {
                    result -= observations[i - 1];
                }
                is_even = !is_even;
                for j in (i..observations.len()).rev() {
                    observations[j] -= observations[j - 1];
                }
                if observations[i..].iter().all(|&n| n == 0) {
                    break;
                }
            }
            result
        })
        .sum()
}

fn main() {
    println!("Part 1: {}", part_1(INPUT));
    println!("Part 2: {}", part_2(INPUT));
}

#[cfg(test)]
mod tests {
    use std::hint::black_box;

    use super::*;

    const EXAMPLE: &[u8] = b"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(EXAMPLE), 114);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(EXAMPLE), 2);
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
