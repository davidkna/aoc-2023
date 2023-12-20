#![feature(test)]
extern crate test;

use std::collections::HashSet;

use bstr::ByteSlice;
use smallvec::smallvec_inline as smallvec;

const INPUT: &[u8] = include_bytes!("input.txt");

fn neighbors(row_count: usize, col_count: usize, row: usize, col: usize) -> Vec<usize> {
    let mut neighbors = Vec::new();

    for r in row.saturating_sub(1)..=(row + 1).min(row_count) {
        for c in col.saturating_sub(1)..=(col + 1).min(col_count) {
            neighbors.push(r * col_count + c);
        }
    }

    neighbors
}

fn is_symbol(c: u8) -> bool {
    c != b'.' && !c.is_ascii_digit()
}

fn part_1(input: &[u8]) -> u64 {
    let row_count = input.lines().count();
    let col_count = input.lines().next().unwrap().len();

    let mut grid = vec![false; row_count * col_count];
    for (i, line) in input.lines().enumerate() {
        for (j, &c) in line.iter().enumerate() {
            if is_symbol(c) {
                neighbors(row_count, col_count, i, j)
                    .iter()
                    .for_each(|&n| grid[n] = true);
            }
        }
    }

    let mut result = 0;

    for (i, line) in input.lines().enumerate() {
        let mut temp = 0;
        let mut encountered_symbol = false;
        for (j, c) in line.iter().enumerate() {
            if c.is_ascii_digit() {
                if encountered_symbol || grid[i * col_count + j] {
                    encountered_symbol = true;
                }
                temp = temp * 10 + u64::from(c - b'0');
                continue;
            } else if encountered_symbol {
                result += temp;
            }
            temp = 0;
            encountered_symbol = false;
        }
        if encountered_symbol {
            result += temp;
        }
    }

    result
}

fn part_2(input: &[u8]) -> u64 {
    let row_count = input.lines().count();
    let col_count = input.lines().next().unwrap().len();

    let mut grid = vec![None; row_count * col_count];
    let mut gear_idx = 0;
    for (i, line) in input.lines().enumerate() {
        for (j, &c) in line.iter().enumerate() {
            if c == b'*' {
                for &n in neighbors(row_count, col_count, i, j).iter() {
                    if grid[n].is_none() {
                        grid[n] = Some(smallvec![gear_idx]);
                    } else {
                        grid[n].as_mut().unwrap().push(gear_idx);
                    }
                }
                gear_idx += 1;
            }
        }
    }

    let mut gears = vec![None; gear_idx];

    for (i, line) in input.lines().enumerate() {
        let mut temp = 0;
        let mut encountered_gears: HashSet<usize> = HashSet::new();

        for (j, &c) in line.iter().enumerate() {
            if c.is_ascii_digit() {
                if let Some(gears) = grid[i * col_count + j].as_ref() {
                    encountered_gears.extend(gears);
                }
                temp = temp * 10 + u64::from(c - b'0');
                continue;
            } else {
                for &gear in &encountered_gears {
                    if gears[gear].is_none() {
                        gears[gear] = Some(smallvec![temp]);
                    } else {
                        gears[gear].as_mut().unwrap().push(temp);
                    }
                }
                temp = 0;
                encountered_gears.clear();
            }
        }
        for &gear in &encountered_gears {
            if gears[gear].is_none() {
                gears[gear] = Some(smallvec![temp]);
            } else {
                gears[gear].as_mut().unwrap().push(temp);
            }
        }
    }

    gears
        .iter()
        .flatten()
        .filter(|x| x.len() > 1)
        .map(|x| x.iter().product::<u64>())
        .sum()
}

fn main() {
    println!("Part 1: {}", part_1(INPUT));
    println!("Part 2: {}", part_2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &[u8] = b"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(EXAMPLE.as_bytes()), 4361);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(EXAMPLE.as_bytes()), 467_835);
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
