/*!

!*/

mod data;
mod paper;

use crate::day13::data::{Input, INPUT_DATA};
use crate::day13::paper::Paper;
use std::str::FromStr;

pub fn solve() {
    println!("Part 1: {}", solve_part_1(INPUT_DATA));
    println!("Part 2:\n\n{}", solve_part_2(INPUT_DATA));
}

fn solve_part_1(input: &str) -> usize {
    let input = Input::from_str(input).unwrap();
    let mut paper = Paper::new_from_vec(input.points);
    paper = paper.fold(*input.folds_instructions.first().unwrap());
    paper.points.len()
}

fn solve_part_2(input: &str) -> String {
    let input = Input::from_str(input).unwrap();
    let mut paper = Paper::new_from_vec(input.points);
    for fold_instruction in input.folds_instructions {
        paper = paper.fold(fold_instruction);
    }
    paper.to_string()
}

#[test]
fn solve_part_1_test() {
    crate::init_logger();
    let answer = solve_part_1(crate::day13::data::TEST);
    assert_eq!(answer, 17);
}

#[test]
fn solve_part_2_test() {
    crate::init_logger();
    let answer = solve_part_2(crate::day13::data::TEST);
    assert_eq!(
        answer,
        r#"#####
#...#
#...#
#...#
#####
.....
.....
"#
    );
}
