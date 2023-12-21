#![feature(test)]
extern crate test;

use std::iter;

use bstr::ByteSlice;
use fnv::FnvHashSet;
use itertools::Itertools;

const INPUT: &[u8] = include_bytes!("input.txt");

fn part_1(input: &[u8], steps: usize) -> u32 {
    let grid = input.lines().collect_vec();

    let start_position = grid
        .iter()
        .enumerate()
        .find_map(|(y, line)| line.find_byte(b'S').map(|x| (y, x)))
        .unwrap();

    let n = grid.len();

    dbg!(n);

    let mut queue = FnvHashSet::from_iter(iter::once(start_position));
    for _ in 0..steps {
        queue = queue
            .into_iter()
            .flat_map(|(y, x)| {
                [(0, 1), (0, -1), (1, 0), (-1, 0)]
                    .into_iter()
                    .zip(iter::repeat((y, x)))
                    .filter_map(|((dy, dx), (x, y))| {
                        let (ny, nx) = (y as isize + dy, x as isize + dx);
                        if ny < 0 || nx < 0 {
                            return None;
                        }
                        let (ny, nx) = (ny as usize, nx as usize);
                        if ny >= n || nx >= n {
                            return None;
                        }
                        (grid[ny][nx] != b'#').then_some((ny, nx))
                    })
            })
            .collect::<FnvHashSet<_>>();
    }
    queue.len() as u32
}

fn part_2(input: &[u8], steps: u64) -> u64 {
    let targets = [0, 65, 65 + 131, 65 + 2 * 131];
    let start = (65i16, 65i16);
    let (a, b, c) = targets
        .into_iter()
        .tuple_windows()
        .scan(
            FnvHashSet::from_iter(iter::once(start)),
            |queue, (start, end)| {
                for _ in start..end {
                    *queue = queue
                        .iter()
                        .flat_map(|&(y, x)| {
                            [(0, 1), (0, -1), (1, 0), (-1, 0)]
                                .into_iter()
                                .map(move |(dy, dx)| (y + dy, x + dx))
                                .filter(|&(y, x)| {
                                    input[(y.rem_euclid(131) as usize * 132)
                                        + x.rem_euclid(131) as usize]
                                        != b'#'
                                })
                        })
                        .collect::<FnvHashSet<_>>();
                }
                Some(queue.len() as u64)
            },
        )
        .collect_tuple()
        .unwrap();

    let x = steps / 131;
    a + x * (b - a) + x * (x - 1) / 2 * ((c - b) - (b - a))
}

fn main() {
    println!("Part 1: {}", part_1(INPUT, 64));
    println!("Part 2: {}", part_2(INPUT, 26_501_365));
}

#[cfg(test)]
mod tests {
    use std::hint::black_box;

    use super::*;

    const EXAMPLE: &[u8] = b"...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(EXAMPLE, 6), 16);
    }

    #[bench]
    fn bench_part_1(b: &mut test::Bencher) {
        b.iter(|| part_1(black_box(INPUT), 64));
    }

    #[bench]
    fn bench_part_2(b: &mut test::Bencher) {
        b.iter(|| part_2(black_box(INPUT), 26_501_365));
    }
}
