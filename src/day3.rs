#[aoc_generator(day3)]
fn binary(input: &str) -> (Vec<u32>, u32) {
    (
        input
            .lines()
            .filter_map(|binary| u32::from_str_radix(binary.trim(), 2).ok())
            .collect(),
        input.find('\n').expect("input string had no newlines") as u32,
    )
}

fn num_high_bits_at(nums: &[u32], bitnum: u32) -> u32 {
    nums.iter().filter(|&n| ((n >> bitnum) & 1) == 0).count() as u32
}

fn most_common_bit(nums: &[u32], bit: u32) -> u32 {
    let half_size = nums.len() as u32 / 2;
    (num_high_bits_at(nums, bit) > half_size) as u32
}

#[aoc(day3, part1)]
fn part1((nums, width): &(Vec<u32>, u32)) -> u32 {
    let bits = || (0..*width).rev();

    let gamma = bits()
        .map(|b| most_common_bit(nums, b))
        .fold(0, |v, n| (v << 1) + n);

    let epsilon = gamma ^ ((1 << width) - 1);

    gamma * epsilon
}

#[aoc(day3, part2)]
fn part2((nums, width): &(Vec<u32>, u32)) -> u32 {
    let bits = || (0..*width).rev();

    let mut oxy = nums.clone();
    for bit in bits() {
        if oxy.len() <= 1 {
            break;
        }

        let most_common = most_common_bit(&oxy, bit);
        oxy.retain(|n| ((n >> bit) & 1) == most_common);
    }

    let mut co2 = nums.clone();
    for bit in bits() {
        if co2.len() <= 1 {
            break;
        }

        let most_common = most_common_bit(&co2, bit);
        co2.retain(|n| ((n >> bit) & 1) != most_common);
    }

    *oxy.last().expect("Oxygen is empty!") * co2.last().expect("CO2 is empty!")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let input = binary(
            r"00100
              11110
              10110
              10111
              10101
              01111
              00111
              11100
              10000
              11001
              00010
              01010",
        );

        assert_eq!(part1(&input), 198);
        assert_eq!(part2(&input), 230);
    }
}
