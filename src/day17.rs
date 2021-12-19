#[derive(Debug)]
struct TargetArea {
    top_left: (i64, i64),
    bot_right: (i64, i64),
}

#[derive(Debug)]
struct Probe {
    pos: (i64, i64),
    vel: (i64, i64),
}

impl Probe {
    fn with_vel(vel: (i64, i64)) -> Self {
        Probe { pos: (0, 0), vel }
    }

    fn x(&self) -> i64 {
        self.pos.0
    }

    fn y(&self) -> i64 {
        self.pos.1
    }

    fn dx(&self) -> i64 {
        self.vel.0
    }

    fn step(&mut self) {
        self.pos.0 += self.vel.0;
        self.pos.1 += self.vel.1;
        self.vel.0 -= self.vel.0.signum();
        self.vel.1 -= 1;
    }
}

impl TargetArea {
    fn contains(&self, probe: &Probe) -> bool {
        (self.top_left.0..=self.bot_right.0).contains(&probe.x())
            && (self.bot_right.1..=self.top_left.1).contains(&probe.y())
    }

    fn can_hit(&self, probe: &Probe) -> bool {
        ((probe.x() <= self.bot_right.0 && probe.dx() >= 0)
            || (probe.x() >= self.top_left.0 && probe.dx() <= 0))
            && (probe.y() >= self.bot_right.1)
    }
}

fn find_max_height(target_area: &TargetArea) -> i64 {
    let max_yvel = -target_area.bot_right.1 - 1;
    max_yvel * (max_yvel + 1) / 2
}

fn max_velocities(target_area: &TargetArea) -> (i64, i64) {
    (target_area.bot_right.0, target_area.bot_right.1.abs())
}

fn find_all_velocities(target_area: &TargetArea) -> Vec<(i64, i64)> {
    let (max_x, max_y) = max_velocities(target_area);
    let min_x = (2. * max_x as f64).sqrt().floor() as i64 - 1;

    let mut on_target = Vec::new();
    for dx in min_x..=max_x {
        for dy in -max_y..=max_y {
            let mut probe = Probe::with_vel((dx, dy));
            while target_area.can_hit(&probe) {
                probe.step();

                if target_area.contains(&probe) {
                    on_target.push((dx, dy));
                    break;
                }
            }
        }
    }

    return on_target;
}

fn split_range_str(range: &str) -> (i64, i64) {
    let (min_str, max_str) = range.split_once("..").unwrap();
    (
        min_str.parse::<i64>().unwrap(),
        max_str.parse::<i64>().unwrap(),
    )
}

#[aoc_generator(day17)]
fn target_area(input: &str) -> TargetArea {
    let (x_range, y_range) = input
        .strip_prefix("target area: ")
        .unwrap()
        .split_once(',')
        .unwrap();

    let (x_min, x_max) = split_range_str(x_range.trim().strip_prefix("x=").unwrap());
    let (y_min, y_max) = split_range_str(y_range.trim().strip_prefix("y=").unwrap());
    TargetArea {
        top_left: (x_min, y_max),
        bot_right: (x_max, y_min),
    }
}

#[aoc(day17, part1)]
fn part1(target_area: &TargetArea) -> i64 {
    // Find the highest Y-position that is reachable while still hitting the target area
    find_max_height(target_area)
}

#[aoc(day17, part2)]
fn part2(target_area: &TargetArea) -> i64 {
    let on_target = find_all_velocities(target_area);
    on_target.len() as i64
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let input = target_area(r"target area: x=20..30, y=-10..-5");
        assert_eq!(part1(&input), 45);
        assert_eq!(part2(&input), 112);
    }
}
