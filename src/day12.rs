use aoc_runner_derive::{aoc, aoc_generator};
use itertools::{FoldWhile, Itertools};
use std::collections::HashSet;

#[derive(Copy, Clone, Debug)]
pub struct Rule {
    pattern: [char; 5],
}

pub struct Input {
    initial_state: HashSet<i64>,
    rules: Vec<Rule>,
}

fn parse_initial_state(line: &str) -> Option<HashSet<i64>> {
    let state = line.strip_prefix("initial state: ")?;

    let initial_state = state
        .chars()
        .enumerate()
        .filter_map(|(idx, it)| {
            if it == '#' {
                #[allow(clippy::cast_possible_wrap)]
                Some(idx as i64)
            } else {
                None
            }
        })
        .collect::<HashSet<_>>();

    Some(initial_state)
}

fn parse_rule(line: &str) -> Option<Rule> {
    let mut spl = line.split(" => ");
    let inp = spl.next()?;
    let out = spl.next()?;

    if out == "#" {
        let pattern = inp.chars().collect_vec().try_into().ok()?;
        Some(Rule { pattern })
    } else {
        None
    }
}

#[aoc_generator(day12)]
pub fn generate(inp: &str) -> Option<Input> {
    let mut lines = inp.lines().filter(|it| !it.is_empty());
    let initial_state = parse_initial_state(lines.next()?)?;
    let rules = lines.filter_map(parse_rule).collect();

    Some(Input {
        initial_state,
        rules,
    })
}

fn simulate(cur_state: &HashSet<i64>, rules: &[Rule]) -> Option<HashSet<i64>> {
    const WINDOW: [i64; 5] = [-2, -1, 0, 1, 2];

    assert!(!cur_state.is_empty());

    let mut result = HashSet::new();

    let start = *cur_state.iter().min()?;
    let end = *cur_state.iter().max()?;

    for idx in (start - 3)..=(end + 3) {
        let window = WINDOW.map(|it| {
            let target = idx + it;
            if cur_state.contains(&target) {
                '#'
            } else {
                '.'
            }
        });

        if rules.iter().any(|it| it.pattern == window) {
            result.insert(idx);
        }
    }

    Some(result)
}

fn simulate_state(state: HashSet<i64>, rules: &[Rule], generations: i64) -> HashSet<i64> {
    (0..generations)
        .fold_while(state, |acc, _| {
            simulate(&acc, rules).map_or_else(|| FoldWhile::Done(acc), FoldWhile::Continue)
        })
        .into_inner()
}

#[aoc(day12, part1)]
pub fn part1(inp: &Input) -> i64 {
    let state = simulate_state(inp.initial_state.clone(), &inp.rules, 20);
    state.iter().sum()
}

#[aoc(day12, part2)]
pub fn part2(inp: &Input) -> i64 {
    const STABLE_AFTER: i64 = 100;
    const CHANGE: i64 = 50;

    // stable after 100 generations, increasing by 50 each step
    let state = simulate_state(inp.initial_state.clone(), &inp.rules, STABLE_AFTER);
    state.iter().sum::<i64>() + (50_000_000_000 - STABLE_AFTER) * CHANGE
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "initial state: #..#.#..##......###...###\n\
\n\
                              ...## => #\n\
                              ..#.. => #\n\
                              .#... => #\n\
                              .#.#. => #\n\
                              .#.## => #\n\
                              .##.. => #\n\
                              .#### => #\n\
                              #.#.# => #\n\
                              #.### => #\n\
                              ##.#. => #\n\
                              ##.## => #\n\
                              ###.. => #\n\
                              ###.# => #\n\
                              ####. => #";

    #[test]
    fn test_sample_p1() {
        let Some(data) = generate(TEST_INPUT) else {
            panic!("Could not parse test input")
        };

        let res = part1(&data);
        assert_eq!(res, 325);
    }
}
