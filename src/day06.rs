use aoc_runner_derive::{aoc, aoc_generator};
use itertools::{iproduct, Itertools};
use parse_display::{Display, FromStr};
use std::collections::HashSet;

#[derive(Display, FromStr, Hash, Eq, PartialEq, Copy, Clone, Debug)]
#[display("{x}, {y}")]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    const fn manhatten_distance(self, x: i32, y: i32) -> i32 {
        let x_diff = x - self.x;
        let y_diff = y - self.y;
        x_diff.abs() + y_diff.abs()
    }
}

#[aoc_generator(day06)]
pub fn generate(inp: &str) -> Vec<Point> {
    inp.lines().filter_map(|it| it.parse().ok()).collect()
}

#[aoc(day06, part1)]
pub fn part1(points: &[Point]) -> Option<usize> {
    let (min_x, max_x) = points.iter().map(|it| it.x).minmax().into_option()?;
    let (min_y, max_y) = points.iter().map(|it| it.y).minmax().into_option()?;

    let is_on_edge =
        |x: i32, y: i32| -> bool { min_x == x || max_x == x || min_y == y || max_y == y };

    let mut infinite_regions = HashSet::new();
    let mut single_points = Vec::new();

    for (x, y) in iproduct!(min_x..=max_x, min_y..=max_y) {
        let closest = points
            .iter()
            .map(|p| (p, p.manhatten_distance(x, y)))
            .sorted_by_key(|(_, d)| *d)
            .take(2)
            .collect_vec();

        let (first_pt, first_dist) = closest[0];
        let (_, second_dist) = closest[1];

        if is_on_edge(x, y) {
            infinite_regions.insert(*first_pt);
        }

        if first_dist != second_dist {
            single_points.push(*first_pt);
        }
    }

    let counts = single_points
        .iter()
        .filter(|&it| !infinite_regions.contains(it))
        .counts();

    counts.values().max().copied()
}

#[aoc(day06, part2)]
pub fn part2(points: &[Point]) -> Option<usize> {
    let (min_x, max_x) = points.iter().map(|it| it.x).minmax().into_option()?;
    let (min_y, max_y) = points.iter().map(|it| it.y).minmax().into_option()?;

    Some(
        iproduct!(min_x..=max_x, min_y..=max_y)
            .map(|(x, y)| {
                points
                    .iter()
                    .map(|p| p.manhatten_distance(x, y))
                    .sum::<i32>()
            })
            .filter(|it| *it < 10_000)
            .count(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "1, 1\n\
                              1, 6\n\
                              8, 3\n\
                              3, 4\n\
                              5, 5\n\
                              8, 9";

    #[test]
    fn test_sample_p1() {
        let data = generate(TEST_INPUT);
        let res = part1(&data);
        assert_eq!(res, Some(17));
    }
}
