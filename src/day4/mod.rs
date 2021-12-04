use crate::day4::data::INPUT_DATA;
use crate::day4::input::Input;
use anyhow::{bail, Context, Result};
use std::collections::HashSet;

mod board;
mod data;
mod input;
mod parse;
mod row;

pub fn solve() {
    println!("Part 1: {}", solve_part_1(INPUT_DATA).unwrap());
    println!("Part 2: {}", solve_part_2(INPUT_DATA).unwrap());
}

fn solve_part_1(input_data: &str) -> Result<usize> {
    let mut it = input_data.chars().peekable();
    let Input { caller, mut boards } = Input::parse(&mut it)?;
    for call in caller.iter() {
        for board in &mut boards {
            if board.call(*call) {
                return Ok(board.uncalled_sum() * call);
            }
        }
    }
    bail!("nobody boards are winners")
}

fn solve_part_2(input_data: &str) -> Result<usize> {
    let mut it = input_data.chars().peekable();
    let Input { caller, mut boards } = Input::parse(&mut it)?;
    let mut win_order = Vec::new();
    let mut has_won = HashSet::new();
    for &call in caller.iter() {
        for (board_index, board) in boards.iter_mut().enumerate() {
            if !has_won.contains(&board_index) {
                if board.call(call) {
                    win_order.push((board_index, call));
                    has_won.insert(board_index);
                }
            }
        }
    }
    let (last_board_index, value_called_when_won) = win_order.last().context("huh?")?;
    let last_board = boards.get(*last_board_index).context("huh")?;
    return Ok(last_board.uncalled_sum() * *value_called_when_won);
}

#[test]
fn solve_part_1_test() {
    use crate::day4::data::TEST;
    let answer = solve_part_1(TEST).unwrap();
    assert_eq!(answer, 4512);
}

#[test]
fn solve_part_2_test() {
    use crate::day4::data::TEST;
    let answer = solve_part_2(TEST).unwrap();
    assert_eq!(answer, 1924);
}
