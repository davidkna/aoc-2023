#![feature(test)]
extern crate test;

use bstr::ByteSlice;
use itertools::{izip, Itertools};

const INPUT: &[u8] = include_bytes!("input.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Facing {
    North,
    East,
    South,
    West,
}

fn part_1(input: &[u8]) -> usize {
    let grid = input.lines().collect_vec();
    let start = grid
        .iter()
        .enumerate()
        .find_map(|(y, line)| line.find_byte(b'S').map(|x| (x, y)))
        .unwrap();

    let mut cursor = start;
    let mut facing = [Facing::North, Facing::East, Facing::South, Facing::West]
        .into_iter()
        .find(|facing| {
            let next = match facing {
                Facing::North => (cursor.0, cursor.1 - 1),
                Facing::East => (cursor.0 + 1, cursor.1),
                Facing::South => (cursor.0, cursor.1 + 1),
                Facing::West => (cursor.0 - 1, cursor.1),
            };
            let tile = grid[next.1][next.0];

            match facing {
                Facing::North => matches!(tile, b'|' | b'7' | b'F'),
                Facing::West => matches!(tile, b'-' | b'7' | b'J'),
                Facing::South => matches!(tile, b'|' | b'L' | b'J'),
                Facing::East => matches!(tile, b'-' | b'L' | b'F'),
            }
        })
        .unwrap();

    let mut steps = 0;
    loop {
        steps += 1;
        cursor = match facing {
            Facing::North => (cursor.0, cursor.1 - 1),
            Facing::East => (cursor.0 + 1, cursor.1),
            Facing::South => (cursor.0, cursor.1 + 1),
            Facing::West => (cursor.0 - 1, cursor.1),
        };

        let tile = grid[cursor.1][cursor.0];
        if tile == b'S' {
            return (f64::from(steps) / 2.0).ceil() as usize;
        }

        facing = match facing {
            Facing::North => match tile {
                b'|' => Facing::North,
                b'F' => Facing::East,
                b'7' => Facing::West,
                _ => unreachable!(),
            },
            Facing::East => match tile {
                b'-' => Facing::East,
                b'7' => Facing::South,
                b'J' => Facing::North,
                _ => unreachable!(),
            },
            Facing::South => match tile {
                b'|' => Facing::South,
                b'L' => Facing::East,
                b'J' => Facing::West,
                _ => unreachable!(),
            },
            Facing::West => match tile {
                b'-' => Facing::West,
                b'F' => Facing::South,
                b'L' => Facing::North,
                _ => unreachable!(),
            },
        };
    }
}

fn part_2(input: &[u8]) -> i64 {
    let mut grid = input.lines().map(Vec::from).collect_vec();
    let mut result = vec![vec![false; grid[0].len()]; grid.len()];
    let start = grid
        .iter()
        .enumerate()
        .find_map(|(y, line)| line.find_byte(b'S').map(|x| (x, y)))
        .unwrap();

    let mut cursor = start;
    let (mut facing, start_facing_b) = [Facing::North, Facing::East, Facing::South, Facing::West]
        .into_iter()
        .filter(|facing| {
            let next = match facing {
                Facing::North => (cursor.0, cursor.1 - 1),
                Facing::East => (cursor.0 + 1, cursor.1),
                Facing::South => (cursor.0, cursor.1 + 1),
                Facing::West => (cursor.0 - 1, cursor.1),
            };
            let tile = grid[next.1][next.0];
            match facing {
                Facing::North => matches!(tile, b'|' | b'7' | b'F'),
                Facing::West => matches!(tile, b'-' | b'L' | b'F'),
                Facing::South => matches!(tile, b'|' | b'L' | b'J'),
                Facing::East => matches!(tile, b'-' | b'7' | b'J'),
            }
        })
        .collect_tuple()
        .unwrap();

    let start_replacement_tile = match (facing, start_facing_b) {
        (Facing::North, Facing::East) => b'L',
        (Facing::North, Facing::South) => b'|',
        (Facing::North, Facing::West) => b'J',
        (Facing::East, Facing::South) => b'F',
        (Facing::East, Facing::West) => b'-',
        (Facing::South, Facing::West) => b'7',
        _ => unreachable!(),
    };

    grid[start.1][start.0] = start_replacement_tile;

    loop {
        result[cursor.1][cursor.0] = true;
        cursor = match facing {
            Facing::North => (cursor.0, cursor.1 - 1),
            Facing::East => (cursor.0 + 1, cursor.1),
            Facing::South => (cursor.0, cursor.1 + 1),
            Facing::West => (cursor.0 - 1, cursor.1),
        };

        if cursor == start {
            break;
        };

        let tile = grid[cursor.1][cursor.0];

        facing = match facing {
            Facing::North => match tile {
                b'|' => Facing::North,
                b'F' => Facing::East,
                b'7' => Facing::West,
                _ => unreachable!(),
            },
            Facing::East => match tile {
                b'-' => Facing::East,
                b'7' => Facing::South,
                b'J' => Facing::North,
                _ => unreachable!(),
            },
            Facing::South => match tile {
                b'|' => Facing::South,
                b'L' => Facing::East,
                b'J' => Facing::West,
                _ => unreachable!(),
            },
            Facing::West => match tile {
                b'-' => Facing::West,
                b'F' => Facing::South,
                b'L' => Facing::North,
                _ => unreachable!(),
            },
        };
    }

    let mut total = 0;
    for (is_loop, row) in izip!(result.iter(), grid.iter()) {
        let mut inside = false;
        let mut last_tile = None;
        for (&is_loop, &tile) in izip!(is_loop.iter(), row.iter()) {
            if is_loop {
                if matches!(tile, b'|' | b'L' | b'F') {
                    inside = !inside;
                    last_tile = Some(tile);
                } else if matches!((last_tile, tile), (Some(b'L'), b'J') | (Some(b'F'), b'7')) {
                    inside = !inside;
                    last_tile = None;
                }
                continue;
            }

            last_tile = None;
            if inside {
                total += 1;
            }
        }
    }

    total
}

fn main() {
    println!("Part 1: {}", part_1(INPUT));
    println!("Part 2: {}", part_2(INPUT));
}

#[cfg(test)]
mod tests {
    use std::hint::black_box;

    use super::*;

    const EXAMPLE_01_A: &[u8] = b"-L|F7
7S-7|
L|7||
-L-J|
L|-JF";

    const EXAMPLE_01_B: &[u8] = b"7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";

    const EXAMPLE_02_A: &[u8] = b"...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
..........";

    const EXAMPLE_02_B: &[u8] = b".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(EXAMPLE_01_A), 4);
        assert_eq!(part_1(EXAMPLE_01_B), 8);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(EXAMPLE_02_A), 4);
        assert_eq!(part_2(EXAMPLE_02_B), 8);
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
