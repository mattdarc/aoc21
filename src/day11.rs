use std::collections::VecDeque;

type Octopus = i16;

#[derive(Clone)]
pub struct OctopusBoard {
    octos: Vec<Vec<Octopus>>,
    flash_queue: VecDeque<(isize, isize)>,
    flashes: u64,
}

impl std::fmt::Debug for OctopusBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.octos.iter() {
            writeln!(f)?;
            for octo in row.iter() {
                write!(f, "{}", octo)?;
            }
        }

        Ok(())
    }
}

impl OctopusBoard {
    pub fn with_octopuses(octos: Vec<Vec<Octopus>>) -> Self {
        OctopusBoard {
            octos,
            flashes: 0,
            flash_queue: VecDeque::new(),
        }
    }

    pub fn flashes(&self) -> u64 {
        self.flashes
    }

    pub fn is_synchronized(&self) -> bool {
        self.octos.iter().flatten().all(|octo| *octo == 0)
    }

    pub fn step(&mut self) {
        // 1. Increase energy level of all octopuses by 1
        for row in 0..self.octos.len() {
            for col in 0..self.octos[row].len() {
                self.increment_octo(row as isize, col as isize);
            }
        }

        // 2. Flash all octopuses with an energy level >9. Adjacent octopuses flash
        while let Some((row, col)) = self.flash_queue.pop_front() {
            (-1..=1)
                .flat_map(|drow| (-1..=1).map(move |dcol| (row + drow, col + dcol)))
                .for_each(|(row, col)| self.energize_by_flash(row, col));
        }
    }

    fn out_of_bounds(&self, row: isize, col: isize) -> bool {
        row >= self.octos.len() as isize
            || row < 0
            || col >= self.octos[0].len() as isize
            || col < 0
    }

    fn already_flashed(&self, row: isize, col: isize) -> bool {
        self.octos[row as usize][col as usize] == 0
    }

    fn energize_by_flash(&mut self, row: isize, col: isize) {
        if self.out_of_bounds(row, col) || self.already_flashed(row, col) {
            return;
        }

        self.increment_octo(row, col);
    }

    fn increment_octo(&mut self, row: isize, col: isize) {
        let octo = &mut self.octos[row as usize][col as usize];
        if *octo == 9 {
            *octo = 0;
            self.flashes += 1;
            self.flash_queue.push_back((row, col));
        } else {
            *octo += 1;
        }
    }
}

#[aoc_generator(day11)]
fn octopuses(input: &str) -> OctopusBoard {
    let board = input
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as Octopus)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    OctopusBoard::with_octopuses(board)
}

const N_STEPS: usize = 100;

#[aoc(day11, part1)]
fn part1(octo_board: &OctopusBoard) -> u64 {
    let mut octo_board = octo_board.clone();
    for _ in 0..N_STEPS {
        octo_board.step();
    }

    octo_board.flashes()
}

#[aoc(day11, part2)]
fn part2(octo_board: &OctopusBoard) -> u64 {
    let mut octo_board = octo_board.clone();

    let mut num_steps = 0;
    while !octo_board.is_synchronized() {
        octo_board.step();
        num_steps += 1;
    }

    num_steps
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let input = octopuses(
            r"5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526",
        );
        assert_eq!(part1(&input), 1656);
        assert_eq!(part2(&input), 195);
    }
}
