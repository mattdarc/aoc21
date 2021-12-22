use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Turn {
    Player1,
    Player2,
}

impl Turn {
    fn pass(&self) -> Self {
        match self {
            &Turn::Player1 => Turn::Player2,
            &Turn::Player2 => Turn::Player1,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct Player {
    position: u64,
    score: u64,
}

impl Player {
    fn starting_at(position: u64) -> Self {
        Player { position, score: 0 }
    }

    fn move_by(&mut self, roll: u64) {
        let next = (self.position + roll) % 10;
        if next == 0 {
            self.position = 10;
        } else {
            self.position = next;
        }
        self.score += self.position;
    }
}

/// Universe can be identified by the current scores and the turn of the player
#[derive(PartialEq, Eq, Hash, Clone)]
struct Universe {
    player1: Player,
    player2: Player,
    turn: Turn,
}

impl Universe {
    fn with_players(player1: Player, player2: Player) -> Self {
        Universe {
            player1,
            player2,
            turn: Turn::Player1,
        }
    }

    fn parallel_universe(&self) -> Self {
        let mut parallel_universe = self.clone();
        std::mem::swap(
            &mut parallel_universe.player1,
            &mut parallel_universe.player2,
        );
        parallel_universe.turn = self.turn.pass();
        parallel_universe
    }

    fn next_universe(&self, roll: u64) -> Self {
        let mut next_universe = self.clone();
        match &self.turn {
            &Turn::Player1 => {
                next_universe.player1.move_by(roll);
                next_universe.turn = Turn::Player2;
            }
            &Turn::Player2 => {
                next_universe.player2.move_by(roll);
                next_universe.turn = Turn::Player1;
            }
        }

        next_universe
    }
}

/// Vec of (roll, # of ways to get this roll)
fn generate_rolls() -> Vec<(u64, u64)> {
    let mut rolls = Vec::with_capacity(27);
    for d1 in 1..=3 {
        for d2 in 1..=3 {
            for d3 in 1..=3 {
                rolls.push(d1 + d2 + d3);
            }
        }
    }
    (3..=9)
        .map(|n| {
            (
                n as u64,
                rolls.iter().filter(|&roll| *roll == n).count() as u64,
            )
        })
        .collect::<Vec<_>>()
}

lazy_static! {
    static ref ROLLS: Vec<(u64, u64)> = generate_rolls();
}

/// Map of universes to the number of wins for (player1, player2)
type UniverseCache = HashMap<Universe, (u64, u64)>;

#[aoc_generator(day21)]
fn starting_positions(_input: &str) -> (u64, u64) {
    (8, 6)
}

/// Returns (player1_wins, player2_wins)
fn start_quantum_game(p1: u64, p2: u64) -> (u64, u64) {
    let mut universe_cache = UniverseCache::new();
    let universe = Universe::with_players(Player::starting_at(p1), Player::starting_at(p2));
    play_quantum_game(universe, (0, 0), &mut universe_cache)
}

/// Returns (player1_wins, player2_wins)
fn play_quantum_game(
    universe: Universe,
    previous_wins: (u64, u64),
    universe_cache: &mut UniverseCache,
) -> (u64, u64) {
    if let Some(wins) = universe_cache.get(&universe) {
        return *wins;
    }

    let (mut p1_win, mut p2_win) = previous_wins;
    for &(roll, times) in ROLLS.iter() {
        let next_universe = universe.next_universe(roll);

        let max_wins = 20;
        if next_universe.player1.score > max_wins {
            p1_win += times;
        } else if next_universe.player2.score > max_wins {
            p2_win += times;
        } else {
            let (next_p1_wins, next_p2_wins) =
                play_quantum_game(next_universe, previous_wins, universe_cache);
            p1_win += times * next_p1_wins;
            p2_win += times * next_p2_wins;
        }
    }
    // insert parallel universe, one where player2 and player1 are swapped
    universe_cache.insert(universe.parallel_universe(), (p2_win, p1_win));

    let wins = (p1_win, p2_win);
    universe_cache.insert(universe, wins);

    wins
}

/// Returns (loser_score, num_rolls)
fn play_game(p1: u64, p2: u64, mut die: impl Iterator<Item = u64>) -> (u64, u64) {
    let mut player1 = Player::starting_at(p1);
    let mut player2 = Player::starting_at(p2);
    for num_rolls in (3..).step_by(3) {
        let roll = die.next().unwrap();
        if num_rolls % 2 == 0 {
            player2.move_by(roll);
        } else {
            player1.move_by(roll);
        }

        let max_wins = 1000;
        if player1.score >= max_wins {
            return (player2.score, num_rolls);
        } else if player2.score >= max_wins {
            return (player1.score, num_rolls);
        }
    }

    unreachable!();
}

#[aoc(day21, part1)]
fn part1(&(p1, p2): &(u64, u64)) -> u64 {
    let rolls = (0..)
        .step_by(3)
        .zip((2..).step_by(3))
        .map(|(a, b)| (a..=b).map(|n| 1 + (n % 100)).sum::<u64>());

    let (loser, num_rolls) = play_game(p1, p2, rolls);
    loser * num_rolls
}

#[aoc(day21, part2)]
fn part2(&(p1, p2): &(u64, u64)) -> u64 {
    let (p1_wins, p2_wins) = start_quantum_game(p1, p2);
    p1_wins.max(p2_wins)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(part1(&(4, 8)), 739785);
        assert_eq!(part2(&(4, 8)), 444356092776315);
    }
}
