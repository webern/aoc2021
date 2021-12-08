/*!

!*/

mod data;
mod enigma;
mod parse;

use crate::day8::data::input_data;
use crate::day8::enigma::Enigma;

pub fn solve() {
    println!("Part 1: {}", solve_part_1(&input_data().unwrap()));
    println!("Part 2: {}", solve_part_2(&input_data().unwrap()));
}

fn solve_part_1(input: &[Enigma]) -> usize {
    todo!()
}

fn solve_part_2(_input: &[Enigma]) -> usize {
    todo!()
}
