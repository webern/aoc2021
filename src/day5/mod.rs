mod data;

use crate::day5::data::input_data;
use anyhow::Result;
use derive_more::Add;
use std::fmt::{Display, Formatter};

pub fn solve() {
    println!("Part 1: {}", solve_part_1(input_data()).unwrap());
    println!("Part 2: {}", solve_part_2(input_data()).unwrap());
}

fn solve_part_1(input: Vec<Line>) -> Result<usize> {
    let lines = input
        .iter()
        .filter(|&line| line.is_horizontal() || line.is_vertical());
    let mut grid = Grid::default();
    for line in lines {
        grid.add_line(&line);
    }
    Ok(grid.intersection_count())
}

fn solve_part_2(input: Vec<Line>) -> Result<usize> {
    let lines = input.iter().filter(|&line| line.is_legal_for_part_2());
    let mut grid = Grid::default();
    for line in lines {
        grid.add_line(&line);
    }
    Ok(grid.intersection_count())
}

#[test]
fn solve_part_1_test() {
    let answer = solve_part_1(crate::day5::data::test_data()).unwrap();
    assert_eq!(answer, 5);
}

#[test]
fn solve_part_2_test() {
    let answer = solve_part_2(crate::day5::data::test_data()).unwrap();
    assert_eq!(answer, 12);
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Add)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Add)]
struct Line {
    a: Point,
    b: Point,
}

impl Line {
    #[allow(unused)]
    fn new(ax: usize, ay: usize, bx: usize, by: usize) -> Self {
        Self {
            a: Point { x: ax, y: ay },
            b: Point { x: bx, y: by },
        }
    }

    fn iter(&self) -> impl Iterator<Item = Point> + '_ {
        LinePoints {
            current: None,
            line: &self,
        }
    }

    fn min_point(&self) -> Point {
        std::cmp::min(self.a, self.b)
    }

    fn max_point(&self) -> Point {
        std::cmp::max(self.a, self.b)
    }

    fn is_vertical(&self) -> bool {
        self.a.x == self.b.x
    }

    fn is_horizontal(&self) -> bool {
        self.a.y == self.b.y
    }

    /// Is the line either vertical, horizontal, or diagonal?
    fn is_legal_for_part_2(&self) -> bool {
        self.is_vertical() || self.is_horizontal() || self.is_diagonal().0
    }

    /// If we travel from the "min" point to the "max" point (where X takes precedence over Y in
    /// determining which point is "greater" or "less than"), what are delta X and delta Y? delta X
    /// will always be positive, but deltaY can be negative.
    fn delta(&self) -> Delta {
        let max: Point = self.max_point();
        let min: Point = self.min_point();
        let max_y = std::cmp::max(self.a.y, self.b.y);
        let min_y = std::cmp::min(self.a.y, self.b.y);
        let is_y_negative = max.y < min.y;
        let delta_y = max_y - min_y;
        let delta_x = max.x - min.x;
        Delta {
            x: delta_x,
            y: delta_y,
            is_y_negative,
        }
    }

    /// If we travel from the "min" point to the "max" point (where X takes precedence over Y in
    /// determining which point is "greater" or "less than"), then is the slope of the line exactly
    /// +0.5 (Up) or -0.5 (Down)?
    fn is_diagonal(&self) -> (bool, UpDown) {
        let delta = self.delta();
        (
            delta.x == delta.y,
            if delta.is_y_negative {
                UpDown::Down
            } else {
                UpDown::Up
            },
        )
    }
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct Delta {
    x: usize,
    y: usize,
    is_y_negative: bool,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
enum UpDown {
    Up,
    Down,
}

struct LinePoints<'a> {
    current: Option<Point>,
    line: &'a Line,
}

impl Iterator for LinePoints<'_> {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current {
            None => {
                self.current = Some(self.line.min_point());
                Some(self.line.min_point())
            }
            Some(last) => {
                if last >= self.line.max_point() {
                    return None;
                }
                let next = if self.line.is_vertical() {
                    Point {
                        y: last.y + 1,
                        ..last
                    }
                } else if self.line.is_horizontal() {
                    Point {
                        x: last.x + 1,
                        ..last
                    }
                } else {
                    let (is_diagonal, direction) = self.line.is_diagonal();
                    if !is_diagonal {
                        panic!("unsupported line slope");
                    }
                    match direction {
                        UpDown::Up => Point {
                            x: last.x + 1,
                            y: last.y + 1,
                        },
                        UpDown::Down => Point {
                            x: last.x + 1,
                            y: last.y - 1,
                        },
                    }
                };
                self.current = Some(next);
                Some(next)
            }
        }
    }
}

#[derive(Debug, Default, Clone, Eq, PartialEq)]
struct Grid {
    rows: Vec<Vec<usize>>,
}

impl Grid {
    fn add_line(&mut self, line: &Line) {
        // println!("Adding line {:?}", line);
        for point in line.iter() {
            // println!("Adding point {:?}", point);
            self.add_point(&point);
        }
        // println!("---------------------------------------");
        // println!("{}", self);
        // println!("---------------------------------------");
    }

    fn add_point(&mut self, point: &Point) {
        if self.rows.len() <= point.y {
            self.rows.resize(point.y + 1, Default::default());
        }
        let row = self.rows.get_mut(point.y).unwrap();
        if row.len() <= point.x {
            row.resize(point.x + 1, 0);
        }
        let position = row.get_mut(point.x).unwrap();
        *position += 1;
        if *position >= 2 {
            // println!("intersection");
        }
    }

    fn intersection_count(&self) -> usize {
        let mut count = 0usize;
        for row in &self.rows {
            for &value in row {
                if value > 1 {
                    count += 1;
                }
            }
        }
        count
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in &self.rows {
            for item in row {
                write!(f, "{},", item)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

#[test]
fn test_line_iter_1() {
    let line = Line::new(0, 9, 5, 9);
    let mut iter = line.iter();
    assert_eq!(iter.next().unwrap(), Point { x: 0, y: 9 });
    assert_eq!(iter.next().unwrap(), Point { x: 1, y: 9 });
    assert_eq!(iter.next().unwrap(), Point { x: 2, y: 9 });
    assert_eq!(iter.next().unwrap(), Point { x: 3, y: 9 });
    assert_eq!(iter.next().unwrap(), Point { x: 4, y: 9 });
    assert_eq!(iter.next().unwrap(), Point { x: 5, y: 9 });
    assert!(iter.next().is_none());
}

#[test]
fn test_line_iter_2() {
    let line = Line::new(20, 5, 20, 10);
    let mut iter = line.iter();
    assert_eq!(iter.next().unwrap(), Point { x: 20, y: 5 });
    assert_eq!(iter.next().unwrap(), Point { x: 20, y: 6 });
    assert_eq!(iter.next().unwrap(), Point { x: 20, y: 7 });
    assert_eq!(iter.next().unwrap(), Point { x: 20, y: 8 });
    assert_eq!(iter.next().unwrap(), Point { x: 20, y: 9 });
    assert_eq!(iter.next().unwrap(), Point { x: 20, y: 10 });
    assert!(iter.next().is_none());
}

#[test]
fn test_line_iter_3() {
    let line = Line::new(9, 4, 3, 4);
    let mut iter = line.iter();
    assert_eq!(iter.next().unwrap(), Point { x: 3, y: 4 });
    assert_eq!(iter.next().unwrap(), Point { x: 4, y: 4 });
    assert_eq!(iter.next().unwrap(), Point { x: 5, y: 4 });
    assert_eq!(iter.next().unwrap(), Point { x: 6, y: 4 });
    assert_eq!(iter.next().unwrap(), Point { x: 7, y: 4 });
    assert_eq!(iter.next().unwrap(), Point { x: 8, y: 4 });
    assert_eq!(iter.next().unwrap(), Point { x: 9, y: 4 });
    assert!(iter.next().is_none());
}

// Line { a: Point { x: 9, y: 4 }, b: Point { x: 3, y: 4 } }
