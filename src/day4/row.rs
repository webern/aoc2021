use crate::day4::parse::parse_numbers;
use anyhow::Result;
use std::iter::Peekable;
use std::str::Chars;

/// A row in a bingo game board.
#[derive(Debug, Default, Clone)]
pub(super) struct Row(Vec<usize>);

impl Row {
    pub(super) fn parse(it: &mut Peekable<Chars>) -> Result<Self> {
        Ok(Self(parse_numbers(' ', it)?))
    }

    pub(super) fn vec(&self) -> &Vec<usize> {
        &self.0
    }
}

#[test]
fn test_row_parse_1() {
    let input = " 3 56  6 1  1   4";
    let mut it = input.chars().peekable();
    let row = Row::parse(&mut it).unwrap();
    assert_eq!(row.0.len(), 6);
    assert_eq!(*row.0.get(0).unwrap(), 3);
    assert_eq!(*row.0.get(5).unwrap(), 4);
}

#[test]
fn test_row_parse_2() {
    let input = " 3 56  6 \n\n1  1   4";
    let mut it = input.chars().peekable();
    let row = Row::parse(&mut it).unwrap();
    assert_eq!(row.0.len(), 3);
    assert_eq!(*row.0.get(0).unwrap(), 3);
    assert_eq!(*row.0.get(1).unwrap(), 56);
    assert_eq!(*row.0.get(2).unwrap(), 6);
}

#[test]
fn test_row_parse_3() {
    let input = " 3 56  6,1  1   4";
    let mut it = input.chars().peekable();
    assert!(Row::parse(&mut it).is_err());
}

#[test]
fn test_row_parse_4() {
    let input = "  \n";
    let mut it = input.chars().peekable();
    let row = Row::parse(&mut it).unwrap();
    assert_eq!(row.0.len(), 0);
}
