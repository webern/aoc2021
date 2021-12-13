/*!

!*/

mod data;
mod graph;

use crate::day12::data::INPUT_DATA;
use crate::day12::graph::Graph;
use std::str::FromStr;

pub fn solve() {
    println!("Part 1: {}", solve_part_1(INPUT_DATA));
    println!("Part 2: {}", solve_part_2(INPUT_DATA));
}

fn solve_part_1(input: &str) -> usize {
    let graph = Graph::from_str(input).unwrap();
    graph.distinct_path_count()
}

fn solve_part_2(input: &str) -> usize {
    let graph = Graph::from_str(input).unwrap();
    graph.distinct_path_count_part_2()
}

#[test]
fn solve_part_1_test() {
    crate::init_logger();
    let answer = solve_part_1(crate::day12::data::TEST_1);
    assert_eq!(answer, 10);
}

#[test]
fn solve_part_2_test() {
    crate::init_logger();
    let answer = solve_part_2(crate::day12::data::TEST_1);
    assert_eq!(answer, 36);
}
