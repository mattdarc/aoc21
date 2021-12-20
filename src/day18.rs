use std::fmt::Write;

#[derive(Clone)]
enum Number {
    Regular(i64),
    Pair(Box<Number>, Box<Number>),
}

#[derive(Debug)]
enum Xform {
    Explode(Option<i64>, Option<i64>),
    Split,
    Identity,
}

impl Xform {
    fn reduced(&self) -> bool {
        match self {
            Xform::Identity => false,
            _ => true,
        }
    }
}

impl Number {
    fn magnitude(&self) -> i64 {
        match self {
            Number::Regular(n) => *n,
            Number::Pair(lhs, rhs) => 3 * lhs.magnitude() + 2 * rhs.magnitude(),
        }
    }

    fn reduce(&mut self) -> bool {
        self.explode(0).reduced() || self.split().reduced()
    }

    fn split(&mut self) -> Xform {
        if self.is_regular() {
            let value = self.unwrap_regular();
            if value > 9 {
                let half = (value as f64) / 2.;
                *self = Number::Pair(
                    Box::new(Number::Regular(half.floor() as i64)),
                    Box::new(Number::Regular(half.ceil() as i64)),
                );
                return Xform::Split;
            }
        } else {
            let (lhs, rhs) = self.unwrap_pair();
            let mut xform = lhs.split();
            if !xform.reduced() {
                xform = rhs.split();
            }
            return xform;
        }

        return Xform::Identity;
    }

    fn explode(&mut self, depth: usize) -> Xform {
        if self.is_regular() {
            return Xform::Identity;
        }

        let (lhs, rhs) = self.unwrap_pair();
        if lhs.is_regular() && rhs.is_regular() && depth >= 4 {
            let xform = Xform::Explode(Some(lhs.unwrap_regular()), Some(rhs.unwrap_regular()));
            *self = Number::Regular(0);
            return xform;
        }

        let (lhs, rhs) = self.unwrap_pair();
        let mut xform = lhs.explode(depth + 1);
        if xform.reduced() {
            if let Xform::Explode(a, Some(b)) = xform {
                rhs.explode_rightward(b);
                xform = Xform::Explode(a, None);
            }
        } else {
            xform = rhs.explode(depth + 1);
            if let Xform::Explode(Some(a), b) = xform {
                lhs.explode_leftward(a);
                xform = Xform::Explode(None, b);
            }
        }

        xform
    }

    fn explode_rightward(&mut self, value: i64) {
        if self.is_regular() {
            let sum = self.unwrap_regular() + value;
            *self = Number::Regular(sum);
        } else {
            let (lhs, _) = self.unwrap_pair();
            lhs.explode_rightward(value);
        }
    }

    fn explode_leftward(&mut self, value: i64) {
        if self.is_regular() {
            let sum = self.unwrap_regular() + value;
            *self = Number::Regular(sum);
        } else {
            let (_, rhs) = self.unwrap_pair();
            rhs.explode_leftward(value)
        }
    }

    fn is_regular(&self) -> bool {
        match self {
            Number::Regular(_) => true,
            _ => false,
        }
    }

    fn unwrap_pair(&mut self) -> (&mut Number, &mut Number) {
        match self {
            Number::Pair(a, b) => (a, b),
            _ => panic!("Not a pair"),
        }
    }

    fn unwrap_regular(&self) -> i64 {
        match self {
            Number::Regular(n) => *n,
            _ => panic!("Not a regular"),
        }
    }
}

impl std::fmt::Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self {
            Number::Pair(lhs, rhs) => {
                f.write_char('[')?;
                lhs.fmt(f)?;
                f.write_char(',')?;
                rhs.fmt(f)?;
                f.write_char(']')
            }
            Number::Regular(n) => write!(f, "{}", n),
        }
    }
}

fn parse_pairs(pairs_str: &str) -> Number {
    let mut chars = pairs_str.trim().chars().rev().collect::<Vec<_>>();
    let mut working_stack = Vec::new();

    while let Some(c) = chars.pop() {
        match c {
            '[' => {}
            '0'..='9' => {
                let num = c.to_digit(10).unwrap() as i64;
                working_stack.push(Box::new(Number::Regular(num)))
            }
            ']' => {
                if working_stack.len() < 2 {
                    break;
                }
                let rhs = working_stack.pop().unwrap();
                let lhs = working_stack.pop().unwrap();
                working_stack.push(Box::new(Number::Pair(lhs, rhs)));
            }
            ',' => {}
            _ => panic!("Unknown character!"),
        }
    }

    *working_stack.pop().unwrap()
}

fn add_numbers(lhs: Number, rhs: Number) -> Number {
    let mut result = Number::Pair(Box::new(lhs), Box::new(rhs));
    while result.reduce() {}
    result
}

#[aoc_generator(day18)]
fn fish_math(input: &str) -> Vec<Number> {
    input.lines().map(parse_pairs).collect()
}

#[aoc(day18, part1)]
fn part1(numbers: &[Number]) -> i64 {
    let mut result = numbers[0].clone();
    for num in &numbers[1..] {
        result = add_numbers(result, num.clone());
    }
    result.magnitude()
}

#[aoc(day18, part2)]
fn part2(numbers: &[Number]) -> i64 {
    let mut max_magnitude = i64::MIN;
    for i in 0..numbers.len() {
        for j in 0..numbers.len() {
            if i == j {
                continue;
            }

            let mag = add_numbers(numbers[i].clone(), numbers[j].clone()).magnitude();
            if mag > max_magnitude {
                max_magnitude = mag;
            }
        }
    }
    max_magnitude
}

#[cfg(test)]
mod test {
    use super::*;

    fn result(input: &str) -> String {
        let nums = fish_math(input);
        let mut result = nums[0].clone();
        for num in &nums[1..] {
            println!("\n\nAdd: {}, {}", result, num);
            result = add_numbers(result, num.clone());
        }

        result.to_string()
    }

    #[test]
    fn small_examples() {
        assert_eq!(
            result("[1,1]\n[2,2]\n[3,3]\n[4,4]"),
            "[[[[1,1],[2,2]],[3,3]],[4,4]]"
        );
        assert_eq!(
            result("[1,1]\n[2,2]\n[3,3]\n[4,4]\n[5,5]"),
            "[[[[3,0],[5,3]],[4,4]],[5,5]]"
        );
        assert_eq!(
            result("[1,1]\n[2,2]\n[3,3]\n[4,4]\n[5,5]\n[6,6]"),
            "[[[[5,0],[7,4]],[5,5]],[6,6]]"
        );
        assert_eq!(
            result("[[[[4,3],4],4],[7,[[8,4],9]]]\n[1,1]"),
            "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"
        );
        assert_eq!(
            result("[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]\n[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]"),
            "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]"
        );
        assert_eq!(result(
            "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]\n[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]"),
            "[[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]]");

        //[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
        //[7,[5,[[3,8],[1,4]]]]
        //[[2,[2,2]],[8,[8,1]]]
        //[2,9]
        //[1,[[[9,3],9],[[9,0],[0,7]]]]
        //[[[5,[7,4]],7],1]
        //[[[[4,2],2],6],[8,7]]
    }

    #[test]
    fn example() {
        let input = r"[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";
        assert_eq!(
            result(&input),
            "[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]"
        );

        assert_eq!(part1(&fish_math(input)), 4140);
        assert_eq!(part2(&fish_math(input)), 3993);
    }
}
