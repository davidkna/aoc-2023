#![feature(test)]
extern crate test;

use bstr::ByteSlice;
use itertools::izip;
use itertools::Itertools;

const INPUT: &[u8] = b"Time:        56     97     78     75
Distance:   546   1927   1131   1139";

fn solve(time: u64, distance: u64) -> u64 {
    // x^2 - tx + d = 0
    let p = -(time as f64);
    let q = distance as f64 + 1.0;
    let x_1 = (-p / 2.0 - (p * p / 4.0 - q).sqrt()).ceil() as i64;
    let x_2 = (-p / 2.0 + (p * p / 4.0 - q).sqrt()).floor() as i64;

    (x_2.min(time as _) - x_1.max(0) + 1) as u64
}

fn part_1(input: &[u8]) -> u64 {
    let mut lines = input.lines();
    let time = lines
        .next()
        .unwrap()
        .strip_prefix(b"Time:")
        .unwrap()
        .trim()
        .split_str(" ")
        .filter(|s| !s.is_empty())
        .map(|s| unsafe { s.to_str_unchecked() }.parse::<u64>().unwrap());
    let distance = lines
        .next()
        .unwrap()
        .strip_prefix(b"Distance:")
        .unwrap()
        .trim()
        .split_str(" ")
        .filter(|s| !s.is_empty())
        .map(|s| unsafe { s.to_str_unchecked() }.parse::<u64>().unwrap());

    let parsed = izip!(time, distance).collect_vec();

    parsed
        .iter()
        .map(|&(time, distance)| solve(time, distance))
        .product()
}

fn part_2(input: &[u8]) -> u64 {
    let mut lines = input.lines();
    let time = {
        let joined = lines
            .next()
            .unwrap()
            .strip_prefix(b"Time:")
            .unwrap()
            .trim()
            .iter()
            .filter(|&&c| c != b' ')
            .copied()
            .collect::<Vec<u8>>();
        unsafe { joined.to_str_unchecked() }.parse::<u64>().unwrap()
    };
    let distance = {
        let joined = lines
            .next()
            .unwrap()
            .strip_prefix(b"Distance:")
            .unwrap()
            .trim()
            .iter()
            .filter(|&&c| c != b' ')
            .copied()
            .collect::<Vec<u8>>();
        unsafe { joined.to_str_unchecked() }.parse::<u64>().unwrap()
    };

    solve(time, distance)
}

fn main() {
    println!("Part 1: {}", part_1(INPUT));
    println!("Part 2: {}", part_2(INPUT));
}

#[cfg(test)]
mod tests {
    use std::hint::black_box;

    use super::*;

    const EXAMPLE: &[u8] = b"Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(EXAMPLE), 288);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(EXAMPLE), 71503);
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
