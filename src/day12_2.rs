// Before:
// AOC 2021
// Day 12 - Part 1 : 5920
//         generator: 12.948µs,
//         runner: 9.537342ms
//
// Day 12 - Part 2 : 155477
//         generator: 10.019µs,
//         runner: 305.574075ms
//
// After:
// AOC 2021
// Day 12 - Part 1 : 5920
//         generator: 13.519µs,
//         runner: 11.420273ms
//
// Day 12 - Part 2 : 155477
//         generator: 534.406µs,
//         runner: 426.098867ms
//

use std::collections::{HashMap, HashSet};

#[derive(Clone, Hash, PartialEq, Eq)]
pub enum Cave {
    Large(String),
    Small(String),
    Start,
    End,
}

impl std::str::FromStr for Cave {
    type Err = std::string::ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.chars().all(char::is_uppercase) {
            Ok(Cave::Large(s.to_string()))
        } else if s == "start" {
            Ok(Cave::Start)
        } else if s == "end" {
            Ok(Cave::End)
        } else {
            Ok(Cave::Small(s.to_string()))
        }
    }
}

impl std::fmt::Debug for Cave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match &self {
            Cave::Large(s) | Cave::Small(s) => s,
            Cave::Start => "start",
            Cave::End => "end",
        };

        f.write_str(name)
    }
}

#[derive(Debug, Clone)]
struct CavePath {
    caves: Vec<Cave>,
    contains: HashSet<Cave>,
}

impl CavePath {
    fn start() -> Self {
        CavePath {
            caves: vec![Cave::Start],
            contains: HashSet::new(),
        }
    }

    fn visit(&mut self, cave: Cave) {
        self.caves.push(cave.clone());
        self.contains.insert(cave);
    }

    fn can_visit(&self, cave: &Cave) -> bool {
        cave != &Cave::Start
            && (matches!(cave, &Cave::Large(_) | &Cave::End) || !self.contains.contains(cave))
    }

    fn is_at_end(&self) -> bool {
        self.caves.last().unwrap() == &Cave::End
    }

    fn current(&self) -> &Cave {
        self.caves.last().unwrap()
    }
}

#[derive(Debug, Clone)]
pub struct CaveGraph {
    adj_list: HashMap<Cave, Vec<Cave>>,
    paths: Vec<CavePath>,
}

impl CaveGraph {
    pub fn with_caves(caves: Vec<(Cave, Cave)>) -> Self {
        let mut adj_list = HashMap::new();
        for (a, b) in caves.into_iter() {
            let a_value = a.clone();
            let b_value = b.clone();
            adj_list
                .entry(a)
                .or_insert_with(|| Vec::new())
                .push(b_value);
            adj_list
                .entry(b)
                .or_insert_with(|| Vec::new())
                .push(a_value);
        }

        CaveGraph {
            adj_list,
            paths: Vec::new(),
        }
    }

    pub fn find_paths(&mut self) -> u32 {
        self.find_paths_from(CavePath::start());
        self.paths.len() as u32
    }

    fn find_paths_from(&mut self, path: CavePath) {
        if path.is_at_end() {
            self.paths.push(path);
            return;
        }

        for cave in self.neighbors(&path).into_iter() {
            let mut next_path = path.clone();
            next_path.visit(cave);
            self.find_paths_from(next_path);
        }
    }

    fn neighbors(&self, path: &CavePath) -> Vec<Cave> {
        self.adj_list
            .get(path.current())
            .expect("Inconsistency in cave graph!")
            .iter()
            .filter(|&next| path.can_visit(next))
            .map(|next| next.clone())
            .collect()
    }
}

fn can_visit_from_path(path: &[Cave], cave: &Cave) -> bool {
    !matches!(cave, &Cave::Start)
        && (matches!(cave, &Cave::Large(_) | &Cave::End) || !path.contains(cave))
}

fn can_visit_from_path_pt2(path: &[Cave], cave: &Cave) -> bool {
    let contains_duplicate = || {
        path.iter()
            .enumerate()
            .filter(|(_, visited)| matches!(visited, &Cave::Small(_)))
            .any(|(i, visited)| path[1 + i..].contains(visited))
    };

    !matches!(cave, &Cave::Start)
        && (matches!(cave, &Cave::Large(_) | &Cave::End)
            || !path.contains(cave)
            || !contains_duplicate())
}

#[aoc_generator(day12)]
fn parse_adj_list(input: &str) -> CaveGraph {
    let adj_vec = input
        .lines()
        .filter_map(|line| line.split_once('-'))
        .map(|(a, b)| (a.parse::<Cave>().unwrap(), b.parse::<Cave>().unwrap()))
        .collect::<Vec<_>>();

    CaveGraph::with_caves(adj_vec)
}

#[aoc(day12, part1)]
fn part1(caves: &CaveGraph) -> u32 {
    let mut caves = caves.clone();
    caves.find_paths()
}

#[aoc(day12, part2)]
fn part2(caves: &CaveGraph) -> u32 {
    let mut caves = caves.clone();
    caves.find_paths()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn small_example() {
        let input = parse_adj_list(
            r"start-A
start-b
A-c
A-b
b-d
A-end
b-end",
        );
        assert_eq!(part1(&input), 10);
        assert_eq!(part2(&input), 36);
    }

    #[test]
    fn example() {
        let input = parse_adj_list(
            r"fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW",
        );
        assert_eq!(part1(&input), 226);
        assert_eq!(part2(&input), 3509);
    }
}
