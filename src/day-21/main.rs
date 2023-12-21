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
    let grid = input.lines().collect_vec();
    let n = grid.len() as u64;
    let targets = [0, n / 2, n + n / 2, 2 * n + n / 2];

    let start = grid
        .iter()
        .enumerate()
        .find_map(|(y, line)| line.find_byte(b'S').map(|x| (x, y)))
        .unwrap();

    let (y0, y1, y2) = targets
        .into_iter()
        .tuple_windows()
        .scan(
            FnvHashSet::from_iter(iter::once((start.0 as i64, start.1 as i64))),
            |queue, (start, end)| {
                for _ in start..end {
                    *queue = queue
                        .iter()
                        .flat_map(|&(y, x)| {
                            [(0, 1), (0, -1), (1, 0), (-1, 0)]
                                .into_iter()
                                .zip(iter::repeat((y, x)))
                                .filter_map(|((dy, dx), (x, y))| {
                                    let (ny, nx) = (y + dy, x + dx);
                                    (grid[ny.rem_euclid(n as i64) as usize]
                                        [nx.rem_euclid(n as i64) as usize]
                                        != b'#')
                                        .then_some((ny, nx))
                                })
                        })
                        .collect::<FnvHashSet<_>>();
                }
                Some(queue.len() as u64 - 1)
            },
        )
        .collect_tuple()
        .unwrap();

    let x = steps / n;

    let a = (y2 + y0 - 2 * y1) / 2;
    let b = y1 - y0 - a;
    let c = y0;
    a * x * x + b * x + c
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
