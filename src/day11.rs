#![allow(clippy::trivially_copy_pass_by_ref)]

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::iproduct;
use std::fmt::{Display, Formatter};

const GRID_SIZE: usize = 300;

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub struct Coords {
    x: usize,
    y: usize,
    size: usize,
    value: i64,
}

impl Display for Coords {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{},{} with size {} = {}",
            self.x, self.y, self.size, self.value
        )
    }
}

#[aoc_generator(day11)]
pub fn generate(inp: &str) -> Option<i64> {
    inp.parse().ok()
}

fn hundreds_digit(num: i64) -> i64 {
    (num / 100) % 10
}

fn compute_power_level(serial_number: i64, x: i64, y: i64) -> i64 {
    let rack_id = x + 10;

    let mut power_level = rack_id * y;
    power_level += serial_number;
    power_level *= rack_id;
    power_level = hundreds_digit(power_level);
    power_level -= 5;

    power_level
}

fn compute_square_sum(grid: &[Vec<i64>], x: usize, y: usize, size: usize) -> i64 {
    grid[y][x] + grid[y + size][x + size] - grid[y][x + size] - grid[y + size][x]
}

fn get_summed_area_table(serial_number: i64) -> Vec<Vec<i64>> {
    let mut grid = vec![vec![0; GRID_SIZE]; GRID_SIZE];

    for (x, y) in iproduct!(0..GRID_SIZE, 0..GRID_SIZE) {
        let power_level = compute_power_level(serial_number, x as i64, y as i64);

        let a = if y > 0 { grid[y - 1][x] } else { 0 };
        let b = if x > 0 { grid[y][x - 1] } else { 0 };
        let c = if x > 0 && y > 0 {
            grid[y - 1][x - 1]
        } else {
            0
        };

        grid[y][x] = power_level + a + b - c;
    }

    grid
}

fn find_largest_square_sized(grid: &[Vec<i64>], size: usize) -> Option<Coords> {
    iproduct!(1..GRID_SIZE - size, 1..GRID_SIZE - size)
        .map(|(x, y)| {
            let value = compute_square_sum(grid, x - 1, y - 1, size);
            Coords { x, y, size, value }
        })
        .max_by_key(|it| it.value)
}

fn find_largest_square(serial_number: i64) -> Option<Coords> {
    let grid = get_summed_area_table(serial_number);

    (1..GRID_SIZE)
        .filter_map(|it| find_largest_square_sized(&grid, it))
        .max_by_key(|it| it.value)
}

#[aoc(day11, part1)]
pub fn part1(serial_number: &i64) -> Option<Coords> {
    let grid = get_summed_area_table(*serial_number);
    find_largest_square_sized(&grid, 3)
}

#[aoc(day11, part2)]
pub fn part2(serial_number: &i64) -> Option<Coords> {
    find_largest_square(*serial_number)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_hundreds_digit() {
        assert_eq!(hundreds_digit(256), 2);
        assert_eq!(hundreds_digit(599_455), 4);
        assert_eq!(hundreds_digit(23), 0);
    }

    #[test]
    fn test_sample_p1_1() {
        let serial_number = 18;
        assert_eq!(
            part1(&serial_number),
            Some(Coords {
                x: 33,
                y: 45,
                size: 3,
                value: 29
            })
        );
    }

    #[test]
    fn test_sample_p1_2() {
        let serial_number = 42;
        assert_eq!(
            part1(&serial_number),
            Some(Coords {
                x: 21,
                y: 61,
                size: 3,
                value: 30
            })
        );
    }

    #[test]
    fn test_samples_p2_1() {
        let serial_number = 18;
        assert_eq!(
            part2(&serial_number),
            Some(Coords {
                x: 90,
                y: 269,
                size: 16,
                value: 113
            })
        );
    }

    #[test]
    fn test_samples_p2_2() {
        let serial_number = 42;
        assert_eq!(
            part2(&serial_number),
            Some(Coords {
                x: 232,
                y: 251,
                size: 12,
                value: 119
            })
        );
    }
}
