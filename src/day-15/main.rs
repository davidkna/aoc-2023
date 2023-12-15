#![feature(test, linked_list_cursors)]
extern crate test;

use std::collections::LinkedList;

use bstr::ByteSlice;

const INPUT: &[u8] = include_bytes!("input.txt");

fn hash_box(input: &[u8]) -> u8 {
    input
        .iter()
        .fold(0, |acc, &c| acc.overflowing_add(c).0.overflowing_mul(17).0)
}

fn part_1(input: &[u8]) -> u32 {
    input.split_str(",").map(hash_box).map(u32::from).sum()
}

fn part_2(input: &[u8]) -> u32 {
    input
        .split_str(",")
        .fold(vec![LinkedList::new(); 256], |mut boxes, s| {
            match s {
                [box_name @ .., b'=', value @ b'0'..=b'9'] => {
                    let box_id = hash_box(box_name) as usize;
                    let box_value = value - b'0';

                    for (name, value) in boxes[box_id].iter_mut() {
                        if name == &box_name {
                            *value = box_value;
                            return boxes;
                        }
                    }
                    boxes[box_id].push_back((box_name, box_value));
                }
                [box_name @ .., b'-'] => {
                    let box_id = hash_box(box_name) as usize;
                    let mut cursor = boxes[box_id].cursor_front_mut();
                    while let Some((name, _value)) = cursor.current() {
                        if name == &box_name {
                            cursor.remove_current();
                            break;
                        }
                        cursor.move_next();
                    }
                }
                _ => unreachable!("Invalid input: {}", s.as_bstr()),
            };
            boxes
        })
        .into_iter()
        .zip(1..)
        .map(|(list, box_num)| {
            box_num
                * list
                    .into_iter()
                    .zip(1..)
                    .map(|((_, value), slot)| value as u32 * slot)
                    .sum::<u32>()
        })
        .sum()
}

fn main() {
    println!("Part 1: {}", part_1(INPUT));
    println!("Part 2: {}", part_2(INPUT));
}

#[cfg(test)]
mod tests {
    use std::hint::black_box;

    use super::*;

    const EXAMPLE: &[u8] = b"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(b"HASH"), 52);
        assert_eq!(part_1(EXAMPLE), 1320);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(EXAMPLE), 145);
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
