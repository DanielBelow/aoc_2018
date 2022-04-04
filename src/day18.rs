use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

const MAP_SIZE: usize = 50;

#[derive(Hash, PartialEq, Eq, Copy, Clone)]
pub enum Tile {
    Open,
    Tree,
    Lumber,
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '.' => Self::Open,
            '|' => Self::Tree,
            '#' => Self::Lumber,
            _ => unreachable!("Invalid character for Tile"),
        }
    }
}

#[aoc_generator(day18)]
pub fn generate(inp: &str) -> Vec<Vec<Tile>> {
    inp.lines()
        .map(|line| line.chars().map(Tile::from).collect())
        .collect()
}

fn collect_surrounding(x: usize, y: usize, current: &[Vec<Tile>]) -> Vec<Tile> {
    let mut result = Vec::new();

    let take_left = x > 0;
    let take_right = x + 1 < MAP_SIZE;

    // row above
    if let Some(py) = y.checked_sub(1) {
        let row = &current[py];
        if take_left {
            result.push(row[x - 1]);
        }

        result.push(row[x]);

        if take_right {
            result.push(row[x + 1]);
        }
    }

    // same row left
    if take_left {
        result.push(current[y][x - 1]);
    }

    // same row right
    if take_right {
        result.push(current[y][x + 1]);
    }

    if let Some(row) = current.get(y + 1) {
        if take_left {
            result.push(row[x - 1]);
        }

        result.push(row[x]);

        if take_right {
            result.push(row[x + 1]);
        }
    }

    result
}

fn pick_new_tile(x: usize, y: usize, current: &[Vec<Tile>]) -> Tile {
    let old = current[y][x];
    let surrounding = collect_surrounding(x, y, current);

    match old {
        Tile::Open => {
            if count_tile(&surrounding, Tile::Tree) >= 3 {
                Tile::Tree
            } else {
                old
            }
        }
        Tile::Tree => {
            if count_tile(&surrounding, Tile::Lumber) >= 3 {
                Tile::Lumber
            } else {
                old
            }
        }
        Tile::Lumber => {
            if count_tile(&surrounding, Tile::Lumber) >= 1
                && count_tile(&surrounding, Tile::Tree) >= 1
            {
                old
            } else {
                Tile::Open
            }
        }
    }
}

fn simulate(current: &[Vec<Tile>]) -> Vec<Vec<Tile>> {
    let mut res = vec![vec![Tile::Open; MAP_SIZE]; MAP_SIZE];

    for y in 0..current.len() {
        for x in 0..current[y].len() {
            let new_tile = pick_new_tile(x, y, current);

            res[y][x] = new_tile;
        }
    }

    res
}

fn count_tile(current: &[Tile], tile: Tile) -> usize {
    current.iter().filter(|&&it| it == tile).count()
}

fn simulate_state(inp: &[Vec<Tile>], steps: usize, part2: bool) -> Vec<Tile> {
    let mut state = inp.to_vec();

    let mut seen = HashMap::new();

    for step in 0..steps {
        state = simulate(&state);

        if part2 {
            if seen.contains_key(&state) {
                // found a cycle
                let index = seen[&state];
                let repeat_len = step - index;
                // offset from the cycle beginning
                let num_steps = ((steps - index) % repeat_len) - 1;
                return simulate_state(&state, num_steps, part2);
            }

            seen.insert(state.clone(), step);
        }
    }

    state.iter().flatten().copied().collect()
}

#[aoc(day18, part1)]
pub fn part1(inp: &[Vec<Tile>]) -> usize {
    let final_state = simulate_state(inp, 10, false);

    let wooded = count_tile(&final_state, Tile::Tree);
    let lumber = count_tile(&final_state, Tile::Lumber);

    wooded * lumber
}

#[aoc(day18, part2)]
pub fn part2(inp: &[Vec<Tile>]) -> usize {
    let final_state = simulate_state(inp, 1_000_000_000, true);

    let wooded = count_tile(&final_state, Tile::Tree);
    let lumber = count_tile(&final_state, Tile::Lumber);

    wooded * lumber
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = ".#.#...|#.\n\
                              .....#|##|\n\
                              .|..|...#.\n\
                              ..|#.....#\n\
                              #.#|||#|#|\n\
                              ...#.||...\n\
                              .|....|...\n\
                              ||...#|.#|\n\
                              |.||||..|.\n\
                              ...#.|..|.";

    #[test]
    fn test_sample_p1() {
        let data = generate(TEST_INPUT);
        let res = part1(&data);
        assert_eq!(res, 1147);
    }
}
