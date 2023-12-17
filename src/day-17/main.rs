#![feature(test, map_try_insert)]
extern crate test;

use std::{cmp, collections::BinaryHeap};

use bstr::ByteSlice;
use fnv::FnvHashMap;
const INPUT: &[u8] = include_bytes!("input.txt");

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
enum Direction {
    North,
    South,
    East,
    West,
}

fn part_1(input: &[u8]) -> u32 {
    solve(input, false)
}

fn part_2(input: &[u8]) -> u32 {
    solve(input, true)
}

#[inline]
fn solve(input: &[u8], ultra: bool) -> u32 {
    let mut queue = BinaryHeap::new();
    let mut visited = FnvHashMap::default();
    let rows = input.lines().count();
    let cols = input.lines().next().unwrap().len();

    queue.push((cmp::Reverse(0), 0, 0, 0u8, Direction::East));

    while let Some((cmp::Reverse(prio), y, x, straight_steps, direction)) = queue.pop() {
        if (y, x) == (rows - 1, cols - 1) {
            return prio;
        }

        if let Err(mut err) = visited.try_insert((y, x, direction), straight_steps) {
            if *err.entry.get() <= straight_steps {
                continue;
            }
            err.entry.insert(straight_steps);
        }

        let can_move_straight = if ultra {
            straight_steps < 10
        } else {
            straight_steps < 3
        };

        let north_move = ((can_move_straight || direction != Direction::North)
            && direction != Direction::South
            && y > 0
            && (!ultra || direction == Direction::North || y > 4))
            .then(|| {
                if ultra && direction != Direction::North {
                    (y - 4, x, Direction::North)
                } else {
                    (y - 1, x, Direction::North)
                }
            });

        let south_move = ((direction != Direction::South || can_move_straight)
            && direction != Direction::North
            && (y < rows - 1)
            && (!ultra || direction == Direction::South || y < rows - 4))
            .then(|| {
                if ultra && direction != Direction::South {
                    (y + 4, x, Direction::South)
                } else {
                    (y + 1, x, Direction::South)
                }
            });

        let east_move = ((direction != Direction::East || can_move_straight)
            && direction != Direction::West
            && (x < cols - 1)
            && (!ultra || (y, x) == (0, 0) || direction == Direction::East || x < cols - 4))
            .then(|| {
                if ultra && (direction != Direction::East || (y, x) == (0, 0)) {
                    (y, x + 4, Direction::East)
                } else {
                    (y, x + 1, Direction::East)
                }
            });

        let west_move = ((can_move_straight || direction != Direction::West)
            && direction != Direction::East
            && x > 0
            && (!ultra || direction == Direction::West || x > 4))
            .then(|| {
                if ultra && direction != Direction::West {
                    (y, x - 4, Direction::West)
                } else {
                    (y, x - 1, Direction::West)
                }
            });

        [north_move, south_move, east_move, west_move]
            .into_iter()
            .flatten()
            .for_each(|(y_, x_, d)| {
                let prio = if ultra && (d != direction || (y, x) == (0, 0)) {
                    match d {
                        Direction::North => (0..4)
                            .map(|i| input[(y_ + i) * (cols + 1) + x_] - b'0')
                            .sum::<u8>() as u32,
                        Direction::West => (0..4)
                            .map(|i| input[y_ * (cols + 1) + x_ + i] - b'0')
                            .sum::<u8>() as u32,
                        Direction::South => (0..4)
                            .map(|i| input[(y_ - i) * (cols + 1) + x_] - b'0')
                            .sum::<u8>() as u32,
                        Direction::East => (0..4)
                            .map(|i| input[y_ * (cols + 1) + x_ - i] - b'0')
                            .sum::<u8>() as u32,
                    }
                } else {
                    (input[y_ * (cols + 1) + x_] - b'0') as u32
                } + prio;
                let straight_steps = match d {
                    _ if ultra && (d != direction || (x, y) == (0, 0)) => 4,
                    _ if d != direction => 1,
                    _ => straight_steps + 1,
                };

                queue.push((cmp::Reverse(prio), y_, x_, straight_steps, d));
            });
    }

    unreachable!()
}

fn main() {
    println!("Part 1: {}", part_1(INPUT));
    println!("Part 2: {}", part_2(INPUT));
}

#[cfg(test)]
mod tests {
    use std::hint::black_box;

    use super::*;

    const EXAMPLE: &[u8] = b"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

    const EXAMPLE2: &[u8] = b"111111111111
999999999991
999999999991
999999999991
999999999991";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(EXAMPLE), 102);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(EXAMPLE2), 71);
        assert_eq!(part_2(EXAMPLE), 94);
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
