#![feature(test)]
extern crate test;

use std::mem;

use bstr::ByteSlice;
use itertools::Itertools;
use rayon::prelude::*;

const INPUT: &[u8] = include_bytes!("input.txt");

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, Default)]
struct Visited {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
}

impl Visited {
    fn visit_dir(&mut self, dir: Direction) -> bool {
        let field = match dir {
            Direction::Up => &mut self.up,
            Direction::Down => &mut self.down,
            Direction::Left => &mut self.left,
            Direction::Right => &mut self.right,
        };
        mem::replace(field, true)
    }

    fn is_energized(&self) -> bool {
        self.up || self.down || self.left || self.right
    }
}

fn calc_energy(map: &[u8], start: (usize, usize, Direction)) -> u32 {
    let n = map.lines().next().unwrap().len();
    let mut cursors = vec![start];
    let mut result = vec![Visited::default(); n * n];
    while let Some((x, y, dir)) = cursors.pop() {
        let (x, y) = (x, y);
        let visited = &mut result[y * n + x];
        if visited.visit_dir(dir) {
            continue;
        }

        match (map[y * (n + 1) + x], dir) {
            (b'.' | b'-', Direction::Right) | (b'/', Direction::Up) | (b'\\', Direction::Down)
                if x < n - 1 =>
            {
                cursors.push((x + 1, y, Direction::Right));
            }
            (b'.' | b'-', Direction::Left) | (b'/', Direction::Down) | (b'\\', Direction::Up)
                if x > 0 =>
            {
                cursors.push((x - 1, y, Direction::Left));
            }
            (b'.' | b'|', Direction::Down)
            | (b'/', Direction::Left)
            | (b'\\', Direction::Right)
                if y < n - 1 =>
            {
                cursors.push((x, y + 1, Direction::Down));
            }
            (b'.' | b'|', Direction::Up) | (b'/', Direction::Right) | (b'\\', Direction::Left)
                if y > 0 =>
            {
                cursors.push((x, y - 1, Direction::Up));
            }
            (b'-', Direction::Up | Direction::Down) => {
                if x > 0 {
                    cursors.push((x - 1, y, Direction::Left));
                }
                if x < n - 1 {
                    cursors.push((x + 1, y, Direction::Right));
                }
            }
            (b'|', Direction::Left | Direction::Right) => {
                if y > 0 {
                    cursors.push((x, y - 1, Direction::Up));
                }
                if y < n - 1 {
                    cursors.push((x, y + 1, Direction::Down));
                }
            }
            _ => (),
        }
    }

    result.into_iter().filter(Visited::is_energized).count() as u32
}

fn part_1(input: &[u8]) -> u32 {
    calc_energy(input, (0, 0, Direction::Right))
}

fn part_2(input: &[u8]) -> u32 {
    let n = input.lines().next().unwrap().len();
    (0..n)
        .map(|x| (x, 0, Direction::Down))
        .chain((0..n).map(|y| (0, y, Direction::Right)))
        .chain((0..n).map(|x| (x, n - 1, Direction::Up)))
        .chain((0..n).map(|y| (n - 1, y, Direction::Left)))
        .collect_vec()
        .into_par_iter()
        .map(|start| calc_energy(input, start))
        .max()
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

    const EXAMPLE: &[u8] = br".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(EXAMPLE), 46);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(EXAMPLE), 51);
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
