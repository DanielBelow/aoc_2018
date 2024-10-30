use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use parse_display::{Display, FromStr};
use std::collections::{HashMap, HashSet};

#[derive(Display, FromStr, PartialEq, Eq, Hash, Copy, Clone)]
#[display("<{x},{y},{z}>")]
pub struct Position {
    x: i64,
    y: i64,
    z: i64,
}

impl Position {
    const fn distance_to(&self, other: &Self) -> u64 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y) + self.z.abs_diff(other.z)
    }
}

#[derive(Display, FromStr, PartialEq, Eq, Hash, Copy, Clone)]
#[display("pos={pos}, r={range}")]
pub struct Bot {
    pos: Position,
    range: u64,
}

impl Bot {
    const fn is_in_range_of(&self, pos: &Position) -> bool {
        self.pos.distance_to(pos) <= self.range
    }

    const fn shares_points_with(&self, other: &Self) -> bool {
        self.pos.distance_to(&other.pos) <= self.range + other.range
    }
}

#[aoc_generator(day23)]
pub fn generate(inp: &str) -> Vec<Bot> {
    inp.lines()
        .filter_map(|it| it.parse::<Bot>().ok())
        .sorted_unstable_by_key(|it| it.range)
        .collect()
}

#[aoc(day23, part1)]
pub fn part1(bots: &[Bot]) -> Option<usize> {
    let largest_signal = bots.last()?;

    let bots_in_range = bots
        .iter()
        .filter(|it| largest_signal.is_in_range_of(&it.pos))
        .count();
    Some(bots_in_range)
}

struct BronKerbosch {
    neighbours: HashMap<Bot, HashSet<Bot>>,
}

impl BronKerbosch {
    fn execute(
        &mut self,
        p: &HashSet<Bot>,
        r: HashSet<Bot>,
        x: &HashSet<Bot>,
        best_result: &mut HashSet<Bot>,
    ) {
        if p.is_empty() && x.is_empty() {
            if r.len() > best_result.len() {
                *best_result = r;
            }
        } else {
            let most_neighbours_p_and_x = p
                .union(x)
                .max_by_key(|it| self.neighbours.get(*it).expect("Neighbour exists").len())
                .expect("Non-empty set");

            let p_without_neighbours = p
                .difference(
                    self.neighbours
                        .get(most_neighbours_p_and_x)
                        .expect("Neighbour exists"),
                )
                .copied()
                .collect_vec();

            for v in p_without_neighbours {
                let v_neighbours = self.neighbours.get(&v).expect("Neighbour exists");
                let mut r_with_v = r.clone();
                r_with_v.insert(v);
                self.execute(
                    &p.intersection(v_neighbours).copied().collect(),
                    r_with_v,
                    &x.intersection(v_neighbours).copied().collect(),
                    best_result,
                );
            }
        }
    }

    fn largest_clique(&mut self) -> HashSet<Bot> {
        let mut best_result = HashSet::new();

        self.execute(
            &self.neighbours.keys().copied().collect(),
            HashSet::new(),
            &HashSet::new(),
            &mut best_result,
        );

        best_result
    }
}

#[aoc(day23, part2)]
pub fn part2(bots: &[Bot]) -> Option<u64> {
    let neighbours = bots
        .iter()
        .map(|it| {
            (
                *it,
                bots.iter()
                    .filter(|b| *b != it && b.shares_points_with(it))
                    .copied()
                    .collect::<HashSet<_>>(),
            )
        })
        .collect::<HashMap<_, _>>();

    let mut bk = BronKerbosch { neighbours };
    let clique = bk.largest_clique();

    clique
        .iter()
        .map(|it| {
            it.pos
                .distance_to(&Position { x: 0, y: 0, z: 0 })
                .abs_diff(it.range)
        })
        .max()
}
