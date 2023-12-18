#![feature(test)]
extern crate test;

use bstr::ByteSlice;

const INPUT: &[u8] = include_bytes!("input.txt");

fn part_1(input: &[u8]) -> u32 {
    let (last_point, mut area, exterior) =
        input
            .lines()
            .fold(([0, 0], 0i32, 0i32), |([x0, y0], acc, count), line| {
                let dir = line[0];
                let mut steps = (line[2] - b'0') as i32;
                match line[3] {
                    d @ b'0'..=b'9' => steps = steps * 10 + (d - b'0') as i32,
                    b' ' => (),
                    _ => (),
                };

                let [x1, y1] = match dir {
                    b'R' => [x0 + steps, y0],
                    b'D' => [x0, y0 + steps],
                    b'L' => [x0 - steps, y0],
                    b'U' => [x0, y0 - steps],
                    _ => unreachable!("Unknown direction"),
                };

                ([x1, y1], acc + (y0 + y1) * (x1 - x0), count + steps)
            });

    area += last_point[1] * (0 - last_point[0]);
    area /= 2;
    area.unsigned_abs() + 1 + exterior as u32 / 2
}

fn part_2(input: &[u8]) -> u64 {
    let (last_point, mut area, exterior) =
        input
            .lines()
            .fold(([0, 0], 0i64, 0i64), |([x0, y0], acc, count), line| {
                let dir = line[line.len() - 2];
                let steps_hex = &line[line.len() - 7..line.len() - 2];
                let mut steps_hex_padded = [b'0'; 6];
                steps_hex_padded[1..].copy_from_slice(steps_hex);

                let mut steps_hex_decode = [0; 4];
                faster_hex::hex_decode_unchecked(&steps_hex_padded, &mut steps_hex_decode[1..]);
                let steps = u32::from_be_bytes(steps_hex_decode) as i64;

                let [x1, y1] = match dir {
                    b'0' => [x0 + steps, y0],
                    b'1' => [x0, y0 + steps],
                    b'2' => [x0 - steps, y0],
                    b'3' => [x0, y0 - steps],
                    _ => unreachable!("Unknown direction"),
                };

                ([x1, y1], acc + (y0 + y1) * (x1 - x0), count + steps)
            });

    area += last_point[1] * (0 - last_point[0]);
    area /= 2;
    area.unsigned_abs() + 1 + exterior as u64 / 2
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
