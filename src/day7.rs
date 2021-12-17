#[aoc_generator(day7)]
fn crabs(input: &str) -> Vec<i64> {
    input
        .lines()
        .flat_map(|line| line.split(',').filter_map(|c| c.parse().ok()))
        .collect()
}

fn min_max(all_crabs: &[i64]) -> (i64, i64) {
    let mut min = 0;
    let mut max = i64::MAX;

    all_crabs.iter().for_each(|&crab| {
        if crab < min {
            min = crab;
        } else if crab > max {
            max = crab;
        }
    });

    (min, max)
}

fn optimize_crabs(all_crabs: &[i64], cost_fn: fn(i64, i64) -> i64) -> i64 {
    let (min_pos, max_pos) = min_max(all_crabs);
    let mut last = i64::MAX;

    for pos in min_pos..=max_pos {
        let ans = all_crabs.iter().map(|&crab| cost_fn(crab, pos)).sum();
        if ans < last {
            last = ans;
        } else {
            break;
        }
    }

    last
}

#[aoc(day7, part1)]
fn part1(crabs: &[i64]) -> i64 {
    optimize_crabs(crabs, |crab, pos| (crab - pos).abs())
}

#[aoc(day7, part2)]
fn part2(crabs: &[i64]) -> i64 {
    // Closed form: ((n)(n+1) / 2)
    optimize_crabs(crabs, |crab, pos| {
        let diff = (crab - pos).abs();
        diff * (diff + 1) / 2
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let input = crabs(r"16,1,2,0,4,2,7,1,2,14");
        assert_eq!(part1(&input), 37);
        assert_eq!(part2(&input), 168);
    }
}
