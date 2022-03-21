use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::collections::HashSet;

#[aoc_generator(day01)]
pub fn generate(inp: &str) -> Vec<i64> {
    inp.lines().filter_map(|it| it.parse().ok()).collect_vec()
}

#[aoc(day01, part1)]
pub fn part1(inp: &[i64]) -> i64 {
    inp.iter().sum()
}

#[aoc(day01, part2)]
pub fn part2(inp: &[i64]) -> i64 {
    let mut visited = HashSet::new();

    let mut cur = 0;
    visited.insert(cur);

    for value in inp.iter().cycle() {
        cur += *value;

        if visited.contains(&cur) {
            return cur;
        }

        visited.insert(cur);
    }

    unreachable!("No duplicate found")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_p1() {
        let data = generate("+1\n-2\n+3\n+1");
        let res = part1(&data);
        assert_eq!(res, 3);
    }

    #[test]
    fn test_sample_p2() {
        let test_data = &[("+1\n-1", 0i64), ("+3\n+3\n+4\n-2\n-4", 10)];
        for &(inp, expected) in test_data {
            let data = generate(inp);
            let res = part2(&data);
            assert_eq!(res, expected);
        }
    }
}
