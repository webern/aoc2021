/*!

# Day 9: Smoke Basin

https://adventofcode.com/2021/day/9

These caves seem to be lava tubes. Parts are even still volcanically active; small hydrothermal
vents release smoke into the caves that slowly settles like rain.

If you can model how the smoke flows through the caves, you might be able to avoid it and be that
much safer. The submarine generates a heightmap of the floor of the nearby caves for you (your
puzzle input).

Smoke flows to the lowest point of the area it's in. For example, consider the following heightmap:

```text
2199943210
3987894921
9856789892
8767896789
9899965678
```

Each number corresponds to the height of a particular location, where 9 is the highest and 0 is the
lowest a location can be.

Your first goal is to find the low points - the locations that are lower than any of its adjacent
locations. Most locations have four adjacent locations (up, down, left, and right); locations on the
edge or corner of the map have three or two adjacent locations, respectively. (Diagonal locations do
not count as adjacent.)

In the above example, there are four low points, all highlighted: two are in the first row (a 1 and
a 0), one is in the third row (a 5), and one is in the bottom row (also a 5). All other locations on
the heightmap have some lower adjacent location, and so are not low points.

The risk level of a low point is 1 plus its height. In the above example, the risk levels of the low
points are 2, 1, 6, and 6. The sum of the risk levels of all low points in the heightmap is
therefore 15.

Find all of the low points on your heightmap. What is the sum of the risk levels of all low points
on your heightmap?

## Part Two

Next, you need to find the largest basins so you know what areas are most important to avoid.

A basin is all locations that eventually flow downward to a single low point. Therefore, every low
point has a basin, although some basins are very small. Locations of height 9 do not count as being
in any basin, and all other locations will always be part of exactly one basin.

The size of a basin is the number of locations within the basin, including the low point. The
example above has four basins.

The top-left basin, size 3:

```text
2199943210
3987894921
9856789892
8767896789
9899965678
```

The top-right basin, size 9:

```text
2199943210
3987894921
9856789892
8767896789
9899965678
```

The middle basin, size 14:

```text
2199943210
3987894921
9856789892
8767896789
9899965678
```

The bottom-right basin, size 9:

```text
2199943210
3987894921
9856789892
8767896789
9899965678
```

Find the three largest basins and multiply their sizes together. In the above example, this is
9 * 14 * 9 = 1134.

What do you get if you multiply together the sizes of the three largest basins?

!*/

mod data;

use crate::day9::data::{parse, INPUT_DATA};
use std::collections::HashSet;

pub fn solve() {
    println!("Part 1: {}", solve_part_1(&parse(INPUT_DATA)));
    println!("Part 2: {}", solve_part_2(&parse(INPUT_DATA)));
}

fn solve_part_1(input: &Grid) -> usize {
    let minima = find_minima(input);
    minima
        .iter()
        .map(|(row, col)| input.get(*row, *col) as usize + 1)
        .sum()
}

fn solve_part_2(input: &Grid) -> usize {
    let minima = find_minima(input);
    let mut basin_sizes: Vec<usize> = minima
        .iter()
        .map(|(row, col)| input.basin_size(*row, *col))
        .collect();
    basin_sizes.sort_by(|a, b| b.cmp(a));
    basin_sizes
        .iter()
        .take(3)
        .fold(1, |product, &val| product * val)
}

fn find_minima(input: &Grid) -> Vec<(usize, usize)> {
    let row_len = input.row_len();
    let row_count = input.row_count();
    let mut minima = Vec::new();
    for row_ix in 0..row_count {
        for col_ix in 0..row_len {
            if input.is_local_minimum(row_ix, col_ix) {
                minima.push((row_ix, col_ix));
            }
        }
    }
    minima
}

#[derive(Default, Debug, Clone)]
struct Grid(Vec<Vec<u8>>);

impl Grid {
    fn row_len(&self) -> usize {
        self.0.get(0).unwrap().len()
    }

    fn row_count(&self) -> usize {
        self.0.len()
    }

    fn get(&self, row: usize, col: usize) -> u8 {
        *self.0.get(row).unwrap().get(col).unwrap()
    }

    fn is_last_col(&self, col: usize) -> bool {
        col == self.row_len() - 1
    }

    fn is_last_row(&self, row: usize) -> bool {
        row == self.0.len() - 1
    }

    fn is_local_minimum(&self, row: usize, col: usize) -> bool {
        if !self.is_right_greater(row, col) {
            return false;
        }
        if !self.is_left_greater(row, col) {
            return false;
        }
        if !self.is_up_greater(row, col) {
            return false;
        }
        if !self.is_down_greater(row, col) {
            return false;
        }
        true
    }

    fn is_right_greater(&self, row: usize, col: usize) -> bool {
        if self.is_last_col(col) {
            return true;
        }
        self.get(row, col + 1) > self.get(row, col)
    }

    fn is_left_greater(&self, row: usize, col: usize) -> bool {
        if col == 0 {
            return true;
        }
        self.get(row, col - 1) > self.get(row, col)
    }

    fn is_up_greater(&self, row: usize, col: usize) -> bool {
        if row == 0 {
            return true;
        }
        self.get(row - 1, col) > self.get(row, col)
    }

    fn is_down_greater(&self, row: usize, col: usize) -> bool {
        if self.is_last_row(row) {
            return true;
        }
        self.get(row + 1, col) > self.get(row, col)
    }

    fn basin_size(&self, row: usize, col: usize) -> usize {
        let mut counted = HashSet::new();
        self.basin_size_recursive(row, col, &mut counted)
    }

    fn basin_size_recursive(
        &self,
        row: usize,
        col: usize,
        counted: &mut HashSet<(usize, usize)>,
    ) -> usize {
        let local_value = self.get(row, col);
        if local_value == 9 {
            return 0;
        }
        if counted.contains(&(row, col)) {
            return 0;
        }
        counted.insert((row, col));
        let left_basin_size = if col == 0 {
            0
        } else if self.is_left_greater(row, col) {
            self.basin_size_recursive(row, col - 1, counted)
        } else {
            0
        };
        let right_basin_size = if self.is_last_col(col) {
            0
        } else if self.is_right_greater(row, col) {
            self.basin_size_recursive(row, col + 1, counted)
        } else {
            0
        };
        let up_basin_size = if row == 0 {
            0
        } else if self.is_up_greater(row, col) {
            self.basin_size_recursive(row - 1, col, counted)
        } else {
            0
        };
        let down_basin_size = if self.is_last_row(row) {
            0
        } else if self.is_down_greater(row, col) {
            self.basin_size_recursive(row + 1, col, counted)
        } else {
            0
        };
        1 + left_basin_size + right_basin_size + up_basin_size + down_basin_size
    }
}

#[test]
fn solve_part_1_test() {
    let answer = solve_part_1(&parse(crate::day9::data::TEST));
    assert_eq!(answer, 15);
}

#[test]
fn solve_part_2_test() {
    let answer = solve_part_2(&parse(crate::day9::data::TEST));
    assert_eq!(answer, 1134);
}

#[test]
fn basin_size_test_1() {
    let input = parse(crate::day9::data::TEST);
    let basin_size = input.basin_size(0, 1);
    assert_eq!(basin_size, 3);
}

#[test]
fn basin_size_test_2() {
    let input = parse(crate::day9::data::TEST);
    let basin_size = input.basin_size(0, 9);
    assert_eq!(basin_size, 9);
}

#[test]
fn basin_size_test_3() {
    let input = parse(crate::day9::data::TEST);
    let basin_size = input.basin_size(2, 2);
    assert_eq!(basin_size, 14);
}

#[test]
fn basin_size_test_4() {
    let input = parse(crate::day9::data::TEST);
    let basin_size = input.basin_size(4, 6);
    assert_eq!(basin_size, 9);
}
