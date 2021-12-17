use std::collections::HashMap;

type PolymerRules = HashMap<(char, char), char>;

#[aoc_generator(day14)]
fn parse_polymer_template(input: &str) -> (Vec<char>, PolymerRules) {
    let (template_str, rules_str) = input.split_once('\n').unwrap();
    let template = template_str.chars().collect();
    let rules = rules_str
        .lines()
        .filter_map(|line| line.split_once("->"))
        .map(|(polymers, insert)| {
            let polymers = polymers.trim().to_string();
            let c1 = polymers.chars().nth(0).unwrap();
            let c2 = polymers.chars().nth(1).unwrap();
            ((c1, c2), insert.trim().chars().nth(0).unwrap())
        })
        .collect::<HashMap<_, _>>();

    (template, rules)
}

fn polymer_stats(num_iter: usize, template: &[char], rules: &PolymerRules) -> u64 {
    let mut final_count = HashMap::new();
    let mut pair_counts = HashMap::new();

    for &base in template.iter() {
        *final_count.entry(base).or_insert(0u64) += 1;
    }

    for (a, b) in template.windows(2).map(|w| (w[0], w[1])) {
        *pair_counts.entry((a, b)).or_insert(0) += 1;
    }

    for _ in 0..num_iter {
        let mut pair_counts_prev = HashMap::new();
        std::mem::swap(&mut pair_counts, &mut pair_counts_prev);

        for (pair, count) in pair_counts_prev.iter() {
            let &new = rules.get(pair).unwrap();
            *final_count.entry(new).or_insert(0) += count;
            *pair_counts.entry((pair.0, new)).or_insert(0) += count;
            *pair_counts.entry((new, pair.1)).or_insert(0) += count;
        }
    }

    let min = final_count.values().min().unwrap();
    let max = final_count.values().max().unwrap();

    max - min
}

#[aoc(day14, part1)]
fn part1((chain, rules): &(Vec<char>, PolymerRules)) -> u64 {
    polymer_stats(10, chain, rules)
}

#[aoc(day14, part2)]
fn part2((chain, rules): &(Vec<char>, PolymerRules)) -> u64 {
    polymer_stats(40, chain, rules)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let input = parse_polymer_template(
            r"NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C
            ",
        );
        assert_eq!(part1(&input), 1588);
        //assert_eq!(part2(&input), 2188189693529);
    }
}
