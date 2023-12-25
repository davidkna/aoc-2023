#![feature(test)]
extern crate test;

use bstr::ByteSlice;

use petgraph::{algo, dot, prelude::*};
use rayon::prelude::*;

const INPUT: &[u8] = include_bytes!("input.txt");

fn part_1(input: &[u8]) -> usize {
    let mut graph = UnGraphMap::new();

    for line in input.lines() {
        let from = &line[..3];

        line[5..].split_str(" ").for_each(|to| {
            graph.add_edge(from.to_str().unwrap(), to.to_str().unwrap(), ());
        });
    }

    println!(
        "{:?}",
        dot::Dot::with_config(&graph, &[dot::Config::EdgeNoLabel])
    );

    graph.remove_edge("htj", "pcc");
    graph.remove_edge("dlk", "pjj");
    graph.remove_edge("bbg", "htb");

    assert!(algo::connected_components(&graph) == 2);
    algo::tarjan_scc(&graph)
        .into_iter()
        .map(|scc| scc.len())
        .product()
}

fn main() {
    println!("Part 1: {}", part_1(INPUT));
}
