use std::fmt::Write;

#[derive(Clone)]
enum BingoTile {
    Unmarked(u32),
    Marked,
}

impl BingoTile {
    pub fn with_num(num: u32) -> Self {
        BingoTile::Unmarked(num)
    }

    pub fn marked() -> Self {
        BingoTile::Marked
    }

    pub fn is_marked(&self) -> bool {
        match &self {
            BingoTile::Marked => true,
            _ => false,
        }
    }

    pub fn is_num(&self, num: u32) -> bool {
        match &self {
            BingoTile::Unmarked(v) => *v == num,
            _ => false,
        }
    }

    pub fn value(&self) -> u32 {
        match &self {
            BingoTile::Unmarked(v) => *v,
            _ => panic!("Called value on Marked tile!"),
        }
    }
}

#[derive(Clone)]
struct BingoBoard {
    tiles: Vec<BingoTile>,
    size: usize,
    won: bool,
}

impl BingoBoard {
    pub fn with_tiles(nums: &[u32]) -> Self {
        BingoBoard {
            tiles: nums.iter().map(|&n| BingoTile::with_num(n)).collect(),
            size: (nums.len() as f64).sqrt() as usize,
            won: false,
        }
    }

    pub fn is_winner(&self) -> bool {
        self.won
    }

    pub fn mark(&mut self, num: u32) -> bool {
        if self.won {
            return false;
        }

        if let Some((pos, tile)) = self
            .tiles
            .iter_mut()
            .enumerate()
            .find(|(_, tile)| tile.is_num(num))
        {
            *tile = BingoTile::marked();

            // Check for winning row/tile at this location
            let row_win = || {
                let row_start = (pos / self.size) * self.size;
                self.tiles[row_start..(row_start + self.size)]
                    .iter()
                    .all(BingoTile::is_marked)
            };

            let col_win = || {
                let col_pos = pos % self.size;
                self.tiles
                    .chunks(self.size)
                    .fold(true, |wins, row| wins && row[col_pos].is_marked())
            };

            self.won = row_win() || col_win();
            return self.won;
        }

        false
    }

    pub fn unmarked_sum(&self) -> u32 {
        self.tiles
            .iter()
            .filter(|tile| !tile.is_marked())
            .map(BingoTile::value)
            .sum()
    }
}

impl std::fmt::Debug for BingoTile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&match &self {
            BingoTile::Marked => " X ".to_string(),
            BingoTile::Unmarked(v) => format!("{:2} ", v),
        })
    }
}

impl std::fmt::Debug for BingoBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.tiles.chunks(self.size).for_each(|row| {
            row.iter().for_each(|tile| {
                tile.fmt(f).unwrap();
            });
            f.write_char('\n').unwrap();
        });
        Ok(())
    }
}

fn parse_row(row: &str) -> Vec<u32> {
    row.split(' ')
        .filter_map(|n| n.parse::<u32>().ok())
        .collect()
}

#[aoc_generator(day4)]
fn bingo(input: &str) -> (Vec<u32>, Vec<BingoBoard>) {
    let mut lines = input.lines();
    let draws = lines
        .next()
        .expect("Missing line")
        .split(',')
        .filter_map(|c| c.parse::<u32>().ok())
        .collect();

    if let Some(line) = lines.next() {
        assert!(line.is_empty());
    }

    let mut boards = Vec::new();
    while let Some(line) = lines.next() {
        let mut tiles = Vec::new();
        tiles.append(&mut parse_row(line));

        while let Some(line) = lines.next() {
            if line.is_empty() {
                break;
            }
            tiles.append(&mut parse_row(line));
        }

        boards.push(BingoBoard::with_tiles(&tiles));
    }

    (draws, boards)
}

fn call_num<'a>(num: u32, boards: &'a mut [BingoBoard]) -> Option<BingoBoard> {
    boards
        .iter_mut()
        .filter(|b| !b.is_winner())
        .fold(None, |winner, board| {
            let won = board.mark(num);
            if won && winner.is_none() {
                return Some(board.clone());
            }
            winner
        })
}

fn win_bingo<'a>(nums: &[u32], boards: &'a mut [BingoBoard]) -> (u32, BingoBoard) {
    for &num in nums {
        if let Some(winner) = call_num(num, boards) {
            return (num, winner);
        }
    }

    panic!("No boards won!");
}

fn lose_bingo<'a>(nums: &[u32], boards: &'a mut [BingoBoard]) -> (u32, BingoBoard) {
    for &num in nums {
        if let Some(winner) = call_num(num, boards) {
            if boards.iter().all(|b| b.is_winner()) {
                return (num, winner);
            }
        }
    }

    panic!("No boards lose????!");
}

#[aoc(day4, part1)]
fn part1((nums, boards): &(Vec<u32>, Vec<BingoBoard>)) -> u32 {
    let mut boards: Vec<_> = boards.to_vec();
    let (winning_num, winning_board) = win_bingo(nums, &mut boards);
    winning_num * winning_board.unmarked_sum()
}

#[aoc(day4, part2)]
fn part2((nums, boards): &(Vec<u32>, Vec<BingoBoard>)) -> u32 {
    let mut boards: Vec<_> = boards.to_vec();
    let (losing_num, losing_board) = lose_bingo(nums, &mut boards);
    losing_num * losing_board.unmarked_sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let input = bingo(
            r"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
            ",
        );

        assert_eq!(part1(&input), 4512);
        assert_eq!(part2(&input), 1924);
    }
}
