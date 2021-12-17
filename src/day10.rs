#[derive(Debug)]
pub enum SyntaxError {
    Corrupt(usize, char, char),
    Incomplete(Chunk),
}

impl SyntaxError {
    pub fn incomplete(chunk: Chunk) -> Result<Chunk, Self> {
        Err(SyntaxError::Incomplete(chunk))
    }

    pub fn corrupt(col: usize, act: char, exp: char) -> Result<Chunk, Self> {
        Err(SyntaxError::Corrupt(col, act, exp))
    }
}

pub struct Chunk {
    pub opening: char,
    pub child: Vec<Chunk>,
    pub closing: Option<char>,
}

impl std::fmt::Debug for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.opening)?;
        for chunk in self.child.iter() {
            chunk.fmt(f)?;
        }
        if let Some(closed) = self.closing {
            write!(f, "{}", closed)?;
        }

        Ok(())
    }
}

pub fn get_corrupt_score(c: char) -> u64 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("Unexpected character ({})", c),
    }
}

pub fn get_ac_score(c: char) -> u64 {
    match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => panic!("Unexpected character ({})", c),
    }
}
pub fn is_open(tok: char) -> bool {
    matches!(tok, '(' | '[' | '{' | '<')
}

fn closing_for(tok: char) -> char {
    match tok {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        t => panic!("Unknown token: {}", t),
    }
}

impl Chunk {
    pub fn opens_with(opening: char) -> Self {
        assert!(
            is_open(opening),
            "Chunk not opening with opening token: {}",
            opening
        );

        Chunk {
            opening,
            child: vec![],
            closing: None,
        }
    }

    pub fn close_with(&mut self, c: char) {
        self.closing = Some(c);
    }

    pub fn add_child(&mut self, child: Chunk) {
        self.child.push(child);
    }

    pub fn get_missing(&self) -> Vec<char> {
        let mut missing = self
            .child
            .iter()
            .map(|child| child.get_missing())
            .flatten()
            .collect::<Vec<_>>();

        if self.closing.is_none() {
            missing.push(closing_for(self.opening));
        }

        missing
    }
}

pub struct ChunkParser<'a> {
    line: &'a str,
    col: usize,
}

impl<'a> ChunkParser<'a> {
    pub fn parse(line: &str) -> Result<Chunk, SyntaxError> {
        let mut parser = ChunkParser::with_input(line);
        parser.parse_chunks()
    }

    fn with_input(line: &'a str) -> Self {
        ChunkParser { line, col: 0 }
    }

    fn parse_chunks(&mut self) -> Result<Chunk, SyntaxError> {
        let car = self.consume().expect("Parsing empty input");
        if !is_open(car) {
            return SyntaxError::corrupt(self.col, car, 'o');
        }

        let mut chunk = Chunk::opens_with(car);
        while !self.done() {
            if let Some(next) = self.peek() {
                let closed = closing_for(chunk.opening);

                if is_open(next) {
                    let chunk_or_err = self.parse_chunks();
                    match chunk_or_err {
                        Ok(child) => chunk.add_child(child),
                        Err(SyntaxError::Incomplete(child)) => {
                            chunk.add_child(child);
                            return SyntaxError::incomplete(chunk);
                        }
                        err => return err,
                    }
                } else if next == closed {
                    self.consume();
                    chunk.close_with(next);
                    return Ok(chunk);
                } else {
                    return SyntaxError::corrupt(self.col, next, closed);
                }
            } else {
                return SyntaxError::incomplete(chunk);
            }
        }

        SyntaxError::incomplete(chunk)
    }

    fn peek(&self) -> Option<char> {
        self.line.chars().nth(self.col)
    }

    fn consume(&mut self) -> Option<char> {
        let c = self.peek();
        self.col += 1;
        c
    }

    fn done(&self) -> bool {
        self.col >= self.line.len()
    }
}

#[aoc_generator(day10)]
fn program<'a>(input: &str) -> Vec<String> {
    input.lines().map(|s| s.trim().to_string()).collect()
}

#[aoc(day10, part1)]
fn part1(lines: &[String]) -> u64 {
    lines
        .iter()
        .map(|line| ChunkParser::parse(line))
        .filter_map(|chunk_or_err| match chunk_or_err {
            Err(SyntaxError::Corrupt(_, act, _)) => Some(act),
            _ => None,
        })
        .map(get_corrupt_score)
        .sum()
}

#[aoc(day10, part2)]
fn part2(lines: &[String]) -> u64 {
    let mut ac_scores = lines
        .iter()
        .map(|line| ChunkParser::parse(line))
        .filter_map(|chunk_or_err| match chunk_or_err {
            Err(SyntaxError::Incomplete(chunk)) => Some(chunk),
            _ => None,
        })
        .map(|chunk| {
            chunk
                .get_missing()
                .into_iter()
                .fold(0, |acc, closing| 5 * acc + get_ac_score(closing))
        })
        .collect::<Vec<_>>();

    ac_scores.sort();
    ac_scores[ac_scores.len() / 2]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let input = program(
            r"[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]",
        );
        assert_eq!(part1(&input), 26397);
        assert_eq!(part2(&input), 288957);
    }
}
