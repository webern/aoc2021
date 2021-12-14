use crate::day13::data::{Axis, FoldInstruction};
use log::trace;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::iter::FromIterator;

#[derive(Debug, Clone)]
pub(super) struct Paper {
    pub(super) points: HashSet<(usize, usize)>,
    width: usize,
    height: usize,
}

impl Paper {
    pub(super) fn new(points: HashSet<(usize, usize)>) -> Self {
        let mut p = Self {
            points: HashSet::from_iter(points.into_iter()),
            width: 0,
            height: 0,
        };
        for &point in &p.points {
            if point.0 + 1 > p.width {
                p.width = point.0 + 1
            }
            if point.1 + 1 > p.height {
                p.height = point.1 + 1
            }
        }
        if (p.width + 1) % 2 != 0 {
            panic!("I cannot be folded in half width-wise: {:?}", p);
        }
        if (p.height + 1) % 2 != 0 {
            panic!("I cannot be folded in half height-wise: {:?}", p);
        }
        trace!("\n{}", p);
        p
    }

    pub(super) fn new_from_vec(points: Vec<(usize, usize)>) -> Self {
        Self::new(HashSet::from_iter(points.into_iter()))
    }

    pub(super) fn fold(&self, fold_instruction: FoldInstruction) -> Self {
        // Make sure we are folding in half
        match fold_instruction.axis {
            Axis::X => {
                if self.width / 2 != fold_instruction.location {
                    panic!(
                        "Attempt to fold width-wise not in half, {:?}: {:?}",
                        self, fold_instruction
                    );
                }
            }
            Axis::Y => {
                if self.height / 2 != fold_instruction.location {
                    panic!(
                        "Attempt to fold height-wise not in half, {:?}: {:?}",
                        self, fold_instruction
                    );
                }
            }
        }

        let mut new_points = self.points.clone();

        for &point in &self.points {
            let relevant_value = affected_value(point, fold_instruction.axis);
            let relevant_dimension = self.relevant_dimension(fold_instruction.axis);
            if relevant_value > fold_instruction.location {
                new_points.remove(&point);
                let new_value = relevant_dimension - relevant_value - 1;
                let new_point = match fold_instruction.axis {
                    Axis::X => (new_value, point.1),
                    Axis::Y => (point.0, new_value),
                };
                new_points.insert(new_point);
            }
        }
        let (new_width, new_height) = match fold_instruction.axis {
            Axis::X => (funky_half(self.width), self.height),
            Axis::Y => (self.width, funky_half(self.height)),
        };
        let new_paper = Self {
            points: new_points,
            width: new_width,
            height: new_height,
        };
        trace!("\n{}", new_paper);
        new_paper
    }

    fn relevant_dimension(&self, axis: Axis) -> usize {
        match axis {
            Axis::X => self.width,
            Axis::Y => self.height,
        }
    }
}

impl Display for Paper {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let c = if self.points.contains(&(x, y)) {
                    '#'
                } else {
                    '.'
                };
                write!(f, "{}", c)?;
            }
            write!(f, "\n")?
        }
        Ok(())
    }
}

fn affected_value(point: (usize, usize), axis: Axis) -> usize {
    match axis {
        Axis::X => point.0,
        Axis::Y => point.1,
    }
}

fn funky_half(value: usize) -> usize {
    let new_value = (value - 1) / 2;
    new_value
}

#[cfg(test)]
mod test {
    use crate::day13::data::{Input, TEST};
    use crate::day13::paper::Paper;
    use crate::init_logger;
    use std::str::FromStr;

    #[test]
    fn simple_test_1() {
        init_logger();
        let data = r#"4,0

fold along x=2
"#;
        let input = Input::from_str(data).unwrap();
        let mut paper = Paper::new_from_vec(input.points);
        paper = paper.fold(*input.folds_instructions.first().unwrap());
        assert_eq!(paper.points.len(), 1);
        assert_eq!(paper.width, 2);
        assert_eq!(*paper.points.iter().next().unwrap(), (0, 0));
    }

    #[test]
    fn simple_test_2() {
        init_logger();
        let data = r#"3,0
4,2

fold along x=2
"#;
        let input = Input::from_str(data).unwrap();
        let mut paper = Paper::new_from_vec(input.points);
        paper = paper.fold(*input.folds_instructions.first().unwrap());
        assert_eq!(paper.points.len(), 2);
        assert_eq!(paper.width, 2);
        assert!(paper.points.contains(&(1, 0)));
        assert!(paper.points.contains(&(0, 2)));
    }

    #[test]
    fn test_data() {
        init_logger();
        let input = Input::from_str(TEST).unwrap();
        let mut paper = Paper::new_from_vec(input.points);
        let mut iter = input.folds_instructions.into_iter();
        paper = paper.fold(iter.next().unwrap());
        assert_eq!(paper.points.len(), 17);
        assert_eq!(paper.width, 11);
        assert_eq!(paper.height, 7);
        assert!(paper.points.contains(&(0, 0)));
        assert!(paper.points.contains(&(2, 0)));
        assert!(paper.points.contains(&(3, 0)));
        assert!(paper.points.contains(&(6, 0)));
        assert!(paper.points.contains(&(9, 0)));
        //
        assert!(paper.points.contains(&(0, 1)));
        assert!(paper.points.contains(&(4, 1)));
        //
        assert!(paper.points.contains(&(6, 2)));
        assert!(paper.points.contains(&(10, 2)));
        //
        assert!(paper.points.contains(&(0, 3)));
        assert!(paper.points.contains(&(4, 3)));
        //
        assert!(paper.points.contains(&(1, 4)));
        assert!(paper.points.contains(&(3, 4)));
        assert!(paper.points.contains(&(6, 4)));
        assert!(paper.points.contains(&(8, 4)));
        assert!(paper.points.contains(&(9, 4)));
        assert!(paper.points.contains(&(10, 4)));
        paper = paper.fold(iter.next().unwrap());
        assert_eq!(paper.points.len(), 16);
        assert_eq!(paper.width, 5);
        assert_eq!(paper.height, 7);
        //
        assert!(paper.points.contains(&(0, 0)));
        assert!(paper.points.contains(&(1, 0)));
        assert!(paper.points.contains(&(2, 0)));
        assert!(paper.points.contains(&(3, 0)));
        assert!(paper.points.contains(&(4, 0)));
        //
        assert!(paper.points.contains(&(0, 1)));
        assert!(paper.points.contains(&(4, 1)));
        //
        assert!(paper.points.contains(&(0, 2)));
        assert!(paper.points.contains(&(4, 2)));
        //
        assert!(paper.points.contains(&(0, 3)));
        assert!(paper.points.contains(&(4, 3)));
        //
        assert!(paper.points.contains(&(0, 4)));
        assert!(paper.points.contains(&(1, 4)));
        assert!(paper.points.contains(&(2, 4)));
        assert!(paper.points.contains(&(3, 4)));
        assert!(paper.points.contains(&(4, 4)));
    }

    #[test]
    fn test_input() {
        init_logger();
        let input = Input::from_str(TEST).unwrap();
        let mut paper = Paper::new_from_vec(input.points);
        for fold_instruction in input.folds_instructions {
            paper = paper.fold(fold_instruction);
        }
    }
}
