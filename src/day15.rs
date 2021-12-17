use std::cmp::{Ord, Ordering, PartialOrd};
use std::collections::BinaryHeap;

type CaveMap = Vec<Vec<i32>>;

#[derive(Eq)]
struct PathNode {
    pub risk: i32,
    pub pos: (usize, usize),
}

impl Ord for PathNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.risk.cmp(&self.risk)
    }
}

impl PartialOrd for PathNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for PathNode {
    fn eq(&self, other: &Self) -> bool {
        self.risk == other.risk
    }
}

fn find_lowest_risk_path(map: &CaveMap, repeats: usize) -> i32 {
    let map_rows = map.len();
    let map_cols = map[0].len();
    let max_rows = map.len() * repeats;
    let max_cols = map[0].len() * repeats;
    let dest = (max_rows - 1, max_cols - 1);
    let mut path_queue = BinaryHeap::new();
    path_queue.push(PathNode {
        risk: 0,
        pos: (0, 0),
    });

    let compute_risk = |row: usize, col: usize| {
        let mut risk =
            map[row % map_rows][col % map_cols] + (row / map_rows) as i32 + (col / map_cols) as i32;

        if risk > 9 {
            risk = risk - (9 * ((risk - 1) / 9));
        }

        risk
    };

    let mut visited = vec![vec![false; max_cols]; max_rows];
    while let Some(PathNode { pos: (r, c), risk }) = path_queue.pop() {
        if visited[r][c] {
            continue;
        }

        visited[r][c] = true;
        if (r, c) == dest {
            return risk;
        }

        if r > 0 {
            path_queue.push(PathNode {
                risk: risk + compute_risk(r - 1, c),
                pos: (r - 1, c),
            });
        }

        if r < max_rows - 1 {
            path_queue.push(PathNode {
                risk: risk + compute_risk(r + 1, c),
                pos: (r + 1, c),
            });
        }

        if c > 0 {
            path_queue.push(PathNode {
                risk: risk + compute_risk(r, c - 1),
                pos: (r, c - 1),
            });
        }

        if c < max_cols - 1 {
            path_queue.push(PathNode {
                risk: risk + compute_risk(r, c + 1),
                pos: (r, c + 1),
            });
        }
    }

    panic!("Did not make it to the end");
}

#[aoc_generator(day15)]
fn cave_map(input: &str) -> CaveMap {
    input
        .lines()
        .map(|s| {
            s.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

#[aoc(day15, part1)]
fn part1(map: &CaveMap) -> i32 {
    find_lowest_risk_path(map, 1)
}

#[aoc(day15, part2)]
fn part2(map: &CaveMap) -> i32 {
    find_lowest_risk_path(map, 5)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let input = cave_map(
            r"1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581",
        );
        assert_eq!(part1(&input), 40);
        assert_eq!(part2(&input), 315);
    }
}
