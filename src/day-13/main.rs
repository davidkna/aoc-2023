#![feature(test)]
extern crate test;

use bstr::ByteSlice;
use itertools::{izip, Itertools};

const INPUT: &[u8] = include_bytes!("input.txt");

fn find_palindrome(input: &[u32], should_smudge: bool) -> usize {
    (1..input.len())
        .find(|&center| {
            let (l, r) = input.split_at(center);
            let mut smudged = false;
            izip!(l.iter().rev(), r.iter()).all(|(l, r)| match l == r {
                true => true,
                false if should_smudge && !smudged => {
                    let can_smudge = (l ^ r).count_ones() == 1;
                    if can_smudge {
                        smudged = true;
                        true
                    } else {
                        false
                    }
                }
                _ => false,
            }) && (!should_smudge || smudged)
        })
        .unwrap_or(0)
}

#[inline]
fn solve(input: &[u8], should_smudge: bool) -> usize {
    let maps = input.split_str("\n\n");
    maps.map(|map| {
        let cols_simplified = {
            let mut storage = vec![0; map.lines().next().unwrap().len()];
            for (i, row) in map.lines().enumerate() {
                for (j, c) in row.iter().enumerate() {
                    storage[j] |= u32::from(*c == b'#') << i;
                }
            }
            storage
        };

        let rows_simplified = map
            .lines()
            .map(|line| {
                line.iter()
                    .enumerate()
                    .fold(0, |acc, (i, c)| acc | (u32::from(*c == b'#') << i))
            })
            .collect_vec();

        find_palindrome(&cols_simplified, should_smudge)
            + find_palindrome(&rows_simplified, should_smudge) * 100
    })
    .sum::<usize>()
}

fn part_1(input: &[u8]) -> usize {
    solve(input, false)
}

fn part_2(input: &[u8]) -> usize {
    solve(input, true)
}

fn main() {
    println!("Part 1: {}", part_1(INPUT));
    println!("Part 2: {}", part_2(INPUT));
}

#[cfg(test)]
mod tests {
    use std::hint::black_box;

    use super::*;

    const EXAMPLE: &[u8] = b"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(EXAMPLE), 405);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(EXAMPLE), 400);
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
