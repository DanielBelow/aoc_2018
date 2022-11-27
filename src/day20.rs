use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::collections::HashMap;

fn update_position(c: char, x: isize, y: isize) -> (isize, isize) {
    match c {
        'N' => (x, y - 1),
        'E' => (x + 1, y),
        'S' => (x, y + 1),
        'W' => (x - 1, y),
        _ => unreachable!(),
    }
}

#[aoc_generator(day20)]
pub fn generate(inp: &str) -> HashMap<(isize, isize), usize> {
    let (mut x, mut y) = (0isize, 0isize);
    let (mut prev_x, mut prev_y) = (x, y);
    let mut pos = vec![];

    let mut result: HashMap<(isize, isize), usize> = HashMap::new();

    for c in inp.chars().dropping(1).dropping_back(1) {
        match c {
            '(' => pos.push((x, y)),
            ')' => (x, y) = pos.pop().unwrap(),
            '|' => (x, y) = *pos.last().unwrap(),
            _ => {
                (x, y) = update_position(c, x, y);

                let prev_cost = *result.get(&(prev_x, prev_y)).unwrap_or(&0);
                if let Some(cur_cost) = result.get_mut(&(x, y)) {
                    *cur_cost = (*cur_cost).min(prev_cost + 1);
                } else {
                    result.insert((x, y), prev_cost + 1);
                }
            }
        }

        prev_x = x;
        prev_y = y;
    }

    result
}

#[aoc(day20, part1)]
pub fn part1(distances: &HashMap<(isize, isize), usize>) -> Option<usize> {
    distances.values().copied().max()
}

#[aoc(day20, part2)]
pub fn part2(distances: &HashMap<(isize, isize), usize>) -> usize {
    distances.values().filter(|it| **it >= 1_000).count()
}
