use aoc_runner_derive::aoc;
use itertools::Itertools;

fn indices_to_delete(polymer: &str) -> Option<usize> {
    for (idx, (l, r)) in polymer.chars().tuple_windows::<(_, _)>().enumerate() {
        assert!(l.is_ascii() && r.is_ascii());

        if l.to_ascii_uppercase() == r.to_ascii_uppercase() && (l.is_uppercase() ^ r.is_uppercase())
        {
            return Some(idx);
        }
    }

    None
}

fn perform_reaction(polymer: &str) -> String {
    let mut result = polymer.to_string();

    loop {
        let index = indices_to_delete(&result);
        match index {
            None => return result,
            Some(idx) => {
                // remove char at index and the next
                result.remove(idx);
                result.remove(idx);
            }
        };
    }
}

#[aoc(day05, part1)]
pub fn part1(polymer: &str) -> usize {
    let fully_reacted = perform_reaction(polymer);
    fully_reacted.len()
}

#[aoc(day05, part2)]
pub fn part2(polymer: &str) -> Option<usize> {
    ('a'..='z')
        .map(|it| {
            let new_polymer = polymer.replace([it, it.to_ascii_uppercase()], "");
            let fully_reacted = perform_reaction(&new_polymer);
            fully_reacted.len()
        })
        .min()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "dabAcCaCBAcCcaDA";

    #[test]
    fn test_sample_p1() {
        let res = part1(TEST_INPUT);
        assert_eq!(res, 10);
    }

    #[test]
    fn test_sample_p2() {
        let res = part2(TEST_INPUT);
        assert_eq!(res, Some(4));
    }
}
