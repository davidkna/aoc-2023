#![feature(test)]
extern crate test;

use bstr::ByteSlice;
use itertools::Itertools;

const INPUT: &[u8] = include_bytes!("input.txt");

fn part_1(input: &[u8]) -> usize {
    solve(input, 2)
}

fn part_2(input: &[u8]) -> usize {
    solve(input, 1_000_000)
}

fn solve(input: &[u8], expansion_factor: usize) -> usize {
    let stars = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| memchr::memchr_iter(b'#', line).map(move |x| (x, y)))
        .collect_vec();

    let stars_ys = stars.iter().map(|(_, y)| *y).sorted().collect_vec();
    let stars_xs = stars.iter().map(|(x, _)| *x).sorted().collect_vec();

    stars
        .into_iter()
        .tuple_combinations()
        .map(|(star1, star2)| {
            let (x1, y1) = star1;
            let (x2, y2) = star2;

            let x1_idx = stars_xs.binary_search(&x1.min(x2)).unwrap();
            let x2_idx = stars_xs.binary_search(&x1.max(x2)).unwrap();

            let distance_x = stars_xs[x1_idx..=x2_idx]
                .iter()
                .tuple_windows()
                .map(|(x1, x2)| match x2 - x1 {
                    0 => 0,
                    1 => 1,
                    n => 1 + expansion_factor * (n - 1),
                })
                .sum::<usize>();

            let y1_idx = stars_ys.binary_search(&y1.min(y2)).unwrap();
            let y2_idx = stars_ys.binary_search(&y1.max(y2)).unwrap();

            let distance_y = stars_ys[y1_idx..=y2_idx]
                .iter()
                .tuple_windows()
                .map(|(y1, y2)| match y2 - y1 {
                    0 => 0,
                    1 => 1,
                    n => 1 + expansion_factor * (n - 1),
                })
                .sum::<usize>();

            distance_x + distance_y
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
