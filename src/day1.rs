#[aoc_generator(day1)]
fn depths(input: &str) -> Vec<u32> {
    input
        .lines()
        .filter_map(|depth| depth.parse().ok())
        .collect()
}

fn count_adjacent_increases(range: &[u32]) -> usize {
    range
        .iter()
        .zip(range.iter().skip(1))
        .filter(|(first, second)| first < second)
        .count()
}

#[aoc(day1, part1)]
fn part1(scan_depths: &[u32]) -> usize {
    count_adjacent_increases(scan_depths)
}

#[aoc(day1, part2)]
fn part2(scan_depths: &[u32]) -> usize {
    let scan_sums = scan_depths
        .windows(3)
        .map(|w| w.iter().sum())
        .collect::<Vec<_>>();

    count_adjacent_increases(&scan_sums)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let input = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

        assert_eq!(part1(&input), 7);
        assert_eq!(part2(&input), 5);
    }
}
