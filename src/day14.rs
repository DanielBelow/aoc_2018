use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day14)]
pub fn generate(inp: &str) -> Option<usize> {
    inp.parse().ok()
}

#[allow(clippy::cast_possible_truncation)]
fn char_to_digit(c: char) -> u8 {
    c.to_digit(10).unwrap() as u8
}

fn add_new_recipes(elf1: usize, elf2: usize, recipes: &mut Vec<u8>) {
    let result = format!("{}", recipes[elf1] + recipes[elf2]);
    for num in result.chars().map(char_to_digit) {
        recipes.push(num);
    }
}

fn update_elf_positions(elf1: usize, elf2: usize, recipes: &[u8]) -> (usize, usize) {
    let elf1_steps = recipes[elf1] + 1;
    let elf2_steps = recipes[elf2] + 1;

    let num_recipes = recipes.len();
    let new_elf1 = (elf1 + elf1_steps as usize) % num_recipes;
    let new_elf2 = (elf2 + elf2_steps as usize) % num_recipes;

    (new_elf1, new_elf2)
}

fn craft_n_recipes(steps: usize, elf1: usize, elf2: usize, recipes: &mut Vec<u8>) {
    let mut cur_elf1 = elf1;
    let mut cur_elf2 = elf2;

    for _ in 0..steps {
        add_new_recipes(cur_elf1, cur_elf2, recipes);
        (cur_elf1, cur_elf2) = update_elf_positions(cur_elf1, cur_elf2, recipes);
    }
}

#[aoc(day14, part1)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn part1(num: &usize) -> String {
    let mut recipes = vec![3, 7];

    craft_n_recipes(*num + 10, 0, 1, &mut recipes);

    recipes.iter().skip(*num).take(10).join("")
}

#[aoc(day14, part2)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn part2(num: &usize) -> usize {
    let mut recipes = vec![3, 7];
    let mut cur_elf1 = 0;
    let mut cur_elf2 = 1;

    let num_as_str = format!("{num}");

    loop {
        // do 2mil steps at a time
        for _ in 0..2_000_000 {
            add_new_recipes(cur_elf1, cur_elf2, &mut recipes);
            (cur_elf1, cur_elf2) = update_elf_positions(cur_elf1, cur_elf2, &recipes);
        }

        let cur_recipes = recipes.iter().join("");
        if let Some(idx) = cur_recipes.find(&num_as_str) {
            return idx;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_recipes() {
        let mut recipes = vec![3, 7];
        add_new_recipes(0, 1, &mut recipes);
        assert_eq!(recipes, vec![3, 7, 1, 0]);
    }

    #[test]
    fn test_move_elves_1() {
        let recipes = vec![3, 7, 1, 0];
        let (e1, e2) = update_elf_positions(0, 1, &recipes);
        assert_eq!(e1, 0);
        assert_eq!(e2, 1);
    }

    #[test]
    fn test_move_elves_2() {
        let recipes = vec![3, 7, 1, 0, 1, 0];
        let (e1, e2) = update_elf_positions(0, 1, &recipes);
        assert_eq!(e1, 4);
        assert_eq!(e2, 3);
    }

    #[test]
    fn test_sample_p1() {
        assert_eq!("5158916779", part1(&9));
    }

    #[test]
    fn test_sample_p2() {
        assert_eq!(9, part2(&51589));
    }
}
