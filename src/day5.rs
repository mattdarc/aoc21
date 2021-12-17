use std::collections::HashMap;

type Coord = i32;

const MAX_SIZE: Coord = 9;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug, Hash)]
struct Point {
    pub x: Coord,
    pub y: Coord,
}

struct Line {
    start: Point,
    end: Point,
}

struct ActivityMap {
    vents: HashMap<Point, usize>,
}

fn inclusive_range(from: Coord, to: Coord) -> Box<dyn Iterator<Item = Coord>> {
    if to > from {
        Box::new(from..=to)
    } else {
        Box::new((to..=from).rev())
    }
}

impl Line {
    pub fn points(&self) -> Vec<Point> {
        let &Point { x: x0, y: y0 } = self.start();
        let &Point { x: x1, y: y1 } = self.end();
        if x0 == x1 {
            inclusive_range(y0, y1)
                .map(|y| Point { x: x0, y })
                .collect()
        } else if y0 == y1 {
            inclusive_range(x0, x1)
                .map(|x| Point { x, y: y0 })
                .collect()
        } else {
            inclusive_range(x0, x1)
                .zip(inclusive_range(y0, y1))
                .map(|(x, y)| Point { x, y })
                .collect()
        }
    }

    pub fn is_diagonal(&self) -> bool {
        self.start.x != self.end.x && self.start.y != self.end.y
    }

    pub fn start(&self) -> &Point {
        &self.start
    }

    pub fn end(&self) -> &Point {
        &self.end
    }
}

impl ActivityMap {
    pub fn new() -> Self {
        ActivityMap {
            vents: HashMap::new(),
        }
    }

    pub fn add_line(&mut self, line: &Line) {
        line.points().iter().for_each(|p| {
            *self.vents.entry(p.clone()).or_insert(0) += 1;
        });
    }

    pub fn vents(&self) -> &HashMap<Point, usize> {
        &self.vents
    }
}

struct ParseLineError;
impl std::str::FromStr for Point {
    type Err = ParseLineError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let points = input.trim().split(',').collect::<Vec<_>>();
        match points.len() {
            2 => {
                let x = points[0].parse().or_else(|_| Err(ParseLineError))?;
                let y = points[1].parse().or_else(|_| Err(ParseLineError))?;
                Ok(Point { x, y })
            }
            _ => Err(ParseLineError),
        }
    }
}

impl std::str::FromStr for Line {
    type Err = ParseLineError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let points = input.trim().split("->").collect::<Vec<_>>();

        match points.len() {
            2 => {
                let start: Point = points[0].parse().or_else(|_| Err(ParseLineError))?;
                let end: Point = points[1].parse().or_else(|_| Err(ParseLineError))?;
                Ok(Line { start, end })
            }
            _ => Err(ParseLineError),
        }
    }
}

impl std::fmt::Debug for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({}, {}) -> ({}, {})\n",
            self.start().x,
            self.start.y,
            self.end().x,
            self.end().y
        )
    }
}

impl std::fmt::Debug for ActivityMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..=MAX_SIZE {
            for col in 0..=MAX_SIZE {
                match self.vents.get(&Point { x: col, y: row }) {
                    Some(overlap) => f.write_str(&format!(" {:2}", overlap))?,
                    None => f.write_str("  .")?,
                }
            }
            f.write_str("\n")?;
        }

        Ok(())
    }
}

#[aoc_generator(day5)]
fn lines(input: &str) -> Vec<Line> {
    input.lines().filter_map(|line| line.parse().ok()).collect()
}

#[aoc(day5, part1)]
fn part1(lines: &[Line]) -> usize {
    let mut activity = ActivityMap::new();
    lines
        .iter()
        .filter(|line| !line.is_diagonal())
        .for_each(|line| activity.add_line(line));
    activity.vents().values().filter(|&v| *v > 1).count()
}

#[aoc(day5, part2)]
fn part2(lines: &[Line]) -> usize {
    let mut activity = ActivityMap::new();
    lines.iter().for_each(|line| activity.add_line(line));
    activity.vents().values().filter(|&v| *v > 1).count()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let input = lines(
            r"0,9 -> 5,9
              8,0 -> 0,8
              9,4 -> 3,4
              2,2 -> 2,1
              7,0 -> 7,4
              6,4 -> 2,0
              0,9 -> 2,9
              3,4 -> 1,4
              0,0 -> 8,8
              5,5 -> 8,2",
        );

        assert_eq!(part1(&input), 5);
        assert_eq!(part2(&input), 12);
    }
}
