use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use num_complex::Complex;
use pathfinding::prelude::AstarSolution;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Tile {
    Wall,
    Open,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum Kind {
    Elf,
    Goblin,
}

#[derive(Clone)]
pub struct Map {
    tiles: HashMap<Complex<i64>, Tile>,
    enemies: Vec<Enemy>,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Enemy {
    kind: Kind,
    hp: i64,
    ap: i64,
    position: Complex<i64>,
    is_dead: bool,
}

fn reading_order(lhs: &Complex<i64>, rhs: &Complex<i64>) -> Ordering {
    lhs.re.cmp(&rhs.re).then_with(|| lhs.im.cmp(&rhs.im))
}

impl Ord for Enemy {
    fn cmp(&self, other: &Self) -> Ordering {
        reading_order(&self.position, &other.position)
    }
}

impl PartialOrd<Self> for Enemy {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Enemy {
    fn elf(position: Complex<i64>) -> Self {
        Self::elf_with_ap(position, 3)
    }

    fn goblin(position: Complex<i64>) -> Self {
        Self {
            kind: Kind::Goblin,
            position,
            hp: 200,
            ap: 3,
            is_dead: false,
        }
    }

    fn elf_with_ap(position: Complex<i64>, ap: i64) -> Self {
        Self {
            kind: Kind::Elf,
            position,
            hp: 200,
            ap,
            is_dead: false,
        }
    }
}

fn adjacent_open_squares(
    position: Complex<i64>,
    map: &HashMap<Complex<i64>, Tile>,
    enemy_positions: &HashSet<Complex<i64>>,
) -> Vec<Complex<i64>> {
    const NEIGHBOURS: [Complex<i64>; 4] = [
        Complex::new(-1, 0),
        Complex::new(0, -1),
        Complex::new(0, 1),
        Complex::new(1, 0),
    ];

    NEIGHBOURS
        .iter()
        .map(|it| *it + position)
        .filter(|it| {
            map.get(it).map_or(false, |it| *it == Tile::Open) && !enemy_positions.contains(it)
        })
        .sorted_by(reading_order)
        .collect()
}

fn adjacent_enemies(position: Complex<i64>, kind: Kind, enemies: &mut [Enemy]) -> Vec<&mut Enemy> {
    enemies
        .iter_mut()
        .filter(|it| !it.is_dead && it.kind != kind && (it.position - position).l1_norm() == 1)
        .collect()
}

#[aoc_generator(day15)]
pub fn generate(inp: &str) -> Map {
    let mut tiles = HashMap::new();
    let mut enemies = Vec::new();

    for (row_idx, line) in inp.lines().enumerate() {
        for (col_idx, chr) in line.chars().enumerate() {
            let cur_pos = Complex::new(row_idx as i64, col_idx as i64);
            if chr == 'E' {
                enemies.push(Enemy::elf(cur_pos));
            } else if chr == 'G' {
                enemies.push(Enemy::goblin(cur_pos));
            }

            match chr {
                'E' | 'G' | '.' => tiles.insert(cur_pos, Tile::Open),
                '#' => tiles.insert(cur_pos, Tile::Wall),
                _ => panic!("Unexpected character"),
            };
        }
    }

    Map { tiles, enemies }
}

fn do_attack(
    idx: usize,
    cur_kind: Kind,
    enemies: &mut [Enemy],
    positions_cache: &mut HashSet<Complex<i64>>,
) -> bool {
    let damage = enemies[idx].ap;

    let enemies_in_range = adjacent_enemies(enemies[idx].position, cur_kind, enemies);

    let did_attack = if let Some(lowest_enemy) = enemies_in_range
        .into_iter()
        .sorted_by(|lhs, rhs| lhs.hp.cmp(&rhs.hp).then_with(|| lhs.cmp(rhs)))
        .next()
    {
        lowest_enemy.hp -= damage;
        lowest_enemy.is_dead = lowest_enemy.hp <= 0;
        Some((lowest_enemy.position, lowest_enemy.is_dead))
    } else {
        None
    };

    if let Some((pos, true)) = did_attack {
        positions_cache.remove(&pos);
    }

    did_attack.is_some()
}

fn enemy_positions(enemies: &[Enemy]) -> HashSet<Complex<i64>> {
    enemies
        .iter()
        .filter_map(|it| if it.is_dead { None } else { Some(it.position) })
        .collect()
}

fn shortest_paths_to_target(
    target: Complex<i64>,
    current_pos: Complex<i64>,
    tiles: &HashMap<Complex<i64>, Tile>,
    enemy_positions: &HashSet<Complex<i64>>,
) -> Option<(AstarSolution<Complex<i64>>, usize)> {
    let (paths, cost) = pathfinding::prelude::astar_bag(
        &current_pos,
        |it| {
            adjacent_open_squares(*it, tiles, enemy_positions)
                .iter()
                .map(|it| (*it, 1usize))
                .collect_vec()
        },
        |it| (target - *it).l1_norm() as usize,
        |it| *it == target,
    )?;

    Some((paths, cost))
}

fn move_to_closest_target(
    current_pos: Complex<i64>,
    tiles: &HashMap<Complex<i64>, Tile>,
    target_squares: &[Complex<i64>],
    enemy_positions: &HashSet<Complex<i64>>,
) -> Option<Complex<i64>> {
    let shortest_paths = target_squares
        .iter()
        .map(|it| {
            (
                *it,
                shortest_paths_to_target(*it, current_pos, tiles, enemy_positions),
            )
        })
        .filter_map(|(target, paths)| paths.map(|p| (target, p)))
        .min_set_by_key(|(_, (_, cost))| *cost);

    shortest_paths
        .iter()
        .min_set_by(|(lhs, _), (rhs, _)| reading_order(lhs, rhs))
        .iter()
        .flat_map(|(_, (it, _))| it.clone().map(|p| p[1]).collect_vec())
        .min_by(reading_order)
}

pub enum GameState {
    NoTargets,
    DeadElf,
}

fn play_round(
    tiles: &HashMap<Complex<i64>, Tile>,
    enemies: &mut Vec<Enemy>,
    fail_on_dead_elf: bool,
) -> Result<(), GameState> {
    enemies.sort();

    let mut enemy_positions_cache = enemy_positions(enemies);

    for idx in 0..enemies.len() {
        assert_eq!(enemy_positions_cache, enemy_positions(enemies));

        if enemies[idx].is_dead {
            if fail_on_dead_elf && enemies[idx].kind == Kind::Elf {
                return Err(GameState::DeadElf);
            }

            continue;
        }

        let current_kind = enemies[idx].kind;

        if do_attack(idx, current_kind, enemies, &mut enemy_positions_cache) {
            continue;
        }

        let possible_targets = enemies
            .iter()
            .filter(|it| !it.is_dead && it.kind != current_kind)
            .collect_vec();

        if possible_targets.is_empty() {
            return Err(GameState::NoTargets);
        }

        let possible_target_squares = possible_targets
            .iter()
            .flat_map(|it| adjacent_open_squares(it.position, tiles, &enemy_positions_cache))
            .collect_vec();

        // move & try to attack again
        if let Some(next_pos) = move_to_closest_target(
            enemies[idx].position,
            tiles,
            &possible_target_squares,
            &enemy_positions_cache,
        ) {
            enemy_positions_cache.remove(&enemies[idx].position);
            enemies[idx].position = next_pos;
            enemy_positions_cache.insert(next_pos);

            do_attack(idx, current_kind, enemies, &mut enemy_positions_cache);
        }

        assert_eq!(enemy_positions_cache, enemy_positions(enemies));
    }

    Ok(())
}

fn remaining_hp(enemies: &[Enemy]) -> i64 {
    enemies
        .iter()
        .filter(|it| !it.is_dead)
        .map(|it| it.hp)
        .sum()
}

#[aoc(day15, part1)]
pub fn part1(map: &Map) -> i64 {
    let mut enemies = map.enemies.clone();

    let mut round = 0;
    while play_round(&map.tiles, &mut enemies, false).is_ok() {
        round += 1;
    }

    round * remaining_hp(&enemies)
}

#[aoc(day15, part2)]
pub fn part2(map: &Map) -> i64 {
    for ap in 4.. {
        let mut enemies = map
            .enemies
            .iter()
            .map(|it| {
                if it.kind == Kind::Elf {
                    Enemy::elf_with_ap(it.position, ap)
                } else {
                    *it
                }
            })
            .collect_vec();

        for enemy in &mut enemies {
            if enemy.kind == Kind::Elf {
                enemy.ap = ap;
            }
        }

        let mut round = 0;
        loop {
            match play_round(&map.tiles, &mut enemies, true) {
                Ok(_) => round += 1,
                Err(GameState::DeadElf) => break,
                Err(GameState::NoTargets) => return round * remaining_hp(&enemies),
            };
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_p1_1() {
        const TEST_DATA: &str = "#########\n\
                                 #G......#\n\
                                 #.E.#...#\n\
                                 #..##..G#\n\
                                 #...##..#\n\
                                 #...#...#\n\
                                 #.G...G.#\n\
                                 #.....G.#\n\
                                 #########";

        let map = generate(TEST_DATA);
        let result = part1(&map);
        assert_eq!(result, 18_740);
    }

    #[test]
    fn test_sample_p1_2() {
        const TEST_DATA: &str = "#######\n\
                                 #G..#E#\n\
                                 #E#E.E#\n\
                                 #G.##.#\n\
                                 #...#E#\n\
                                 #...E.#\n\
                                 #######";

        let map = generate(TEST_DATA);
        let result = part1(&map);
        assert_eq!(result, 36_334);
    }

    #[test]
    fn test_sample_p1_3() {
        const TEST_DATA: &str = "#######\n\
                                 #E..EG#\n\
                                 #.#G.E#\n\
                                 #E.##E#\n\
                                 #G..#.#\n\
                                 #..E#.#\n\
                                 #######";

        let map = generate(TEST_DATA);
        let result = part1(&map);
        assert_eq!(result, 39_514);
    }

    #[test]
    fn test_sample_p1_4() {
        const TEST_DATA: &str = "#######\n\
                                 #E.G#.#\n\
                                 #.#G..#\n\
                                 #G.#.G#\n\
                                 #G..#.#\n\
                                 #...E.#\n\
                                 #######";

        let map = generate(TEST_DATA);
        let result = part1(&map);
        assert_eq!(result, 27_755);
    }

    #[test]
    fn test_sample_p1_5() {
        const TEST_DATA: &str = "#######\n\
                                 #.E...#\n\
                                 #.#..G#\n\
                                 #.###.#\n\
                                 #E#G#G#\n\
                                 #...#G#\n\
                                 #######";

        let map = generate(TEST_DATA);
        let result = part1(&map);
        assert_eq!(result, 28_944);
    }

    #[test]
    fn test_sample_p1_6() {
        const TEST_DATA: &str = "#######\n\
                                 #.G...#\n\
                                 #...EG#\n\
                                 #.#.#G#\n\
                                 #..G#E#\n\
                                 #.....#\n\
                                 #######";

        let map = generate(TEST_DATA);
        let result = part1(&map);
        assert_eq!(result, 27_730);
    }

    #[test]
    fn test_compare() {
        {
            let enemy1 = Enemy::elf(Complex::new(1, 1));
            let enemy2 = Enemy::elf(Complex::new(2, 1));
            assert!(enemy1 < enemy2);
        }
        {
            let enemy1 = Enemy::elf(Complex::new(1, 10));
            let enemy2 = Enemy::elf(Complex::new(2, 1));
            assert!(enemy1 < enemy2);
        }
        {
            let enemy1 = Enemy::elf(Complex::new(2, 7));
            let enemy2 = Enemy::elf(Complex::new(2, 5));
            assert!(enemy1 > enemy2);
        }
    }
}
