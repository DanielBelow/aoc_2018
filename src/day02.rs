use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::collections::HashSet;

#[aoc_generator(day02)]
pub fn generate(inp: &str) -> Vec<String> {
    inp.lines().map(ToString::to_string).collect_vec()
}

fn letter_frequencies(s: &str) -> HashSet<usize> {
    s.chars().counts().values().copied().collect::<HashSet<_>>()
}

#[aoc(day02, part1)]
pub fn part1(inp: &[String]) -> u64 {
    let mut twos = 0;
    let mut threes = 0;

    for word in inp {
        let freqs = letter_frequencies(word);
        if freqs.contains(&2) {
            twos += 1;
        }

        if freqs.contains(&3) {
            threes += 1;
        }
    }

    twos * threes
}

fn find_single_differing_position(lhs: &str, rhs: &str) -> Option<usize> {
    let differing_letters = lhs
        .chars()
        .zip(rhs.chars())
        .enumerate()
        .filter(|&(_, (l, r))| l != r)
        .collect_vec();

    if differing_letters.len() != 1 {
        return None;
    }

    let (idx, _) = differing_letters[0];
    Some(idx)
}

#[aoc(day02, part2)]
pub fn part2(inp: &[String]) -> String {
    for word1 in inp {
        for word2 in inp {
            if word1 == word2 {
                continue;
            }

            if let Some(position) = find_single_differing_position(word1, word2) {
                let mut new_word = word1.clone();
                new_word.remove(position);
                return new_word;
            }
        }
    }

    "foobar".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_p1() {
        const INP: &str = "abcdef\n\
                           bababc\n\
                           abbcde\n\
                           abcccd\n\
                           aabcdd\n\
                           abcdee\n\
                           ababab";

        let data = generate(INP);
        let res = part1(&data);
        assert_eq!(res, 12);
    }

    #[test]
    fn test_sample_p2() {
        const INP: &str = "abcde\n\
                           fghij\n\
                           klmno\n\
                           pqrst\n\
                           fguij\n\
                           axcye\n\
                           wvxyz";

        let data = generate(INP);
        let res = part2(&data);
        assert_eq!(res, "fgij");
    }
}
