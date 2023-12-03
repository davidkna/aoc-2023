#![feature(test)]
extern crate test;

use bstr::ByteSlice;

const INPUT: &[u8] = include_bytes!("input.txt");

fn part_1(input: &[u8]) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut first = 0;
            let mut last = 0;

            for c in line {
                if c.is_ascii_digit() {
                    first = u32::from(c - b'0');
                    break;
                }
            }

            for c in line.iter().rev() {
                if c.is_ascii_digit() {
                    last = u32::from(c - b'0');
                    break;
                }
            }

            first * 10 + last
        })
        .sum()
}

fn part_2(input: &[u8]) -> u32 {
    let re_forward =
        regex::bytes::Regex::new(r"(\d|(?:one|two|three|four|five|six|seven|eight|nine))").unwrap();
    let re_backward =
        regex::bytes::Regex::new(r"(\d|(?:eno|owt|eerht|ruof|evif|xis|neves|thgie|enin))").unwrap();

    input
        .lines()
        .map(|line| {
            let first = re_forward
                .captures_iter(line)
                .next()
                .map(|num| match &num[1] {
                    n if n[0].is_ascii_digit() => n[0] - b'0',
                    b"one" => 1,
                    b"two" => 2,
                    b"three" => 3,
                    b"four" => 4,
                    b"five" => 5,
                    b"six" => 6,
                    b"seven" => 7,
                    b"eight" => 8,
                    b"nine" => 9,
                    _ => unreachable!(),
                })
                .unwrap();

            let line_rev = line.iter().rev().copied().collect::<Vec<_>>();
            let last = re_backward
                .captures_iter(&line_rev)
                .next()
                .map(|c| match &c[1] {
                    n if n[0].is_ascii_digit() => n[0] - b'0',
                    b"eno" => 1,
                    b"owt" => 2,
                    b"eerht" => 3,
                    b"ruof" => 4,
                    b"evif" => 5,
                    b"xis" => 6,
                    b"neves" => 7,
                    b"thgie" => 8,
                    b"enin" => 9,
                    _ => unreachable!(),
                })
                .unwrap();

            u32::from(first * 10 + last)
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

    const EXAMPLE_01: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    const EXAMPLE_02: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(EXAMPLE_01.as_bytes()), 142);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(EXAMPLE_02.as_bytes()), 281);
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
