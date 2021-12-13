use anyhow::{Context, Error, Result};
use log::{debug, trace};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::str::FromStr;

#[derive(Debug, Default)]
pub(super) struct Graph {
    edges: BTreeMap<String, Vec<String>>,
}

impl FromStr for Graph {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut graph = Graph::default();
        for line in s.lines() {
            let mut iter = line.split('-');
            let from = iter.next().context("Missing 'from' node during parse")?;
            let to = iter.next().context("Missing 'tp' node during parse")?;
            graph
                .edges
                .entry(from.to_owned())
                .or_default()
                .push(to.to_owned());
            graph
                .edges
                .entry(to.to_owned())
                .or_default()
                .push(from.to_owned());
        }
        Ok(graph)
    }
}

impl Graph {
    pub(super) fn distinct_path_count(&self) -> usize {
        for item in self.edges.iter() {
            trace!("{:?}", item)
        }
        let visited = HashSet::new();
        let mut paths = Vec::new();
        let current_path = Vec::new();
        self.recurse(current_path.clone(), &mut paths, visited.clone(), "start");
        for path in &paths {
            debug!("{:?}", path);
        }
        paths.len()
    }

    fn recurse(
        &self,
        mut current_path: Vec<String>,
        paths: &mut Vec<Vec<String>>,
        mut visited: HashSet<String>,
        current: &str,
    ) {
        trace!("Current is '{:?}", current);
        current_path.push(current.to_owned());
        if current == "end" {
            trace!("end reached: pushing {:?}", current_path);
            paths.push(current_path);
            return;
        }
        let is_small = is_small_cave(&current) || current == "start";
        if is_small && visited.contains(current) {
            trace!("is_small_or_start && dfs.visited.contains({})", current);
            return;
        }
        if is_small {
            trace!("pushing '{}' into visited)", current);
            visited.insert(current.to_owned());
        }
        if let Some(me) = self.edges.get(current) {
            for next in me {
                let next_name = next.to_owned();
                if !visited.contains(&next_name) {
                    trace!("calling recurse '{}')", next_name);
                    self.recurse(current_path.clone(), paths, visited.clone(), &next_name);
                }
            }
        }
    }

    pub(super) fn distinct_path_count_part_2(&self) -> usize {
        for item in self.edges.iter() {
            trace!("{:?}", item)
        }
        let small_caves = self.distinct_small_caves();
        trace!("small caves {:?}", small_caves);
        let mut paths = HashSet::new();
        for small_cave in &small_caves {
            trace!(
                "small cave '{}' can be visited twice ----------------",
                small_cave
            );
            self.recurse_part_2(Vec::new(), &mut paths, HashMap::new(), "start", small_cave);
        }
        for path in &paths {
            debug!("{:?}", path);
        }
        paths.len()
    }

    fn recurse_part_2(
        &self,
        mut current_path: Vec<String>,
        paths: &mut HashSet<Vec<String>>,
        mut visited: HashMap<String, usize>,
        current: &str,
        small_cave_that_can_be_visited_twice: &str,
    ) {
        trace!("Current is '{:?}", current);
        current_path.push(current.to_owned());
        if current == "end" {
            trace!("end reached: pushing {:?}", current_path);
            paths.insert(current_path);
            return;
        }
        if is_visited(current, &visited, small_cave_that_can_be_visited_twice) {
            trace!(
                "'{:?}' has been visited the maximum number of times",
                current
            );
            return;
        }
        visit(current, &mut visited);
        if let Some(me) = self.edges.get(current) {
            for next in me {
                let next_name = next.to_owned();
                if !is_visited(&next_name, &visited, small_cave_that_can_be_visited_twice) {
                    trace!("calling recurse '{}')", next_name);
                    self.recurse_part_2(
                        current_path.clone(),
                        paths,
                        visited.clone(),
                        &next_name,
                        small_cave_that_can_be_visited_twice,
                    );
                }
            }
        }
    }

    fn distinct_small_caves(&self) -> Vec<String> {
        let mut small_caves = HashSet::new();
        for cave in &self.edges {
            if is_small_cave(cave.0) && cave.0 != "start" && cave.0 != "end" {
                small_caves.insert(cave.0);
            }
        }
        let mut vec: Vec<String> = small_caves.iter().map(|&s| s.to_owned()).collect();
        vec.sort();
        vec
    }
}

fn is_small_cave(s: &str) -> bool {
    for c in s.chars() {
        if c.is_ascii_uppercase() {
            return false;
        }
    }
    true
}

fn is_visited(
    node: &str,
    map: &HashMap<String, usize>,
    small_cave_that_can_be_visited_twice: &str,
) -> bool {
    if !is_small_cave(node) {
        return false;
    }
    let is_visited = map
        .get(node)
        .map(|&visit_count| {
            if node == small_cave_that_can_be_visited_twice {
                visit_count >= 2
            } else {
                visit_count >= 1
            }
        })
        .unwrap_or_default();
    trace!("{} is visited: {}", node, is_visited);
    is_visited
}

fn visit(node: &str, map: &mut HashMap<String, usize>) {
    if !is_small_cave(node) {
        return;
    }
    *map.entry(node.to_owned()).or_default() += 1;
    trace!("incremented visit count for {}", node);
    trace!("visit map is {:?}", map);
}

#[cfg(test)]
mod test {
    use crate::day12::data::{TEST_1, TEST_2, TEST_3};
    use crate::day12::graph::Graph;
    use crate::init_logger;
    use std::str::FromStr;

    #[test]
    fn distinct_path_count_1() {
        init_logger();
        let graph = Graph::from_str(TEST_1).unwrap();
        let answer = graph.distinct_path_count();
        assert_eq!(answer, 10);
    }

    #[test]
    fn distinct_path_count_2() {
        init_logger();
        let graph = Graph::from_str(TEST_2).unwrap();
        let answer = graph.distinct_path_count();
        assert_eq!(answer, 19);
    }

    #[test]
    fn distinct_path_count_3() {
        init_logger();
        let graph = Graph::from_str(TEST_3).unwrap();
        let answer = graph.distinct_path_count();
        assert_eq!(answer, 226);
    }

    #[test]
    fn distinct_path_count_1_part_2() {
        init_logger();
        let graph = Graph::from_str(TEST_1).unwrap();
        let answer = graph.distinct_path_count_part_2();
        assert_eq!(answer, 36);
    }

    #[test]
    fn distinct_path_count_2_part_2() {
        init_logger();
        let graph = Graph::from_str(TEST_3).unwrap();
        let answer = graph.distinct_path_count_part_2();
        assert_eq!(answer, 3509);
    }
}
