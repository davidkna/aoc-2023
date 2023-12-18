#![feature(test)]
extern crate test;

use bstr::ByteSlice;


const INPUT: &[u8] = include_bytes!("input.txt");

fn part_1(input: &[u8]) -> u64 {
    let (_, visited, exterior) = input.lines().fold(
        ([0, 0], vec![[0, 0]], 0),
        |([x, y], mut visited, count), line| {
            let dir = line[0];
            let mut steps = (line[2] - b'0') as i64;
            match line[3] {
                d @ b'0'..=b'9' => steps = steps * 10 + (d - b'0') as i64,
                b' ' => (),
                _ => (),
            };

            let [x, y] = match dir {
                b'R' => [x + steps, y],
                b'D' => [x, y + steps],
                b'L' => [x - steps, y],
                b'U' => [x, y - steps],
                _ => panic!("Unknown direction"),
            };
            visited.push([x, y]);

            ([x, y], visited, count + steps)
        },
    );
    calc(&visited, exterior as _)
}

fn part_2(input: &[u8]) -> u64 {
    let (_, visited, exterior) = input.lines().fold(
        ([0, 0], vec![[0, 0]], 0),
        |([x, y], mut visited, count), line| {
            let dir = line[line.len() - 2];
            let steps_hex = &line[line.len() - 7..line.len() - 2];
            let mut steps_hex_padded = [b'0'; 6];
            steps_hex_padded[1..].copy_from_slice(steps_hex);

            let mut steps_hex_decode = [0; 4];
            faster_hex::hex_decode(&steps_hex_padded, &mut steps_hex_decode[1..]).unwrap();
            let steps = u32::from_be_bytes(steps_hex_decode) as i64;

            let [x, y] = match dir {
                b'0' => [x + steps, y],
                b'1' => [x, y + steps],
                b'2' => [x - steps, y],
                b'3' => [x, y - steps],
                _ => panic!("Unknown direction"),
            };
            visited.push([x, y]);

            ([x, y], visited, count + steps)
        },
    );
    calc(&visited, exterior as _)
}

fn calc(visited: &[[i64; 2]], exterior: u64) -> u64 {
    let area = visited
        .iter()
        .zip(visited.iter().cycle().skip(1))
        .map(|(&[x1, y1], &[x2, y2])| (y1 + y2) * (x2 - x1))
        .sum::<i64>()
        .abs()
        / 2;

    area as u64 + 1 + exterior / 2
}

fn main() {
    println!("Part 1: {}", part_1(INPUT));
    println!("Part 2: {}", part_2(INPUT));
}

#[cfg(test)]
mod tests {
    use std::hint::black_box;

    use super::*;

    const EXAMPLE: &[u8] = b"R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(EXAMPLE), 62);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(EXAMPLE), 952408144115);
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
