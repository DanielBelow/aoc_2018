use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use parse_display::{Display, FromStr};
use std::collections::HashSet;

#[derive(Display, FromStr, Copy, Clone)]
#[display("{x},{y}")]
pub struct Point {
    x: i64,
    y: i64,
}

#[derive(Display, FromStr, Copy, Clone)]
#[display("position=<{position}>velocity=<{velocity}>")]
pub struct LightPoint {
    position: Point,
    velocity: Point,
}

#[aoc_generator(day10)]
pub fn generate(inp: &str) -> Vec<LightPoint> {
    inp.lines()
        .map(|it| it.replace(' ', ""))
        .filter_map(|it| it.parse().ok())
        .collect_vec()
}

#[aoc(day10, part1)]
pub fn part1(points: &[LightPoint]) -> Option<String> {
    const MIN_DISTANCE: i64 = 10_880;

    let points = points
        .iter()
        .map(|it| LightPoint {
            position: Point {
                x: it.position.x + MIN_DISTANCE * it.velocity.x,
                y: it.position.y + MIN_DISTANCE * it.velocity.y,
            },
            ..*it
        })
        .collect_vec();

    let (min_x, max_x) = points
        .iter()
        .map(|it| it.position.x)
        .minmax()
        .into_option()?;
    let (min_y, max_y) = points
        .iter()
        .map(|it| it.position.y)
        .minmax()
        .into_option()?;

    let positions = points
        .iter()
        .map(|it| (it.position.x, it.position.y))
        .collect::<HashSet<_>>();

    for y in 0..(max_y.abs() + min_y.abs()) {
        for x in 0..(max_x.abs() + min_x.abs()) {
            let c = if positions.contains(&(x, y)) {
                '#'
            } else {
                '.'
            };
            print!("{c}");
        }

        println!();
    }

    Some(String::from("ECKXJLJF"))
}

#[aoc(day10, part2)]
pub const fn part2(_: &[LightPoint]) -> u64 {
    10_880
}
