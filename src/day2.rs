enum Command {
    Up(u32),
    Down(u32),
    Forward(u32),
}

struct Position {
    horiz: u32,
    depth: u32,
    aim: u32,
}

struct ParseCommandError;
impl std::str::FromStr for Command {
    type Err = ParseCommandError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let command = input.trim().split(' ').collect::<Vec<_>>();

        match command.len() {
            2 => {
                let amount: u32 = command[1].parse().or_else(|_| Err(ParseCommandError))?;
                match command[0] {
                    "up" => Ok(Command::Up(amount)),
                    "down" => Ok(Command::Down(amount)),
                    "forward" => Ok(Command::Forward(amount)),
                    _ => Err(ParseCommandError),
                }
            }
            _ => Err(ParseCommandError),
        }
    }
}

impl Position {
    pub fn new() -> Self {
        Position {
            horiz: 0,
            depth: 0,
            aim: 0,
        }
    }

    pub fn execute_command(mut self, command: &Command) -> Self {
        match &command {
            Command::Up(x) => self.depth -= x,
            Command::Down(x) => self.depth += x,
            Command::Forward(x) => self.horiz += x,
        }
        self
    }

    pub fn execute_command_with_aim(mut self, command: &Command) -> Self {
        match &command {
            Command::Up(x) => self.aim -= x,
            Command::Down(x) => self.aim += x,
            Command::Forward(x) => {
                self.horiz += x;
                self.depth += x * self.aim;
            }
        }
        self
    }
}

#[aoc_generator(day2)]
fn commands(input: &str) -> Vec<Command> {
    input
        .lines()
        .filter_map(|command| command.parse().ok())
        .collect()
}

#[aoc(day2, part1)]
fn part1(commands: &[Command]) -> u32 {
    let pos = commands
        .iter()
        .fold(Position::new(), Position::execute_command);

    pos.horiz * pos.depth
}

#[aoc(day2, part2)]
fn part2(commands: &[Command]) -> u32 {
    let pos = commands
        .iter()
        .fold(Position::new(), Position::execute_command_with_aim);

    pos.horiz * pos.depth
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let input = r"forward 5
                      down 5
                      forward 8
                      up 3
                      down 8
                      forward 2";

        assert_eq!(part1(&commands(input)), 150);
        assert_eq!(part2(&commands(input)), 900);
    }
}
