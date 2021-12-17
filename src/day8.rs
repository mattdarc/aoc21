use std::collections::HashMap;

pub struct LogEntry {
    patterns: Vec<String>,
    output: Vec<String>,
}

impl LogEntry {
    pub fn patterns_of_len(&self, len: usize) -> Vec<String> {
        self.patterns
            .iter()
            .filter(|pat| pat.len() == len)
            .map(|pat| pat.clone())
            .collect()
    }
}

pub fn lhs_subsets_rhs(sub: &str, sup: &str) -> bool {
    sub.chars().all(|c| sup.contains(c))
}

pub fn sort_str(s: &str) -> String {
    let mut char_vec = s.chars().collect::<Vec<_>>();
    char_vec.sort();
    String::from_iter(char_vec.iter())
}

pub fn parse_patterns(pats: &str) -> Vec<String> {
    pats.split(' ').map(sort_str).collect::<Vec<String>>()
}

pub fn decode_entry(ent: &LogEntry) -> u32 {
    let mut decoded: HashMap<&str, u32> = HashMap::new();

    // Strings of unique lengths
    let one = ent.patterns_of_len(2).pop().unwrap();
    decoded.insert(&one, 1);

    let seven = ent.patterns_of_len(3).pop().unwrap();
    decoded.insert(&seven, 7);

    let four = ent.patterns_of_len(4).pop().unwrap();
    decoded.insert(&four, 4);

    let eight = ent.patterns_of_len(7).pop().unwrap();
    decoded.insert(&eight, 8);

    // Strings of length 6
    let nine_zero_six = ent.patterns_of_len(6);
    let (nine, zero_six): (Vec<String>, Vec<String>) = nine_zero_six
        .into_iter()
        .partition(|s| lhs_subsets_rhs(&four, s));
    let nine = nine.first().unwrap();
    decoded.insert(nine, 9);

    let (zero, six): (Vec<String>, Vec<String>) = zero_six
        .into_iter()
        .partition(|s| lhs_subsets_rhs(&seven, s));
    let zero = zero.first().unwrap();
    decoded.insert(zero, 0);

    let six = six.first().unwrap();
    decoded.insert(six, 6);

    // Strings of length 5
    let two_three_five = ent.patterns_of_len(5);
    let (three, two_five): (Vec<String>, Vec<String>) = two_three_five
        .into_iter()
        .partition(|s| lhs_subsets_rhs(&seven, s));
    let three = three.first().unwrap();
    decoded.insert(three, 3);

    let (five, two): (Vec<String>, Vec<String>) =
        two_five.into_iter().partition(|s| lhs_subsets_rhs(s, &six));
    decoded.insert(two.first().unwrap(), 2);
    decoded.insert(five.first().unwrap(), 5);

    ent.output
        .iter()
        .map(|s| decoded.get(s.as_str()).expect("Missing string!"))
        .fold(0, |acc, digit| 10 * acc + digit)
}

#[aoc_generator(day8)]
fn digits(input: &str) -> Vec<LogEntry> {
    input
        .lines()
        .filter_map(|line| {
            if let Some((patterns_str, output_str)) = line.split_once('|') {
                let patterns = parse_patterns(patterns_str.trim());
                let output = parse_patterns(output_str.trim());
                Some(LogEntry { patterns, output })
            } else {
                None
            }
        })
        .collect()
}

#[aoc(day8, part1)]
fn part1(input: &[LogEntry]) -> usize {
    input
        .iter()
        .map(|entry| {
            entry
                .output
                .iter()
                .filter(|segments| matches!(segments.len(), 2 | 3 | 4 | 7))
                .count()
        })
        .sum()
}

#[aoc(day8, part2)]
fn part2(entries: &[LogEntry]) -> u32 {
    entries.iter().map(decode_entry).sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn small_example() {
        let input = digits(
            r"acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf",
        );

        assert_eq!(part2(&input), 5353);
    }

    #[test]
    fn example() {
        let input = digits(
            r"
be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce",
        );
        assert_eq!(part1(&input), 26);
        assert_eq!(part2(&input), 61229);
    }
}
