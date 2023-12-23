#![feature(test)]
extern crate test;

use std::collections::VecDeque;

use bstr::ByteSlice;
use itertools::Itertools;

const INPUT: &[u8] = include_bytes!("input.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PathDirection {
    Forward,
    Backward,
    Both,
    Impassable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Facing {
    Up,
    Down,
    Left,
    Right,
}

fn part_1(input: &[u8]) -> usize {
    solve(input, false)
}

fn part_2(input: &[u8]) -> usize {
    solve(input, true)
}

fn solve(input: &[u8], part_2: bool) -> usize {
    use petgraph::{algo, prelude::*};

    let mut graph = DiGraphMap::new();
    let n = input.lines().count();

    let mut queue = VecDeque::new();
    queue.push_back((
        [1usize, 1usize],
        [0, 1],
        1,
        PathDirection::Both,
        Facing::Right,
    ));

    while let Some((pos, mut prev, mut path_len, direction, facing)) = queue.pop_front() {
        let (x, y) = (pos[0], pos[1]);

        if pos == [n - 2, n - 1] {
            graph.add_edge(prev, pos, path_len);
            continue;
        }

        if graph.contains_node(pos) {
            if part_2 || matches!(direction, PathDirection::Both | PathDirection::Forward) {
                graph.add_edge(prev, pos, path_len);
            }

            if part_2 || matches!(direction, PathDirection::Both | PathDirection::Backward) {
                graph.add_edge(pos, prev, path_len);
            }
            continue;
        }

        let mut next = Vec::with_capacity(3);

        // Up
        if facing != Facing::Down && y > 0 && input[(y - 1) * (n + 1) + x] != b'#' {
            let dir = match input[(y - 1) * (n + 1) + x] {
                b'^' => PathDirection::Forward,
                b'v' => PathDirection::Backward,
                _ => direction,
            };
            next.push(([x, y - 1], dir, Facing::Up));
        }
        if facing != Facing::Up && input[(y + 1) * (n + 1) + x] != b'#' {
            let dir = match input[(y + 1) * (n + 1) + x] {
                b'^' => PathDirection::Backward,
                b'v' => PathDirection::Forward,
                _ => direction,
            };
            next.push(([x, y + 1], dir, Facing::Down));
        }

        if facing != Facing::Right && input[y * (n + 1) + x - 1] != b'#' {
            let dir = match input[y * (n + 1) + x - 1] {
                b'<' => PathDirection::Forward,
                b'>' => PathDirection::Backward,
                _ => direction,
            };
            next.push(([x - 1, y], dir, Facing::Left));
        }

        if facing != Facing::Left && input[y * (n + 1) + x + 1] != b'#' {
            let dir = match input[y * (n + 1) + x + 1] {
                b'<' => PathDirection::Backward,
                b'>' => PathDirection::Forward,
                _ => direction,
            };
            next.push(([x + 1, y], dir, Facing::Right));
        }

        if next.len() >= 2 && pos != [1, 1] {
            // println!("Found junction at {:?} with {} paths", pos, next.len());
            if part_2 || matches!(direction, PathDirection::Both | PathDirection::Forward) {
                graph.add_edge(prev, pos, path_len);
            }

            if part_2 || matches!(direction, PathDirection::Both | PathDirection::Backward) {
                graph.add_edge(pos, prev, path_len);
            }
            path_len = 0;
            prev = pos;
        }
        path_len += 1;

        // println!("Found {:?} paths", next);

        for (pos, mut is_directed, facing) in next {
            if direction == PathDirection::Impassable
                || direction != PathDirection::Both && is_directed != direction
            {
                is_directed = PathDirection::Impassable;
            }
            queue.push_back((pos, prev, path_len, is_directed, facing));
        }
    }
    algo::all_simple_paths(&graph, [0, 1], [n - 2, n - 1], 0, None)
        .map(|x: Vec<_>| {
            x.iter()
                .tuple_windows()
                .map(|(a, b)| graph.edge_weight(*a, *b).unwrap())
                .sum::<usize>()
        })
        .max()
        .unwrap()
}
fn main() {
    println!("Part 1: {}", part_1(INPUT));
    println!("Part 2: {}", part_2(INPUT));
}

#[cfg(test)]
mod tests {
    use std::hint::black_box;

    use super::*;

    const EXAMPLE: &[u8] = b"#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(EXAMPLE), 94);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(EXAMPLE), 154);
    }

    #[bench]
    fn bench_part_1(b: &mut test::Bencher) {
        b.iter(|| part_1(black_box(INPUT)));
    }

    #[bench]
    fn bench_part_1_graph(b: &mut test::Bencher) {
        b.iter(|| part_1(black_box(INPUT)));
    }

    #[bench]
    fn bench_part_2(b: &mut test::Bencher) {
        b.iter(|| part_2(black_box(INPUT)));
    }
}
