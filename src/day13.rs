#[derive(Debug)]
enum FoldDirection {
    Left,
    Up,
}

#[derive(Debug)]
struct FoldInstruction {
    line: usize,
    direction: FoldDirection,
}

#[derive(Debug, Clone)]
struct Paper {
    dots: Vec<(usize, usize)>,
}

impl Paper {
    fn fold(&mut self, instr: &FoldInstruction) {
        match instr.direction {
            FoldDirection::Up => self
                .dots
                .iter_mut()
                .filter(|(_, y)| *y > instr.line)
                .for_each(|(_, y)| *y = 2 * instr.line - *y),
            FoldDirection::Left => self
                .dots
                .iter_mut()
                .filter(|(x, _)| *x > instr.line)
                .for_each(|(x, _)| *x = 2 * instr.line - *x),
        }
        self.dots.sort_unstable();
        self.dots.dedup()
    }

    fn dots(&self) -> &[(usize, usize)] {
        &self.dots
    }
}

impl std::str::FromStr for FoldInstruction {
    type Err = std::string::ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut instruction = s
            .split(' ')
            .filter(|substr| substr.contains('='))
            .filter_map(|substr| substr.split_once('='))
            .map(|(dir_str, line_str)| {
                let direction = match dir_str {
                    "x" => FoldDirection::Left,
                    "y" => FoldDirection::Up,
                    _ => panic!("Unknown fold direction"),
                };
                let line = line_str.parse::<usize>().unwrap();
                FoldInstruction { direction, line }
            })
            .collect::<Vec<_>>();
        Ok(instruction.pop().unwrap())
    }
}

#[aoc_generator(day13)]
fn parse_instructions(input: &str) -> (Paper, Vec<FoldInstruction>) {
    let (dots_str, instructions_str): (Vec<_>, Vec<_>) =
        input.lines().partition(|s| s.contains(','));

    let dots = dots_str
        .iter()
        .filter_map(|coord_str| coord_str.split_once(','))
        .map(|(x, y)| (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()))
        .collect::<Vec<_>>();
    let instructions = instructions_str
        .iter()
        .skip(1)
        .filter_map(|line| line.parse::<FoldInstruction>().ok())
        .collect::<Vec<_>>();

    (Paper { dots }, instructions)
}

#[aoc(day13, part1)]
fn part1((paper, instructions): &(Paper, Vec<FoldInstruction>)) -> usize {
    let mut paper = paper.clone();
    paper.fold(&instructions[0]);
    paper.dots().len()
}

#[aoc(day13, part2)]
fn part2((paper, instructions): &(Paper, Vec<FoldInstruction>)) -> u32 {
    let mut paper = paper.clone();
    for inst in instructions.iter() {
        paper.fold(inst);
    }
    let max_x = paper.dots().iter().map(|&(x, _)| x).max().unwrap() as usize;
    let max_y = paper.dots().iter().map(|&(_, y)| y).max().unwrap() as usize;

    for y in 0..=max_y {
        for x in 0..=max_x {
            let c = if paper.dots().contains(&(x, y)) {
                '#'
            } else {
                '.'
            };
            print!("{}", c);
        }
        println!();
    }
    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let input = parse_instructions(
            r"6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5
",
        );
        assert_eq!(part1(&input), 17);
        //assert_eq!(part2(&input), 3509);
    }
}
