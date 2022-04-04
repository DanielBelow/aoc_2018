use aoc_runner_derive::aoc;

fn run_program(r5: usize) -> usize {
    (1..=r5).filter(|it| r5 % *it == 0).sum()
}

#[aoc(day19, part1)]
pub fn part1(_: &str) -> usize {
    run_program(864)
}

#[aoc(day19, part2)]
pub fn part2(_: &str) -> usize {
    run_program(10_551_264)
}
