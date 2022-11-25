use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day17)]
pub fn generate(_: &str) -> String {
    include_str!("../input/2018/day17_filled.txt").to_string()
}

#[aoc(day17, part1)]
pub fn part1(inp: &str) -> usize {
    inp.chars().filter(|it| *it == '~' || *it == '|').count()
}

#[aoc(day17, part2)]
pub fn part2(inp: &str) -> usize {
    inp.chars().filter(|it| *it == '~').count()
}
