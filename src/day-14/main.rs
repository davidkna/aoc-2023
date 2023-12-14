#![feature(test)]
extern crate test;

use std::hash::{BuildHasher, BuildHasherDefault, Hash, Hasher};

use bstr::ByteSlice;
use itertools::{izip, Itertools};

const INPUT: &[u8] = include_bytes!("input.txt");

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Rock {
    Round,
    Cube,
}

fn part_1(input: &[u8]) -> usize {
    let rows = input.lines().count();
    let mut map = vec![vec![None; rows]; rows];

    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.iter().enumerate() {
            map[j][i] = match c {
                b'.' => None,
                b'O' => Some(Rock::Round),
                b'#' => Some(Rock::Cube),
                _ => unreachable!(),
            };
        }
    }

    map.into_iter()
        .map(|col| {
            izip!(col.into_iter(), (1..=rows).rev())
                .filter_map(|(c, i)| Some((c?, i)))
                .scan(rows + 1, |last_wall_pos, (rock, i)| match rock {
                    Rock::Round => {
                        *last_wall_pos -= 1;
                        Some(*last_wall_pos)
                    }
                    Rock::Cube => {
                        *last_wall_pos = i;
                        Some(0)
                    }
                })
                .sum::<usize>()
        })
        .sum()
}

fn turn(map: &mut [Vec<Option<Rock>>]) {
    let n = map.len();
    for i in 0..n / 2 {
        for j in i..n - i - 1 {
            let temp = map[i][j];
            map[i][j] = map[n - j - 1][i];
            map[n - j - 1][i] = map[n - i - 1][n - j - 1];
            map[n - i - 1][n - j - 1] = map[j][n - i - 1];
            map[j][n - i - 1] = temp;
        }
    }

    for row in map.iter_mut() {
        let mut last_wall_pos = n;
        for x in (0..n).rev() {
            match row[x] {
                None => (),
                Some(Rock::Round) => {
                    last_wall_pos -= 1;
                    row[x] = None;
                    row[last_wall_pos] = Some(Rock::Round);
                }
                Some(Rock::Cube) => {
                    last_wall_pos = x;
                }
            }
        }
    }
}

fn cycle(map: &mut [Vec<Option<Rock>>]) {
    for _ in 0..4 {
        turn(map);
    }
}

fn get_load_p2(map: &[Vec<Option<Rock>>]) -> usize {
    izip!(map.iter(), (1..=map.len()).rev())
        .map(|(col, value)| {
            value
                * col
                    .iter()
                    .filter(|c| matches!(c, Some(Rock::Round)))
                    .count()
        })
        .sum()
}

fn part_2(input: &[u8]) -> usize {
    let mut cache: Vec<(u64, Vec<Vec<Option<Rock>>>)> = Vec::new();
    let mut map = input
        .lines()
        .map(|line| {
            line.iter()
                .map(|c| match c {
                    b'.' => None,
                    b'O' => Some(Rock::Round),
                    b'#' => Some(Rock::Cube),
                    _ => unreachable!(),
                })
                .collect_vec()
        })
        .collect_vec();

    for it in 0..1e9 as _ {
        cycle(&mut map);
        let mut hasher = BuildHasherDefault::<fnv::FnvHasher>::default().build_hasher();
        map.hash(&mut hasher);
        let map_hash = hasher.finish();

        if let Some(cycle_start) = cache.iter().position(|(h, m)| h == &map_hash && m == &map) {
            let cycle_len = it - cycle_start;
            let remaining_steps = 1e9 as usize - it - 1;
            let remainder = remaining_steps % cycle_len;

            return get_load_p2(&cache[cycle_start + remainder].1);
        }

        cache.push((map_hash, map.clone()));
    }
    get_load_p2(&map)
}

fn main() {
    println!("Part 1: {}", part_1(INPUT));
    println!("Part 2: {}", part_2(INPUT));
}

#[cfg(test)]
mod tests {
    use std::hint::black_box;

    use super::*;

    const EXAMPLE: &[u8] = b"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(EXAMPLE), 136);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(EXAMPLE), 64);
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
