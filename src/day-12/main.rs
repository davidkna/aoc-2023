#![feature(test)]
extern crate test;

use std::collections::BTreeMap;

use bstr::ByteSlice;
use itertools::Itertools;
use rayon::prelude::*;

const INPUT: &[u8] = include_bytes!("input.txt");

#[derive(Clone, Copy, PartialEq, Eq)]
enum State {
    Damaged,
    Operational,
}

fn part_1(input: &[u8]) -> usize {
    solve(input, false)
}

fn part_2(input: &[u8]) -> usize {
    solve(input, true)
}

fn collect_map_sum(
    map: impl Iterator<Item = ((usize, usize), usize)>,
) -> BTreeMap<(usize, usize), usize> {
    map.fold(BTreeMap::default(), |mut acc, ((damaged, stage), count)| {
        acc.entry((damaged, stage))
            .and_modify(|c| *c += count)
            .or_insert(count);
        acc
    })
}

fn solve(input: &[u8], is_part_2: bool) -> usize {
    input
        .lines()
        .collect_vec()
        .into_par_iter()
        .map(|line| {
            let (states, consecutive_damaged) = line.split_once_str(" ").unwrap();
            let mut states = states
                .iter()
                .map(|c| match c {
                    b'.' => Some(State::Operational),
                    b'#' => Some(State::Damaged),
                    b'?' => None,
                    _ => unreachable!(),
                })
                .collect_vec();
            let mut consecutive_damaged = consecutive_damaged
                .split_str(",")
                .map(|s| unsafe { s.to_str_unchecked() }.parse::<usize>().unwrap())
                .collect_vec();

            if is_part_2 {
                let bak = states.clone();
                for _ in 0..4 {
                    states.push(None);
                    states.extend_from_slice(&bak);
                }

                consecutive_damaged = (0..5)
                    .flat_map(|_| consecutive_damaged.clone())
                    .collect_vec();
            }

            let mut posssible_choices = BTreeMap::from_iter([((0, 0), 1)]);
            for state in states {
                posssible_choices = match state {
                    Some(State::Damaged) => posssible_choices
                        .into_iter()
                        .filter_map(|choice| {
                            let ((mut damaged, stage), count) = choice;
                            damaged += 1;
                            (damaged <= consecutive_damaged[stage])
                                .then_some(((damaged, stage), count))
                        })
                        .collect(),
                    Some(State::Operational) => {
                        collect_map_sum(posssible_choices.into_iter().filter_map(|choice| {
                            let ((damaged, stage), count) = choice;
                            match damaged {
                                0 => Some(choice),
                                n if n == consecutive_damaged[stage] => {
                                    if stage == consecutive_damaged.len() - 1 {
                                        Some(choice)
                                    } else {
                                        Some(((0, stage + 1), count))
                                    }
                                }
                                _ => None,
                            }
                        }))
                    }
                    None => {
                        collect_map_sum(posssible_choices.into_iter().flat_map(|choice| {
                            let ((damaged, stage), count) = choice;

                            // A) is Operational a valid choice?
                            let choice_operational = (damaged == 0
                                || damaged == consecutive_damaged[stage])
                                .then(|| {
                                    if damaged != 0 && stage != consecutive_damaged.len() - 1 {
                                        ((0, stage + 1), count)
                                    } else {
                                        choice
                                    }
                                });

                            // B) is Damaged a valid choice?
                            let choice_damaged = (damaged < consecutive_damaged[stage])
                                .then(|| ((damaged + 1, stage), count));

                            choice_operational
                                .into_iter()
                                .chain(choice_damaged.into_iter())
                        }))
                    }
                }
            }
            posssible_choices
                .into_iter()
                .filter_map(|((damaged, stage), count)| {
                    (stage == consecutive_damaged.len() - 1
                        && damaged == consecutive_damaged[stage])
                        .then_some(count)
                })
                .sum::<usize>()
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

    const EXAMPLE: &[u8] = b"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(EXAMPLE), 21);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(EXAMPLE), 525_152);
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
