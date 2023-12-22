#![feature(test)]
extern crate test;

use std::collections::VecDeque;

use bstr::ByteSlice;
use itertools::Itertools;
use rstar::AABB;

const INPUT: &[u8] = include_bytes!("input.txt");

#[derive(Debug, Clone)]
struct Brick {
    above: AABB<[i32; 3]>,
    supported_by: u8,
}

#[derive(Debug, Clone)]
struct TreeItem {
    aabb: AABB<[i32; 3]>,
    id: usize,
}

impl rstar::RTreeObject for TreeItem {
    type Envelope = AABB<[i32; 3]>;

    fn envelope(&self) -> Self::Envelope {
        self.aabb
    }
}

fn solve(input: &[u8]) -> (u32, u32) {
    let line_cnt = input.lines().count();
    let mut rests_on = vec![vec![]; line_cnt];
    let parsed = input
        .lines()
        .map(|brick_info| {
            let (from, to) = brick_info.split_once_str("~").unwrap();

            let (x0, y0, z0) = from
                .split_str(",")
                .map(|n| unsafe { n.to_str_unchecked() }.parse::<i32>().unwrap())
                .collect_tuple()
                .unwrap();
            let (x1, y1, z1) = to
                .split_str(",")
                .map(|n| unsafe { n.to_str_unchecked() }.parse::<i32>().unwrap())
                .collect_tuple()
                .unwrap();

            AABB::from_corners([x0, y0, z0], [x1, y1, z1])
        })
        .sorted_by_key(|aabb: &AABB<[i32; 3]>| aabb.lower()[2])
        .enumerate()
        .map(|(id, aabb)| TreeItem { aabb, id })
        .collect_vec();

    let mut tree = rstar::RTree::new();

    let bricks = parsed
        .into_iter()
        .map(|mut ti| {
            let (supported_by, max_z_len, z_base) = if ti.aabb.lower()[2] == 1 {
                (vec![], 0, 0)
            } else {
                let lower = ti.aabb.lower();
                let upper = ti.aabb.upper();
                let below_aabb =
                    AABB::from_corners([lower[0], lower[1], 0], [upper[0], upper[1], lower[2] - 1]);

                let candidates = tree.locate_in_envelope_intersecting(&below_aabb);
                let (max_z_ids, max_z_len, max_z) = candidates.fold(
                    (vec![], 0, 0),
                    |(mut max_z_ids, mut max_z_len, mut max_z), r: &TreeItem| {
                        let z = r.aabb.upper()[2];
                        if z > max_z {
                            max_z_ids.clear();
                            max_z_ids.push(r.id);
                            max_z = z;
                            max_z_len = 1;
                        } else if z == max_z {
                            max_z_ids.push(r.id);
                            max_z_len += 1;
                        }
                        (max_z_ids, max_z_len, max_z)
                    },
                );

                (max_z_ids, max_z_len, max_z)
            };

            rests_on[ti.id] = supported_by;

            let mut lower = ti.aabb.lower();
            let mut upper = ti.aabb.upper();
            upper[2] = upper[2] - lower[2] + z_base + 1;
            lower[2] = z_base + 1;
            ti.aabb = AABB::from_corners(lower, upper);

            tree.insert(ti);

            upper[2] += 1;
            lower[2] = upper[2];

            Brick {
                above: AABB::from_corners(lower, upper),
                supported_by: max_z_len as u8,
            }
        })
        .collect_vec();

    let supports =
        rests_on
            .iter()
            .enumerate()
            .fold(vec![vec![]; line_cnt], |mut acc, (id, supported_by)| {
                for &support in supported_by {
                    acc[support].push(id);
                }
                acc
            });

    let mut removed = 0;
    let mut supporting = vec![vec![]; line_cnt];
    for i in 0..bricks.len() {
        let above_ids = tree
            .locate_in_envelope_intersecting(&bricks[i].above)
            .map(|r| r.id)
            .collect_vec();
        supporting[i] = above_ids.clone();
        if above_ids.iter().all(|&id| bricks[id].supported_by > 1) {
            removed += 1_u32;
        }
    }

    let mut total_removed = 0;
    for bx in 0..bricks.len() {
        let mut queue = VecDeque::from_iter(supporting[bx].iter().copied());
        let mut visited = vec![0; bricks.len()];
        visited[bx] = usize::MAX;

        while let Some(bx) = queue.pop_front() {
            visited[bx] += 1;
            if visited[bx] == bricks[bx].supported_by as usize {
                total_removed += 1;
                for &by in &supports[bx] {
                    queue.push_back(by);
                }
            }
        }
    }
    (removed, total_removed)
}

fn main() {
    let (part_1, part_2) = solve(INPUT);
    println!("Part 1: {part_1}");
    println!("Part 2: {part_2}");
}

#[cfg(test)]
mod tests {
    use std::hint::black_box;

    use super::*;

    const EXAMPLE: &[u8] = b"1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";

    #[test]
    fn test_part_1() {
        assert_eq!(solve(EXAMPLE).0, 5);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(solve(EXAMPLE).1, 7);
    }

    #[bench]
    fn bench_solve(b: &mut test::Bencher) {
        b.iter(|| solve(black_box(INPUT)));
    }
}
