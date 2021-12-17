use std::collections::HashMap;

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
pub struct CaveGraph {
    adj_list: HashMap<Cave, Vec<Cave>>,
    visited_twice: Option<Cave>,
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
            visited_twice: None,
        }
    }

    pub fn find_paths(&self) -> u32 {
        self.find_path_from(&Cave::Start)
    }

    pub fn find_paths2(&self) -> u32 {
        self.find_path_from2(&Cave::Start)
    }

    fn neighbors(&self, cave: &Cave) -> &[Cave] {
        self.adj_list
            .get(cave)
            .expect("Inconsistency in cave graph!")
    }

    fn find_path_from2(&self, cave: &Cave) -> u32 {
        if *cave == Cave::End {
            // If we allowed visiting twice but didn't, this path was already hit
            if let Some(twice_cave) = &self.visited_twice {
                return !self.adj_list.contains_key(twice_cave) as u32;
            }

            return 1;
        }

        let visit_neighbors_on = |next_graph: Self| {
            self.neighbors(cave)
                .iter()
                .map(|next| next_graph.find_path_from2(next))
                .sum()
        };

        let mut next_graph = self.clone();
        if cave == &Cave::Start {
            next_graph.remove_cave(cave);
        } else if matches!(cave, &Cave::Small(_)) {
            next_graph.remove_cave(cave);
            if self.visited_twice.is_none() {
                let mut sm_twice_graph = self.clone();
                sm_twice_graph.visited_twice = Some(cave.clone());
                return visit_neighbors_on(sm_twice_graph) + visit_neighbors_on(next_graph);
            }
        }

        visit_neighbors_on(next_graph)
    }

    fn find_path_from(&self, cave: &Cave) -> u32 {
        if *cave == Cave::End {
            return 1;
        }

        let mut next_graph = self.clone();
        if matches!(cave, &Cave::Small(_) | &Cave::Start) {
            next_graph.remove_cave(cave);
        }

        self.neighbors(cave)
            .iter()
            .map(|next| next_graph.find_path_from(next))
            .sum()
    }

    fn remove_cave(&mut self, cave: &Cave) {
        if let Some(connections) = self.adj_list.remove(cave) {
            for other_cave in &connections {
                if let Some(other_conns) = self.adj_list.get_mut(other_cave) {
                    other_conns.retain(|c| c != cave);
                }
            }
        }
    }
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
    caves.find_paths()
}

#[aoc(day12, part2)]
fn part2(caves: &CaveGraph) -> u32 {
    caves.find_paths2()
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
