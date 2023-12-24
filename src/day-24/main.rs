#![feature(test)]
extern crate test;

use bstr::ByteSlice;
use itertools::Itertools;
use z3::ast::{Ast, Int};

const INPUT: &[u8] = include_bytes!("input.txt");

fn part_1(input: &[u8], start: f64, stop: f64) -> usize {
    let hailstones = input
        .lines()
        .map(|line| {
            let (pos, vel) = line.split_once_str(" @ ").unwrap();
            let (px, py, _pz) = pos
                .split_str(", ")
                .map(|s| unsafe { s.to_str_unchecked() }.parse::<f64>().unwrap())
                .collect_tuple()
                .unwrap();
            let (vx, vy, _vz) = vel
                .split_str(", ")
                .map(|s| {
                    unsafe { s.to_str_unchecked() }
                        .trim()
                        .parse::<f64>()
                        .unwrap()
                })
                .collect_tuple()
                .unwrap();
            ((px, py), (vx, vy))
        })
        .collect_vec();

    hailstones
        .into_iter()
        .tuple_combinations()
        .filter(|&(a, b)| {
            let ((x1, y1), (vx1, vy1)) = a;
            let ((x2, y2), (vx2, vy2)) = b;

            let d = vx1.mul_add(vy2, -(vy1 * vx2));

            if d.abs() <= f64::EPSILON {
                return false;
            }

            let t = (x2 - x1).mul_add(vy2, -((y2 - y1) * vx2)) / d;
            let u = (x2 - x1).mul_add(vy1, -((y2 - y1) * vx1)) / d;

            if t < 0.0 || u < 0.0 {
                return false;
            }

            let x = t.mul_add(vx1, x1);
            let y = t.mul_add(vy1, y1);

            (start..=stop).contains(&x) && (start..=stop).contains(&y)
        })
        .count()
}

fn part_2(input: &[u8]) -> i64 {
    let hailstones = input.lines().map(|line| {
        let (pos, vel) = line.split_once_str(" @ ").unwrap();
        let (px, py, pz) = pos
            .split_str(", ")
            .map(|s| unsafe { s.to_str_unchecked() }.parse::<i64>().unwrap())
            .collect_tuple()
            .unwrap();
        let (vx, vy, vz) = vel
            .split_str(", ")
            .map(|s| {
                unsafe { s.to_str_unchecked() }
                    .trim()
                    .parse::<i64>()
                    .unwrap()
            })
            .collect_tuple()
            .unwrap();
        ((px, py, pz), (vx, vy, vz))
    });

    let ctx = z3::Context::new(&z3::Config::default());
    let solver = z3::Solver::new(&ctx);

    let (x, y, z, vx, vy, vz) = (
        Int::new_const(&ctx, "x"),
        Int::new_const(&ctx, "y"),
        Int::new_const(&ctx, "z"),
        Int::new_const(&ctx, "vx"),
        Int::new_const(&ctx, "vy"),
        Int::new_const(&ctx, "vz"),
    );

    for (i, ((xi, yi, zi), (vxi, vyi, vzi))) in hailstones.into_iter().enumerate() {
        let ti = Int::new_const(&ctx, format!("t{i}"));
        solver.assert(
            &(ti.ge(&Int::from_i64(&ctx, 0))
                & (&ti * vxi + xi)._eq(&(&ti * &vx + &x))
                & (&ti * vyi + yi)._eq(&(&ti * &vy + &y))
                & (&ti * vzi + zi)._eq(&(&ti * &vz + &z))),
        );
    }

    assert_eq!(solver.check(), z3::SatResult::Sat);
    let model = solver.get_model().unwrap();
    let out = model.eval(&(x + y + z), true).unwrap();
    out.as_i64().unwrap()
}

fn main() {
    println!(
        "Part 1: {}",
        part_1(INPUT, 200_000_000_000_000.0, 400_000_000_000_000.0)
    );
    println!("Part 2: {}", part_2(INPUT));
}

#[cfg(test)]
mod tests {
    use std::hint::black_box;

    use super::*;

    const EXAMPLE: &[u8] = b"19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(EXAMPLE, 7.0, 27.0), 2);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(EXAMPLE), 47);
    }

    #[bench]
    fn bench_part_1(b: &mut test::Bencher) {
        b.iter(|| {
            part_1(
                black_box(INPUT),
                200_000_000_000_000.0,
                400_000_000_000_000.0,
            )
        });
    }

    #[bench]
    fn bench_part_2(b: &mut test::Bencher) {
        b.iter(|| part_2(black_box(INPUT)));
    }
}
