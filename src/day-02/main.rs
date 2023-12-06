#![feature(test)]
extern crate test;

use bstr::ByteSlice;

const INPUT: &[u8] = include_bytes!("input.txt");

#[derive(Debug, Clone, Default)]
struct Move {
    blue: u32,
    green: u32,
    red: u32,
}

impl Move {
    fn parse(input: &[u8]) -> Self {
        let mut move_ = Self::default();
        input.split_str(", ").for_each(|ins| {
            let (count, color) = ins.split_once_str(" ").unwrap();
            let count = unsafe { count.to_str_unchecked() }.parse::<u32>().unwrap();
            match color {
                b"blue" => move_.blue += count,
                b"green" => move_.green += count,
                b"red" => move_.red += count,
                _ => unreachable!(),
            }
        });
        move_
    }
}

fn part_1(input: &[u8]) -> u32 {
    const MAX_MOVE: Move = Move {
        red: 12,
        green: 13,
        blue: 14,
    };

    input
        .lines()
        .enumerate()
        .filter(|(_i, line)| {
            let (_game, line) = line.split_once_str(": ").unwrap();
            let mut moves = line.split_str("; ").map(Move::parse);

            !moves.any(|move_| {
                move_.blue > MAX_MOVE.blue
                    || move_.green > MAX_MOVE.green
                    || move_.red > MAX_MOVE.red
            })
        })
        .map(|(i, _)| i as u32 + 1)
        .sum()
}

fn part_2(input: &[u8]) -> u32 {
    input
        .lines()
        .map(|line| {
            let (_game, line) = line.split_once_str(": ").unwrap();
            let moves = line.split_str("; ").map(Move::parse);

            let min_move = moves
                .reduce(|acc, move_| Move {
                    blue: acc.blue.max(move_.blue),
                    green: acc.green.max(move_.green),
                    red: acc.red.max(move_.red),
                })
                .unwrap();
            min_move.blue * min_move.green * min_move.red
        })
        .sum()
}

fn main() {
    println!("Part 1: {}", part_1(INPUT));
    println!("Part 2: {}", part_2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &[u8] = b"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(EXAMPLE.as_bytes()), 8);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(EXAMPLE.as_bytes()), 2286);
    }

    #[bench]
    fn bench_part_1(b: &mut test::Bencher) {
        b.iter(|| part_1(INPUT));
    }

    #[bench]
    fn bench_part_2(b: &mut test::Bencher) {
        b.iter(|| part_2(INPUT));
    }
}
