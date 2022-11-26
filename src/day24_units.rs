use std::cmp::Ordering;

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum DamageType {
    Fire,
    Cold,
    Slashing,
    Bludgeoning,
    Radiation,
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum Affinity {
    Weakness(DamageType),
    Immunity(DamageType),
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub struct Attack {
    pub damage_type: DamageType,
    pub power: usize,
}

impl Attack {
    pub fn new(damage_type: DamageType, power: usize) -> Self {
        Self { damage_type, power }
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct UnitGroup {
    pub num_units: usize,
    pub hp_per_unit: usize,
    pub affinities: Vec<Affinity>,
    pub attack: Attack,
    pub initiative: usize,
}

impl UnitGroup {
    pub fn new(
        num_units: usize,
        hp_per_unit: usize,
        affinities: Vec<Affinity>,
        attack: Attack,
        initiative: usize,
    ) -> Self {
        Self {
            num_units,
            hp_per_unit,
            affinities,
            attack,
            initiative,
        }
    }

    pub fn effective_power(&self) -> usize {
        self.num_units * self.attack.power
    }

    fn is_immune_to(&self, damage_type: DamageType) -> bool {
        let immunity = Affinity::Immunity(damage_type);
        self.affinities.iter().any(|it| *it == immunity)
    }

    fn is_weak_to(&self, damage_type: DamageType) -> bool {
        let weakness = Affinity::Weakness(damage_type);
        self.affinities.iter().any(|it| *it == weakness)
    }

    pub fn calculate_damage_done_to_group(&self, group: &Self) -> usize {
        let damage_multiplier = if group.is_immune_to(self.attack.damage_type) {
            0
        } else if group.is_weak_to(self.attack.damage_type) {
            2
        } else {
            1
        };

        damage_multiplier * self.effective_power()
    }

    pub fn compare_for_target_selection(lhs: &Self, rhs: &Self) -> Ordering {
        match lhs.effective_power().cmp(&rhs.effective_power()) {
            Ordering::Less => Ordering::Greater,
            Ordering::Greater => Ordering::Less,
            Ordering::Equal => lhs.initiative.cmp(&rhs.initiative).reverse(),
        }
    }
}

#[derive(Clone)]
pub struct Army {
    pub units: Vec<UnitGroup>,
}

impl Army {
    pub fn is_defeated(&self) -> bool {
        self.units.is_empty()
    }

    pub fn count_remaining_units(&self) -> usize {
        self.units.iter().map(|it| it.num_units).sum()
    }
}

pub fn get_immune_system_unit_groups() -> Vec<UnitGroup> {
    vec![
        UnitGroup::new(
            7079,
            12296,
            vec![Affinity::Weakness(DamageType::Fire)],
            Attack::new(DamageType::Bludgeoning, 13),
            14,
        ),
        UnitGroup::new(
            385,
            9749,
            vec![Affinity::Weakness(DamageType::Cold)],
            Attack::new(DamageType::Bludgeoning, 196),
            16,
        ),
        UnitGroup::new(
            2232,
            1178,
            vec![
                Affinity::Weakness(DamageType::Cold),
                Affinity::Weakness(DamageType::Slashing),
            ],
            Attack::new(DamageType::Fire, 4),
            20,
        ),
        UnitGroup::new(
            917,
            2449,
            vec![
                Affinity::Weakness(DamageType::Bludgeoning),
                Affinity::Immunity(DamageType::Fire),
                Affinity::Immunity(DamageType::Cold),
            ],
            Attack::new(DamageType::Cold, 25),
            15,
        ),
        UnitGroup::new(
            2657,
            2606,
            vec![Affinity::Weakness(DamageType::Slashing)],
            Attack::new(DamageType::Cold, 9),
            13,
        ),
        UnitGroup::new(2460, 7566, vec![], Attack::new(DamageType::Cold, 29), 8),
        UnitGroup::new(
            2106,
            6223,
            vec![],
            Attack::new(DamageType::Bludgeoning, 29),
            2,
        ),
        UnitGroup::new(
            110,
            7687,
            vec![
                Affinity::Weakness(DamageType::Slashing),
                Affinity::Immunity(DamageType::Radiation),
                Affinity::Immunity(DamageType::Fire),
            ],
            Attack::new(DamageType::Slashing, 506),
            19,
        ),
        UnitGroup::new(
            7451,
            9193,
            vec![Affinity::Immunity(DamageType::Cold)],
            Attack::new(DamageType::Radiation, 12),
            6,
        ),
        UnitGroup::new(
            1167,
            3162,
            vec![
                Affinity::Immunity(DamageType::Bludgeoning),
                Affinity::Weakness(DamageType::Fire),
            ],
            Attack::new(DamageType::Fire, 23),
            9,
        ),
    ]
}

pub fn get_infection_unit_groups() -> Vec<UnitGroup> {
    vec![
        UnitGroup::new(
            2907,
            11244,
            vec![Affinity::Immunity(DamageType::Slashing)],
            Attack::new(DamageType::Fire, 7),
            7,
        ),
        UnitGroup::new(
            7338,
            12201,
            vec![
                Affinity::Immunity(DamageType::Bludgeoning),
                Affinity::Immunity(DamageType::Slashing),
                Affinity::Immunity(DamageType::Cold),
            ],
            Attack::new(DamageType::Radiation, 3),
            4,
        ),
        UnitGroup::new(
            7905,
            59276,
            vec![Affinity::Immunity(DamageType::Fire)],
            Attack::new(DamageType::Cold, 12),
            17,
        ),
        UnitGroup::new(
            1899,
            50061,
            vec![Affinity::Weakness(DamageType::Fire)],
            Attack::new(DamageType::Radiation, 51),
            10,
        ),
        UnitGroup::new(2711, 27602, vec![], Attack::new(DamageType::Cold, 17), 12),
        UnitGroup::new(
            935,
            38240,
            vec![Affinity::Immunity(DamageType::Slashing)],
            Attack::new(DamageType::Bludgeoning, 78),
            1,
        ),
        UnitGroup::new(
            2783,
            17937,
            vec![
                Affinity::Immunity(DamageType::Cold),
                Affinity::Immunity(DamageType::Bludgeoning),
            ],
            Attack::new(DamageType::Fire, 12),
            11,
        ),
        UnitGroup::new(
            8046,
            13608,
            vec![
                Affinity::Weakness(DamageType::Fire),
                Affinity::Weakness(DamageType::Bludgeoning),
            ],
            Attack::new(DamageType::Slashing, 2),
            5,
        ),
        UnitGroup::new(
            2112,
            37597,
            vec![
                Affinity::Immunity(DamageType::Cold),
                Affinity::Immunity(DamageType::Slashing),
            ],
            Attack::new(DamageType::Slashing, 31),
            18,
        ),
        UnitGroup::new(
            109,
            50867,
            vec![
                Affinity::Immunity(DamageType::Slashing),
                Affinity::Weakness(DamageType::Radiation),
            ],
            Attack::new(DamageType::Cold, 886),
            3,
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_comparison() {
        let lhs = UnitGroup::new(100, 5, vec![], Attack::new(DamageType::Cold, 20), 15);
        let rhs = UnitGroup::new(50, 5, vec![], Attack::new(DamageType::Cold, 20), 10);

        let mut units = vec![lhs.clone(), rhs.clone()];
        units.sort_by(UnitGroup::compare_for_target_selection);
        assert_eq!(units, vec![lhs, rhs]);
    }

    #[test]
    fn test_comparison_tiebreak() {
        let lhs = UnitGroup::new(
            100,
            5,
            vec![Affinity::Weakness(DamageType::Fire)],
            Attack::new(DamageType::Cold, 10),
            10,
        );
        let rhs = UnitGroup::new(50, 5, vec![], Attack::new(DamageType::Fire, 20), 15);

        let mut units = vec![lhs.clone(), rhs.clone()];
        units.sort_by(UnitGroup::compare_for_target_selection);
        assert_eq!(units, vec![rhs, lhs]);
    }
}
