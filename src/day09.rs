use aoc_runner_derive::{aoc, aoc_generator};
use parse_display::{Display, FromStr};
use std::collections::VecDeque;

// 465 players; last marble is worth 71940 points
#[derive(Display, FromStr, Copy, Clone, Debug)]
#[display("{num_players} players; last marble is worth {last_marble} points")]
pub struct GameState {
    num_players: usize,
    last_marble: usize,
}

#[aoc_generator(day09)]
pub fn generate(inp: &str) -> Option<GameState> {
    inp.parse().ok()
}

#[aoc(day09, part1)]
pub fn part1(state: &GameState) -> Option<usize> {
    let mut player_scores = vec![0; state.num_players];

    let mut marbles = VecDeque::new();
    marbles.push_back(0);

    for marble in 1..=state.last_marble {
        if marble % 23 == 0 {
            marbles.rotate_right(7);
            player_scores[marble % state.num_players] +=
                marble + marbles.pop_back().unwrap_or_default();
            marbles.rotate_left(1);
        } else {
            marbles.rotate_left(1);
            marbles.push_back(marble);
        }
    }

    player_scores.iter().max().copied()
}

#[aoc(day09, part2)]
pub fn part2(state: &GameState) -> Option<usize> {
    let new_state = GameState {
        num_players: state.num_players,
        last_marble: state.last_marble * 100,
    };

    part1(&new_state)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_p1() {
        let state = GameState {
            num_players: 9,
            last_marble: 25,
        };

        let result = part1(&state);
        assert_eq!(result, Some(32));
    }
}
