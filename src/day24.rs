use super::day24_units::{
    get_immune_system_unit_groups, get_infection_unit_groups, Army, UnitGroup,
};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day24)]
pub fn generate(_: &str) -> (Army, Army) {
    let immune_system = Army {
        units: get_immune_system_unit_groups(),
    };

    let infection = Army {
        units: get_infection_unit_groups(),
    };

    (immune_system, infection)
}

fn select_targets_for(attacker: &Army, defender: &Army) -> Vec<Option<usize>> {
    let mut result = vec![];

    for attacker in &attacker.units {
        let potential_targets = defender
            .units
            .iter()
            .enumerate()
            .filter(|(_, it)| attacker.calculate_damage_done_to_group(it) > 0)
            .filter(|(pos, _)| !result.contains(&Some(*pos)))
            .max_set_by_key(|(_, it)| attacker.calculate_damage_done_to_group(it));

        if potential_targets.len() == 1 {
            result.push(Some(potential_targets[0].0));
        } else if let Some((pos, _)) = potential_targets
            .iter()
            .min_by(|(_, lhs), (_, rhs)| UnitGroup::compare_for_target_selection(lhs, rhs))
        {
            result.push(Some(*pos));
        } else {
            result.push(None);
        }
    }

    result
}

fn select_targets(
    immune_system: &Army,
    infection: &Army,
) -> (Vec<Option<usize>>, Vec<Option<usize>>) {
    let immune_system_targets = select_targets_for(immune_system, infection);
    let infection_targets = select_targets_for(infection, immune_system);

    (immune_system_targets, infection_targets)
}

fn get_attacking_order(immune_system: &Army, infection: &Army) -> Vec<usize> {
    immune_system
        .units
        .iter()
        .chain(infection.units.iter())
        .enumerate()
        .sorted_by_key(|(_, it)| it.initiative)
        .rev()
        .map(|(pos, _)| pos)
        .collect()
}

fn simulate((immune_system, infection): &(Army, Army)) -> (bool, usize) {
    let mut immune_system = immune_system.clone();
    let mut infection = infection.clone();

    loop {
        // select targets
        let num_immune_system_groups = immune_system.units.len();

        immune_system
            .units
            .sort_by(UnitGroup::compare_for_target_selection);
        infection
            .units
            .sort_by(UnitGroup::compare_for_target_selection);

        if immune_system.is_defeated() {
            // infection won!
            return (false, infection.count_remaining_units());
        } else if infection.is_defeated() {
            // immune system won!
            return (true, immune_system.count_remaining_units());
        }

        let (immune_system_targets, infection_targets) = select_targets(&immune_system, &infection);

        // fight
        let attacking_order = get_attacking_order(&immune_system, &infection);

        let adjusted_index = |atk_idx: usize| {
            if atk_idx >= num_immune_system_groups {
                infection_targets[atk_idx - num_immune_system_groups]
            } else {
                immune_system_targets[atk_idx]
            }
        };

        let mut killed_unit = false;
        for attacking_idx in attacking_order {
            if let Some(def_idx) = adjusted_index(attacking_idx) {
                let (attacker, defender) = if attacking_idx >= num_immune_system_groups {
                    (
                        &infection.units[attacking_idx - num_immune_system_groups],
                        &mut immune_system.units[def_idx],
                    )
                } else {
                    (
                        &immune_system.units[attacking_idx],
                        &mut infection.units[def_idx],
                    )
                };

                assert!(defender.num_units > 0);

                let total_damage = attacker.calculate_damage_done_to_group(defender);
                let num_units_killed =
                    (total_damage / defender.hp_per_unit).min(defender.num_units);
                defender.num_units = defender.num_units.saturating_sub(num_units_killed);

                if num_units_killed > 0 {
                    killed_unit = true;
                }
            }
        }

        if !killed_unit {
            return (false, 0);
        }

        immune_system.units.retain(|it| it.num_units > 0);
        infection.units.retain(|it| it.num_units > 0);
    }
}

#[aoc(day24, part1)]
pub fn part1(inp: &(Army, Army)) -> usize {
    let (_, res) = simulate(inp);
    res
}

#[aoc(day24, part2)]
pub fn part2((immune_system, infection): &(Army, Army)) -> usize {
    let mut cur_boost = 0;
    loop {
        let mut new_immune_system = immune_system.clone();
        let new_infection = infection.clone();

        for unit in &mut new_immune_system.units {
            unit.attack.power += cur_boost;
        }

        let (immune_system_won, result) = simulate(&(new_immune_system, new_infection));
        if immune_system_won {
            return result;
        }

        cur_boost += 1;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::day24_units::{Affinity, Attack, DamageType};

    fn get_test_data() -> (Army, Army) {
        let is = Army {
            units: vec![
                UnitGroup::new(
                    17,
                    5390,
                    vec![
                        Affinity::Weakness(DamageType::Radiation),
                        Affinity::Weakness(DamageType::Bludgeoning),
                    ],
                    Attack::new(DamageType::Fire, 4507),
                    2,
                ),
                UnitGroup::new(
                    989,
                    1274,
                    vec![
                        Affinity::Immunity(DamageType::Fire),
                        Affinity::Weakness(DamageType::Bludgeoning),
                        Affinity::Weakness(DamageType::Slashing),
                    ],
                    Attack::new(DamageType::Slashing, 25),
                    3,
                ),
            ],
        };

        let inf = Army {
            units: vec![
                UnitGroup::new(
                    801,
                    4706,
                    vec![Affinity::Weakness(DamageType::Radiation)],
                    Attack::new(DamageType::Bludgeoning, 116),
                    1,
                ),
                UnitGroup::new(
                    4485,
                    2961,
                    vec![
                        Affinity::Immunity(DamageType::Radiation),
                        Affinity::Weakness(DamageType::Fire),
                    ],
                    Attack::new(DamageType::Slashing, 12),
                    4,
                ),
            ],
        };

        (is, inf)
    }

    #[test]
    fn test_sample_p1() {
        let result = part1(&get_test_data());
        assert_eq!(result, 5216);
    }

    #[test]
    fn test_sample_p2() {
        let result = part2(&get_test_data());
        assert_eq!(result, 51);
    }
}
