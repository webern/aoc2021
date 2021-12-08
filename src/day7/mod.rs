/*!

# Day 7: The Treachery of Whales

A giant whale has decided your submarine is its next meal, and it's much faster than you are. There's nowhere to run!

Suddenly, a swarm of crabs (each in its own tiny submarine - it's too deep for them otherwise) zooms in to rescue you! They seem to be preparing to blast a hole in the ocean floor; sensors indicate a massive underground cave system just beyond where they're aiming!

The crab submarines all need to be aligned before they'll have enough power to blast a large enough hole for your submarine to get through. However, it doesn't look like they'll be aligned before the whale catches you! Maybe you can help?

There's one major catch - crab submarines can only move horizontally.

You quickly make a list of the horizontal position of each crab (your puzzle input). Crab submarines have limited fuel, so you need to find a way to make all of their horizontal positions match while requiring them to spend as little fuel as possible.

For example, consider the following horizontal positions:

```text
16,1,2,0,4,2,7,1,2,14
```

This means there's a crab with horizontal position 16, a crab with horizontal position 1, and so on.

Each change of 1 step in horizontal position of a single crab costs 1 fuel. You could choose any horizontal position to align them all on, but the one that costs the least fuel is horizontal position 2:

```text
Move from 16 to 2: 14 fuel
Move from 1 to 2: 1 fuel
Move from 2 to 2: 0 fuel
Move from 0 to 2: 2 fuel
Move from 4 to 2: 2 fuel
Move from 2 to 2: 0 fuel
Move from 7 to 2: 5 fuel
Move from 1 to 2: 1 fuel
Move from 2 to 2: 0 fuel
Move from 14 to 2: 12 fuel
```

This costs a total of 37 fuel. This is the cheapest possible outcome; more expensive outcomes include aligning at position 1 (41 fuel), position 3 (39 fuel), or position 10 (71 fuel).

Determine the horizontal position that the crabs can align to using the least fuel possible. How much fuel must they spend to align to that position?

## Part 2

The crabs don't seem interested in your proposed solution. Perhaps you misunderstand crab engineering?

As it turns out, crab submarine engines don't burn fuel at a constant rate. Instead, each change of 1 step in horizontal position costs 1 more unit of fuel than the last: the first step costs 1, the second step costs 2, the third step costs 3, and so on.

As each crab moves, moving further becomes more expensive. This changes the best horizontal position to align them all on; in the example above, this becomes 5:

```text
Move from 16 to 5: 66 fuel
Move from 1 to 5: 10 fuel
Move from 2 to 5: 6 fuel
Move from 0 to 5: 15 fuel
Move from 4 to 5: 1 fuel
Move from 2 to 5: 6 fuel
Move from 7 to 5: 3 fuel
Move from 1 to 5: 10 fuel
Move from 2 to 5: 6 fuel
Move from 14 to 5: 45 fuel
```

This costs a total of 168 fuel. This is the new cheapest possible outcome; the old alignment position (2) now costs 206 fuel instead.

Determine the horizontal position that the crabs can align to using the least fuel possible so they can make you an escape route! How much fuel must they spend to align to that position?

!*/

mod data;

use crate::day7::data::INPUT_DATA;
use std::collections::HashMap;

pub fn solve(solution: Solution) {
    println!("Part 1: {}", solve_part_1(&INPUT_DATA, solution));
    println!("Part 2: {}", brute_force_part_2(&INPUT_DATA));
}

#[derive(Debug, Copy, Clone)]
pub enum Solution {
    BruteForce,
    Optimized,
}

fn solve_part_1(input: &[usize], solution: Solution) -> usize {
    match solution {
        Solution::BruteForce => brute_force_part_1(input),
        Solution::Optimized => optimized_part_1(input),
    }
}

fn solve_part_2(input: &[usize], solution: Solution) -> usize {
    match solution {
        Solution::BruteForce => brute_force_part_2(input),
        Solution::Optimized => optimized_part_2(input),
    }
}

fn brute_force_part_1(input: &[usize]) -> usize {
    let mut min = usize::MAX;
    let mut max: usize = 0;
    for &value in input {
        if value > max {
            max = value;
        }
        if value < min {
            min = value;
        }
    }

    let mut cheapest_cost = usize::MAX;
    for i in min..=max {
        let mut this_cost = 0usize;
        for &value in input {
            let bigger = std::cmp::max(value, i);
            let smaller = std::cmp::min(value, i);
            let diff = bigger - smaller;
            this_cost += diff;
        }
        if this_cost < cheapest_cost {
            cheapest_cost = this_cost;
        }
    }

    cheapest_cost
}

fn brute_force_part_2(input: &[usize]) -> usize {
    let mut min = usize::MAX;
    let mut max: usize = 0;
    for &value in input {
        if value > max {
            max = value;
        }
        if value < min {
            min = value;
        }
    }

    let mut cheapest_cost = usize::MAX;
    for i in min..=max {
        let mut this_cost = 0usize;
        for &value in input {
            let bigger = std::cmp::max(value, i);
            let smaller = std::cmp::min(value, i);
            let diff = bigger - smaller;
            let triangular_fuel_cost = triangular_number(diff);
            this_cost += triangular_fuel_cost;
        }
        if this_cost < cheapest_cost {
            cheapest_cost = this_cost;
        }
    }

    cheapest_cost
}

fn triangular_number(n: usize) -> usize {
    (n * (n + 1)) / 2
}

#[test]
fn brute_force_part_1_test() {
    test_part_1(Solution::BruteForce)
}

#[test]
fn optimized_part_1_test() {
    test_part_1(Solution::Optimized)
}

fn test_part_1(solution: Solution) {
    // assert_eq!(solve_part_1(&crate::day7::data::TEST, solution), 37);
    // assert_eq!(solve_part_1(&[0usize, 0, 100], solution), 100);
    // assert_eq!(solve_part_1(&[1usize, 2, 3], solution), 2);
    // assert_eq!(solve_part_1(&[1usize, 1, 0], solution), 1);
    // assert_eq!(solve_part_1(&[100usize, 1], solution), 99);
    // assert_eq!(solve_part_1(&[1usize, 1, 99, 100], solution), 197);
    // assert_eq!(solve_part_1(&[1usize, 1, 99, 99, 100], solution), 197);
    assert_eq!(solve_part_1(&INPUT_DATA, solution), 356958);
}

#[test]
fn solve_part_2_test() {
    assert_eq!(brute_force_part_2(&crate::day7::data::TEST), 168);
    assert_eq!(brute_force_part_2(&INPUT_DATA), 105461913);
}

#[derive(Debug, Clone, Default)]
struct DatasetAnalysis {
    /// All numbers that are tied for being the mode of the dataset.
    modes: Vec<usize>,
    /// The number of times a mode value appears in the dataset.
    #[allow(unused)]
    mode_count: usize,
    minimum: usize,
    maximum: usize,
}

impl DatasetAnalysis {
    fn mean_floor(&self) -> usize {
        let width = self.maximum - self.minimum;
        self.minimum + (width / 2)
    }

    fn mean_ceiling(&self) -> usize {
        let mut width = self.maximum - self.minimum;
        if width % 2 != 0 {
            width += 1
        }
        self.minimum + (width / 2)
    }
}

fn analyze(input: &[usize]) -> DatasetAnalysis {
    let mut histogram = HashMap::<usize, usize>::new();
    let mut sum = 0usize;
    let mut minimum = usize::MAX;
    let mut maximum = usize::MIN;
    for &n in input {
        *histogram.entry(n).or_default() += 1;
        sum += n;
        if n < minimum {
            minimum = n;
        }
        if n > maximum {
            maximum = n;
        }
    }
    let sumf = sum as f64;
    let countf = input.len() as f64;
    let avgf = sumf / countf;
    let mut max_count = 0;
    for (_, &count) in &histogram {
        if count > max_count {
            max_count = count;
        }
    }
    let mut modes = Vec::new();
    for (n, count) in histogram {
        if count == max_count {
            modes.push(n);
        }
    }
    let analysis = DatasetAnalysis {
        modes,
        mode_count: max_count,
        minimum,
        maximum,
    };
    println!("{:?}", analysis);
    analysis
}

fn best_position(analysis: &DatasetAnalysis) -> usize {
    let mut best: usize = usize::MAX;
    let mut best_distance = usize::MAX;
    for &mode in &analysis.modes {
        let distance_from_average =
            std::cmp::max(analysis.average, mode) - std::cmp::min(analysis.average, mode);
        if distance_from_average < best_distance {
            best = mode;
            best_distance = distance_from_average;
        }
    }
    println!("{}", best);
    best
}

fn calculate_fuel_cost_part_1(input: &[usize], position: usize) -> usize {
    let mut fuel_cost = 0usize;
    for &n in input {
        let diff = std::cmp::max(position, n) - std::cmp::min(position, n);
        fuel_cost += diff;
    }
    println!("{}", fuel_cost);
    fuel_cost
}

fn calculate_fuel_cost_part_2(input: &[usize], position: usize) -> usize {
    let mut fuel_cost = 0usize;
    for &n in input {
        let diff = std::cmp::max(position, n) - std::cmp::min(position, n);
        let triangular_fuel_cost = triangular_number(diff);
        fuel_cost += triangular_fuel_cost;
    }
    println!("{}", fuel_cost);
    fuel_cost
}

#[allow(unused)]
fn optimized_part_1(input: &[usize]) -> usize {
    let analysis = analyze(input);
    let position = best_position(&analysis);
    calculate_fuel_cost_part_1(input, position)
}

#[allow(unused)]
fn optimized_part_2(input: &[usize]) -> usize {
    let analysis = analyze(input);
    let position = best_position(&analysis);
    calculate_fuel_cost_part_2(input, position)
}

#[test]
fn best_position_test() {
    let analysis = DatasetAnalysis {
        modes: vec![1, 2, 3],
        mode_count: 1,
        average: 2,
    };
    let best = best_position(&analysis);
    assert_eq!(best, 2);
}
