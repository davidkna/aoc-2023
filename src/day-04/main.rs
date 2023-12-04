#![feature(test)]
extern crate test;

use bstr::ByteSlice;
use std::collections::{HashSet, VecDeque};

const INPUT: &[u8] = include_bytes!("input.txt");

fn part_1(input: &[u8]) -> u64 {
    input
        .lines()
        .map(|line| {
            let (_, game) = line.split_once_str(": ").unwrap();
            let (winning_cards, my_cards) = game.split_once_str(" | ").unwrap();
            let winning_cards = winning_cards
                .split_str(" ")
                .filter(|n| !n.is_empty())
                .map(|n| n.to_str().unwrap().parse::<u32>().unwrap())
                .collect::<HashSet<_>>();

            let my_cards = my_cards
                .split_str(" ")
                .filter(|n| !n.is_empty())
                .map(|n| n.to_str().unwrap().parse::<u32>().unwrap())
                .collect::<HashSet<_>>();

            let match_count = winning_cards.intersection(&my_cards).count();

            match match_count {
                0 => 0,
                1 => 1,
                2 => 2,
                _ => 2_u64.pow((match_count - 1) as u32),
            }
        })
        .sum()
}

fn part_2(input: &[u8]) -> u64 {
    input
        .lines()
        .enumerate()
        .fold(
            (0, VecDeque::new()),
            |(result, mut acc), (card_no, line)| {
                let (_, game) = line.split_once_str(": ").unwrap();
                let (winning_cards, my_cards) = game.split_once_str(" | ").unwrap();
                let winning_cards = winning_cards
                    .split_str(" ")
                    .filter(|n| !n.is_empty())
                    .map(|n| n.to_str().unwrap().parse::<u32>().unwrap())
                    .collect::<HashSet<_>>();

                let my_cards = my_cards
                    .split_str(" ")
                    .filter(|n| !n.is_empty())
                    .map(|n| n.to_str().unwrap().parse::<u32>().unwrap())
                    .collect::<HashSet<_>>();

                let my_count = acc.pop_front().unwrap_or(1);

                let match_count = winning_cards.intersection(&my_cards).count();
                let acc_len = acc.len();
                for item in acc.iter_mut().take(match_count) {
                    *item += my_count;
                }

                for _ in acc_len..match_count {
                    acc.push_back(my_count + 1)
                }

                (result + my_count, acc)
            },
        )
        .0
}

fn main() {
    println!("Part 1: {}", part_1(INPUT));
    println!("Part 2: {}", part_2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &[u8] = b"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11    ";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(EXAMPLE.as_bytes()), 13);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(EXAMPLE.as_bytes()), 30);
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
