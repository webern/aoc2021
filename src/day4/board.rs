use crate::day4::row::Row;
use anyhow::{ensure, Context, Result};
use std::collections::{HashMap, HashSet};
use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub(super) struct Location {
    pub(super) row: usize,
    pub(super) col: usize,
}

/// The current state of a game board.
#[derive(Debug, Default, Clone)]
pub(super) struct Board {
    pub(super) rows: Vec<Row>,
    pub(super) called: HashSet<Location>,
    pub(super) index: HashMap<usize, Vec<Location>>,
}

impl Board {
    pub(super) fn parse(it: &mut Peekable<Chars>) -> Result<Option<Self>> {
        let mut rows = Vec::new();
        loop {
            let row = Row::parse(it)?;
            if row.vec().is_empty() {
                break;
            } else {
                rows.push(row);
            }
        }

        if rows.is_empty() {
            return Ok(None);
        }

        Ok(Some(Self::from_rows(rows)?))
    }

    pub(super) fn from_rows(rows: Vec<Row>) -> Result<Self> {
        let mut row_len = None;
        let mut index: HashMap<usize, Vec<Location>> = HashMap::new();
        for (row_index, row) in rows.iter().enumerate() {
            if let Some(row_len) = row_len {
                ensure!(row.vec().len() == row_len, "Mismatched row lengths");
            } else {
                row_len = Some(row.vec().len());
            }
            for (col_index, &value) in row.vec().iter().enumerate() {
                index.entry(value).or_default().push(Location {
                    row: row_index,
                    col: col_index,
                });
            }
        }
        Ok(Self {
            rows,
            called: Default::default(),
            index,
        })
    }

    #[allow(unused)]
    pub(super) fn get(&self, row: usize, col: usize) -> Result<usize> {
        Ok(*self
            .rows
            .get(row)
            .context("missing row")?
            .vec()
            .get(col)
            .context("missing column")?)
    }

    /// Returns the locations where `value` exists on this board (if any).
    pub(super) fn locations(&self, value: usize) -> Option<&Vec<Location>> {
        self.index.get(&value)
    }

    pub(super) fn width(&self) -> usize {
        self.rows
            .first()
            .map(|row| row.vec().len())
            .unwrap_or_default()
    }

    pub(super) fn height(&self) -> usize {
        self.rows.len()
    }

    /// Marks `value` as called and returns true if the board has a bingo.
    pub(super) fn call(&mut self, value: usize) -> bool {
        let locations = match self.locations(value) {
            None => return false,
            Some(locations) => locations.clone(),
        };
        for location in locations {
            self.called.insert(location);
        }
        self.is_winner()
    }

    pub(super) fn is_called(&self, row: usize, col: usize) -> bool {
        self.called.contains(&Location { row, col })
    }

    pub(super) fn is_winner(&self) -> bool {
        let mut row_called_count: HashMap<usize, usize> = HashMap::new();
        let mut col_called_count: HashMap<usize, usize> = HashMap::new();
        let width = self.width();
        let height = self.height();
        for location in &self.called {
            *row_called_count.entry(location.row).or_default() += 1;
            if *row_called_count.get(&location.row).unwrap() >= width {
                return true;
            }
            *col_called_count.entry(location.col).or_default() += 1;
            if *col_called_count.get(&location.col).unwrap() >= height {
                return true;
            }
        }
        false
    }

    pub(super) fn uncalled_sum(&self) -> usize {
        let mut sum = 0;
        for (row_index, row) in self.rows.iter().enumerate() {
            for (col_index, &value) in row.vec().iter().enumerate() {
                if !self.is_called(row_index, col_index) {
                    sum += value
                }
            }
        }
        sum
    }
}

#[test]
fn board_parse_1() {
    let input = r#" 3 56  6
    1  2   3
    7 8 93987589357
    "#;
    let mut it = input.chars().peekable();
    let board = Board::parse(&mut it).unwrap().unwrap();
    assert_eq!(board.rows.len(), 3);
    assert_eq!(board.get(0, 0).unwrap(), 3);
    assert_eq!(board.get(0, 1).unwrap(), 56);
    assert_eq!(board.get(0, 2).unwrap(), 6);
    assert_eq!(board.get(1, 0).unwrap(), 1);
    assert_eq!(board.get(1, 1).unwrap(), 2);
    assert_eq!(board.get(1, 2).unwrap(), 3);
    assert_eq!(board.get(2, 0).unwrap(), 7);
    assert_eq!(board.get(2, 1).unwrap(), 8);
    assert_eq!(board.get(2, 2).unwrap(), 93987589357);
    assert!(board.get(3, 0).is_err());
    assert!(board.get(0, 3).is_err());
}

#[test]
fn board_parse_2() {
    let input = r#" 3 56  6
    1  2   3
    7 8 93987589357 0
    "#;
    let mut it = input.chars().peekable();
    assert!(Board::parse(&mut it).is_err());
}

#[test]
fn board_parse_3() {
    let input = "\n";
    let mut it = input.chars().peekable();
    let none = Board::parse(&mut it).unwrap();
    assert!(none.is_none());
}
