use aoc_runner_derive::{aoc, aoc_generator};
use itertools::{iproduct, Itertools};
use num_complex::Complex;
use std::collections::HashMap;

pub struct MapData {
    depth: usize,
    target: Complex<usize>,
}

fn parse_with_prefix(prefix: &str, line: Option<&str>) -> Option<usize> {
    line.and_then(|it| {
        it.strip_prefix(prefix)
            .and_then(|v| v.parse::<usize>().ok())
    })
}

fn parse_coords(prefix: &str, line: Option<&str>) -> Option<Complex<usize>> {
    let coords = line.and_then(|it| it.strip_prefix(prefix))?;
    if let [x, y] = coords.split(',').collect_vec()[..] {
        let x = x.parse::<usize>().ok()?;
        let y = y.parse::<usize>().ok()?;
        Some(Complex::new(x, y))
    } else {
        None
    }
}

#[aoc_generator(day22)]
pub fn generate(inp: &str) -> Option<MapData> {
    let mut lines = inp.lines();

    let depth = parse_with_prefix("depth: ", lines.next())?;
    let target = parse_coords("target: ", lines.next())?;

    Some(MapData { depth, target })
}

fn erosion_level(depth: usize, geological_index: usize) -> usize {
    (depth + geological_index) % 20_183
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Tile {
    Rocky,
    Wet,
    Narrow,
}

impl Tile {
    fn int_value(self) -> usize {
        match self {
            Tile::Rocky => 0,
            Tile::Wet => 1,
            Tile::Narrow => 2,
        }
    }
}

fn generate_map(data: &MapData) -> Vec<Vec<Tile>> {
    let mut geological_indices = HashMap::new();

    for (y, x) in iproduct!(0..=data.target.im, 0..=data.target.re) {
        let cur_index = if (y == 0 && x == 0) || (y == data.target.im && x == data.target.re) {
            0
        } else if y == 0 {
            x * 16_807
        } else if x == 0 {
            y * 48_271
        } else {
            let lhs_gi = geological_indices.get(&Complex::new(x - 1, y)).unwrap();
            let rhs_gi = geological_indices.get(&Complex::new(x, y - 1)).unwrap();

            erosion_level(data.depth, *lhs_gi) * erosion_level(data.depth, *rhs_gi)
        };

        geological_indices.insert(Complex::new(x, y), cur_index);
    }

    let erosion_levels = geological_indices
        .iter()
        .map(|(k, v)| (k, erosion_level(data.depth, *v) % 3))
        .collect::<HashMap<_, _>>();

    let rows = data.target.im + 1;
    let cols = data.target.re + 1;

    let mut result = vec![vec![Tile::Narrow; cols]; rows];

    for (y, x) in iproduct!(0..=data.target.im, 0..=data.target.re) {
        result[y][x] = match erosion_levels.get(&Complex::new(x, y)).unwrap() {
            0 => Tile::Rocky,
            1 => Tile::Wet,
            2 => Tile::Narrow,
            _ => unreachable!(),
        };
    }

    result
}

#[aoc(day22, part1)]
pub fn part1(data: &MapData) -> usize {
    generate_map(data)
        .iter()
        .flatten()
        .map(|it| it.int_value())
        .sum::<usize>()
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
enum Gear {
    None,
    ClimbingGear,
    Torch,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct State {
    position: Complex<usize>,
    gear: Gear,
}

fn can_enter_square(state: &State, other: Tile) -> bool {
    can_use_gear_in_region(state.gear, other)
}

fn can_use_gear_in_region(gear: Gear, region: Tile) -> bool {
    match region {
        // In rocky regions, you can use the climbing gear or the torch. You cannot use neither
        // => deny None
        Tile::Rocky => gear != Gear::None,
        // In wet regions, you can use the climbing gear or neither tool. You cannot use the torch
        // => deny Torch
        Tile::Wet => gear != Gear::Torch,
        // In narrow regions, you can use the torch or neither tool. You cannot use the climbing gear
        // => deny ClimbingGear
        Tile::Narrow => gear != Gear::ClimbingGear,
    }
}

fn get_gear_for_region(region: Tile) -> Vec<Gear> {
    match region {
        Tile::Rocky => vec![Gear::Torch, Gear::ClimbingGear],
        Tile::Wet => vec![Gear::None, Gear::ClimbingGear],
        Tile::Narrow => vec![Gear::None, Gear::Torch],
    }
}

fn add_moves(
    state: &State,
    cur_region: Tile,
    next_pos: Complex<usize>,
    map: &[Vec<Tile>],
    result: &mut Vec<(State, usize)>,
) {
    if next_pos.im >= map.len() || next_pos.re >= map[next_pos.im].len() {
        return;
    }

    let next_region = map[next_pos.im][next_pos.re];
    if can_enter_square(state, next_region) {
        result.push((
            State {
                position: next_pos,
                gear: state.gear,
            },
            1,
        ));
    } else {
        // we need to change gear, but there are multiple options
        // cost is 7 (switching gear) + 1
        for gear in get_gear_for_region(next_region) {
            if can_use_gear_in_region(gear, cur_region) {
                assert_ne!(gear, state.gear);
                result.push((
                    State {
                        position: next_pos,
                        gear,
                    },
                    8,
                ));
            }
        }
    }
}

fn get_successors(state: &State, map: &[Vec<Tile>]) -> Vec<(State, usize)> {
    let mut result = vec![];

    let cur_pos = state.position;
    let cur_region = map[cur_pos.im][cur_pos.re];

    // up
    if cur_pos.im > 0 {
        add_moves(
            state,
            cur_region,
            Complex::new(cur_pos.re, cur_pos.im - 1),
            map,
            &mut result,
        );
    }

    // left
    if cur_pos.re > 0 {
        add_moves(
            state,
            cur_region,
            Complex::new(cur_pos.re - 1, cur_pos.im),
            map,
            &mut result,
        );
    }

    // down
    add_moves(
        state,
        cur_region,
        Complex::new(cur_pos.re, cur_pos.im + 1),
        map,
        &mut result,
    );

    // right
    add_moves(
        state,
        cur_region,
        Complex::new(cur_pos.re + 1, cur_pos.im),
        map,
        &mut result,
    );

    result
}

#[aoc(day22, part2)]
pub fn part2(data: &MapData) -> usize {
    let bigger_data = MapData {
        depth: data.depth,
        target: Complex::new(data.target.re + 50, data.target.im + 50),
    };

    let map = generate_map(&bigger_data);

    let state = State {
        position: Complex::new(0, 0),
        gear: Gear::Torch,
    };

    if let Some((path, cost)) = pathfinding::prelude::dijkstra(
        &state,
        |it| get_successors(it, &map),
        |it| it.position == data.target,
    ) {
        // need a torch at the end
        let add_gear_swap = path.last().unwrap().gear != Gear::Torch;
        return cost + if add_gear_swap { 7 } else { 0 };
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_p1() {
        let data = MapData {
            depth: 510,
            target: Complex::new(10, 10),
        };

        let result = part1(&data);
        assert_eq!(result, 114);
    }
}
