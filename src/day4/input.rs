use crate::day4::board::Board;
use crate::day4::parse::parse_numbers;
use anyhow::Result;
use std::iter::Peekable;
use std::str::Chars;

/// The predetermined sequence of numbers that will be called during the game.
#[derive(Debug, Default, Clone)]
pub(super) struct Caller(Vec<usize>);

impl Caller {
    pub(super) fn parse(it: &mut Peekable<Chars>) -> Result<Self> {
        Ok(Self(parse_numbers(',', it)?))
    }

    pub(super) fn iter(&self) -> impl Iterator<Item = &usize> {
        self.0.iter()
    }

    #[allow(unused)]
    pub(super) fn len(&self) -> usize {
        self.0.len()
    }
}

/// The input of a predetermined bingo game.
#[derive(Debug, Default, Clone)]
pub(super) struct Input {
    pub(super) caller: Caller,
    pub(super) boards: Vec<Board>,
}

impl Input {
    pub(super) fn parse(it: &mut Peekable<Chars>) -> Result<Self> {
        let caller = Caller::parse(it)?;
        let mut boards = Vec::new();
        while let Some(_) = it.peek() {
            if let Some(board) = Board::parse(it)? {
                boards.push(board);
            }
        }
        Ok(Self { caller, boards })
    }
}

#[test]
fn test_caller_parse_1() {
    let input = ",3,56,,6,1,,1,,,4";
    let mut it = input.chars().peekable();
    let caller = Caller::parse(&mut it).unwrap();
    assert_eq!(caller.0.len(), 6);
    assert_eq!(*caller.0.get(0).unwrap(), 3);
    assert_eq!(*caller.0.get(5).unwrap(), 4);
}

#[test]
fn test_caller_parse_2() {
    let input = ",3,56,,6,\n\n1,,1,,,4";
    let mut it = input.chars().peekable();
    let caller = Caller::parse(&mut it).unwrap();
    assert_eq!(caller.0.len(), 3);
    assert_eq!(*caller.0.get(0).unwrap(), 3);
    assert_eq!(*caller.0.get(1).unwrap(), 56);
    assert_eq!(*caller.0.get(2).unwrap(), 6);
}

#[test]
fn test_caller_parse_3() {
    let input = " 3,56,,6 1,,1,,,4";
    let mut it = input.chars().peekable();
    assert!(Caller::parse(&mut it).is_err());
}

#[test]
fn test_caller_parse_4() {
    let input = ",,\n";
    let mut it = input.chars().peekable();
    let caller = Caller::parse(&mut it).unwrap();
    assert_eq!(caller.0.len(), 0);
}

#[test]
fn input_parse_test() {
    let mut it = crate::day4::data::TEST.chars().peekable();
    let input = Input::parse(&mut it).unwrap();
    assert_eq!(input.caller.len(), 27);
    assert_eq!(input.boards.len(), 3);
}
