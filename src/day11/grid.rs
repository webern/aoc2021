use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(super) struct Grid<const SIZE: usize> {
    pub(super) data: [[usize; SIZE]; SIZE],
}

impl<const SIZE: usize> Grid<SIZE> {
    pub(super) const fn size(&self) -> usize {
        SIZE
    }

    /// Returns the number of flashes seen and the first all-simultaneous flash iteration.
    /// If `steps` is given, halts after that many steps. Otherwise continues until an
    /// all-simultaneous flash step is seen.
    pub(super) fn advance(&mut self, steps: Option<usize>) -> (usize, Option<usize>) {
        let mut flash_count = 0usize;
        let mut first_simultaneous_flash = None;
        let mut step = 0usize;
        loop {
            let mut hash_flashed = HashSet::new();
            let this_flash_count = self.increment(&mut hash_flashed);
            flash_count += this_flash_count;
            if this_flash_count == 100 && first_simultaneous_flash.is_none() {
                first_simultaneous_flash = Some(step + 1 /* 1-based instead of 0-based */)
            }
            match steps {
                None if this_flash_count == 100 => break,
                Some(steps) if step == steps - 1 => break,
                _ => {}
            }
            step += 1;
        }
        (flash_count, first_simultaneous_flash)
    }

    pub(super) fn increment(&mut self, has_flashed: &mut HashSet<(usize, usize)>) -> usize {
        // println!("Incrementing --------------");
        // println!("{}", self);
        let mut flash_count = 0usize;

        for r in 0..SIZE {
            for c in 0..SIZE {
                if !has_flashed.contains(&(r, c)) {
                    self.data[r][c] += 1;
                }
                if self.data[r][c] >= 10 {
                    flash_count += self.flash((r, c), has_flashed);
                }
            }
        }
        flash_count
    }

    pub(super) fn flash(
        &mut self,
        cell: (usize, usize),
        has_flashed: &mut HashSet<(usize, usize)>,
    ) -> usize {
        if has_flashed.contains(&cell) {
            return 0;
        }

        has_flashed.insert(cell);
        self.data[cell.0][cell.1] = 0;
        let mut flash_count = 1usize;
        for (affected_row, affected_col) in Neighbors::new(cell.0, cell.1, self.size()) {
            if !has_flashed.contains(&(affected_row, affected_col)) {
                self.data[affected_row][affected_col] += 1;
            }
            if self.data[affected_row][affected_col] >= 10 {
                flash_count += self.flash((affected_row, affected_col), has_flashed);
            }
        }
        flash_count
    }
}

impl<const SIZE: usize> FromStr for Grid<SIZE> {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut data = [[0usize; SIZE]; SIZE];
        for (row, line) in s.lines().enumerate() {
            for (col, c) in line.chars().enumerate() {
                data[row][col] = c as usize - '0' as usize;
            }
        }
        Ok(Self { data })
    }
}

impl<const SIZE: usize> Display for Grid<SIZE> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in 0..SIZE {
            if row > 0 {
                write!(f, "\n")?;
            }
            for col in 0..SIZE {
                if self.data[row][col] >= 10 {
                    Display::fmt(&'*', f)?;
                } else {
                    write!(f, "{}", self.data[row][col])?;
                }
            }
        }
        Ok(())
    }
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
struct Neighbors {
    center: (usize, usize),
    size: usize,
    step: usize,
}

impl Neighbors {
    fn new(center_row: usize, center_col: usize, grid_size: usize) -> Self {
        Self {
            center: (center_row, center_col),
            size: grid_size,
            step: 0,
        }
    }
}

impl Iterator for Neighbors {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let last = self.size - 1;
        let (mut row, mut col) = self.center;
        if self.step == 8 {
            return None;
        }
        let current = self.step;
        self.step += 1;
        match current {
            0 | 1 | 7 if col == last => return self.next(),
            0 | 1 | 7 => col += 1,
            3..=5 if col == 0 => return self.next(),
            3..=5 => col -= 1,
            _ => {}
        }
        match current {
            1..=3 if row == last => return self.next(),
            1..=3 => row += 1,
            5..=7 if row == 0 => return self.next(),
            5..=7 => row -= 1,
            _ => {}
        }
        Some((row, col))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::day11::data::{
        SMALL, SMALL_AFTER_STEP_1, SMALL_AFTER_STEP_2, TEST, TEST_AFTER_STEP_1, TEST_AFTER_STEP_2,
        TEST_AFTER_STEP_3, TEST_AFTER_STEP_4, TEST_AFTER_STEP_5,
    };

    #[test]
    fn neighbors_1() {
        let mut iter = Neighbors {
            center: (1, 1),
            size: 3,
            step: 0,
        };
        assert_eq!(iter.next().unwrap(), (1, 2));
        assert_eq!(iter.next().unwrap(), (2, 2));
        assert_eq!(iter.next().unwrap(), (2, 1));
        assert_eq!(iter.next().unwrap(), (2, 0));
        assert_eq!(iter.next().unwrap(), (1, 0));
        assert_eq!(iter.next().unwrap(), (0, 0));
        assert_eq!(iter.next().unwrap(), (0, 1));
        assert_eq!(iter.next().unwrap(), (0, 2));
        assert!(iter.next().is_none());
    }

    #[test]
    fn neighbors_2() {
        let mut iter = Neighbors {
            center: (0, 0),
            size: 3,
            step: 0,
        };
        assert_eq!(iter.next().unwrap(), (0, 1));
        assert_eq!(iter.next().unwrap(), (1, 1));
        assert_eq!(iter.next().unwrap(), (1, 0));
        assert!(iter.next().is_none());
    }

    #[test]
    fn neighbors_3() {
        let mut iter = Neighbors {
            center: (1, 2),
            size: 3,
            step: 0,
        };
        assert_eq!(iter.next().unwrap(), (2, 2));
        assert_eq!(iter.next().unwrap(), (2, 1));
        assert_eq!(iter.next().unwrap(), (1, 1));
        assert_eq!(iter.next().unwrap(), (0, 1));
        assert_eq!(iter.next().unwrap(), (0, 2));
        assert!(iter.next().is_none());
    }

    #[test]
    fn from_str() {
        let grid = Grid::<3>::from_str("123\n456\n789").unwrap();
        assert_eq!(grid.data[0][0], 1);
        assert_eq!(grid.data[0][1], 2);
        assert_eq!(grid.data[0][2], 3);
        assert_eq!(grid.data[1][0], 4);
        assert_eq!(grid.data[1][1], 5);
        assert_eq!(grid.data[1][2], 6);
        assert_eq!(grid.data[2][0], 7);
        assert_eq!(grid.data[2][1], 8);
        assert_eq!(grid.data[2][2], 9);
    }

    #[test]
    fn from_test_data() {
        let grid = Grid::<10>::from_str(TEST).unwrap();
        assert_eq!(grid.data[0][0], 5);
        assert_eq!(grid.data[0][3], 3);
        assert_eq!(grid.data[0][9], 3);
        assert_eq!(grid.data[7][2], 8);
        assert_eq!(grid.data[9][9], 6);
    }

    #[test]
    fn from_small_data() {
        let grid = Grid::<5>::from_str(SMALL).unwrap();
        assert_eq!(grid.data[0][0], 1);
        assert_eq!(grid.data[0][4], 1);
        assert_eq!(grid.data[3][4], 1);
        assert_eq!(grid.data[4][4], 1);
    }

    #[test]
    fn from_small_after_step_1() {
        let mut grid = Grid::<5>::from_str(SMALL).unwrap();
        grid.advance(Some(1));
        let expected = Grid::<5>::from_str(SMALL_AFTER_STEP_1).unwrap();
        assert_eq!(grid, expected);
    }

    #[test]
    fn from_small_after_step_2() {
        let mut grid = Grid::<5>::from_str(SMALL).unwrap();
        grid.advance(Some(2));
        let expected = Grid::<5>::from_str(SMALL_AFTER_STEP_2).unwrap();
        assert_eq!(grid, expected);
    }

    #[test]
    fn from_test_after_step_1() {
        let mut grid = Grid::<10>::from_str(TEST).unwrap();
        grid.advance(Some(1));
        let after_step_1 = Grid::<10>::from_str(TEST_AFTER_STEP_1).unwrap();
        assert_eq!(grid, after_step_1);
    }

    #[test]
    fn from_test_after_step_2() {
        let mut grid = Grid::<10>::from_str(TEST_AFTER_STEP_1).unwrap();
        grid.advance(Some(1));
        let after_step_1 = Grid::<10>::from_str(TEST_AFTER_STEP_2).unwrap();
        assert_eq!(grid, after_step_1);
    }

    #[test]
    fn from_test_after_step_3() {
        let mut grid = Grid::<10>::from_str(TEST_AFTER_STEP_2).unwrap();
        grid.advance(Some(1));
        let after_step_1 = Grid::<10>::from_str(TEST_AFTER_STEP_3).unwrap();
        assert_eq!(grid, after_step_1);
    }

    #[test]
    fn from_test_after_step_4() {
        let mut grid = Grid::<10>::from_str(TEST_AFTER_STEP_3).unwrap();
        grid.advance(Some(1));
        let after_step_1 = Grid::<10>::from_str(TEST_AFTER_STEP_4).unwrap();
        assert_eq!(grid, after_step_1);
    }

    #[test]
    fn from_test_after_step_5() {
        let mut grid = Grid::<10>::from_str(TEST).unwrap();
        grid.advance(Some(5));
        let after_step_1 = Grid::<10>::from_str(TEST_AFTER_STEP_5).unwrap();
        assert_eq!(grid, after_step_1);
    }
}
