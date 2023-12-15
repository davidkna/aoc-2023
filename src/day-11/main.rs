#![feature(cmp_minmax)]
#![feature(test)]
extern crate test;

use bstr::ByteSlice;
use itertools::{izip, Itertools};

const INPUT: &[u8] = include_bytes!("input.txt");

fn part_1(input: &[u8]) -> usize {
    solve(input, 2)
}

fn part_2(input: &[u8]) -> usize {
    solve(input, 1_000_000)
}

fn solve_dimension(
    it: impl Iterator<Item = usize>,
    expansion_factor: usize,
    stars_len: usize,
) -> usize {
    it.sorted()
        .tuple_windows()
        .zip(izip!((0..stars_len).rev(), 0..))
        .scan(0, |state, ((curr, next), (to_add, to_remove))| {
            *state = *state + to_add - to_remove;
            Some(match curr - next {
                0 => 0,
                1 => *state,
                n => *state * (1 + expansion_factor * (n - 1)),
            })
        })
        .sum::<usize>()
}

fn solve(input: &[u8], expansion_factor: usize) -> usize {
    let stars = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| memchr::memchr_iter(b'#', line).map(move |x| (x, y)))
        .collect_vec();

    let stars_len = stars.len();

    solve_dimension(stars.iter().map(|(x, _)| *x), expansion_factor, stars_len)
        + solve_dimension(stars.iter().map(|(_, y)| *y), expansion_factor, stars_len)
}
fn main() {
    println!("Part 1: {}", part_1(INPUT));
    println!("Part 2: {}", part_2(INPUT));
}

#[cfg(test)]
mod tests {
    use std::hint::black_box;

    use super::*;

    const EXAMPLE: &[u8] = b"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(EXAMPLE), 374);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(solve(EXAMPLE, 10), 1030);
        assert_eq!(solve(EXAMPLE, 100), 8410);
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
