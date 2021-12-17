#[aoc_generator(day6)]
fn fish(input: &str) -> Vec<i64> {
    input
        .lines()
        .flat_map(|line| line.split(',').filter_map(|c| c.parse().ok()))
        .collect()
}

fn simulate_fish(all_fish: &[i64], num_days: usize) -> i64 {
    let mut counts = [0i64; 9];
    all_fish.iter().for_each(|&n| counts[n as usize] += 1);

    for _ in 0..num_days {
        counts.rotate_left(1);
        counts[6] += counts[8];
    }

    counts.iter().sum()
}

#[aoc(day6, part1)]
fn part1(fish: &[i64]) -> i64 {
    simulate_fish(fish, 80)
}

#[aoc(day6, part2)]
fn part2(fish: &[i64]) -> i64 {
    simulate_fish(fish, 256)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let input = fish(r"3,4,3,1,2");
        assert_eq!(part1(&input), 5934);
        assert_eq!(part2(&input), 26984457539);
    }
}
