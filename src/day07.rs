use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use parse_display::{Display, FromStr};
use std::collections::{HashMap, HashSet};

#[derive(Display, FromStr, Clone, Debug)]
#[display("Step {node} must be finished before step {successor} can begin.")]
pub struct Step {
    node: char,
    successor: char,
}

#[aoc_generator(day07)]
pub fn generate(inp: &str) -> Vec<Step> {
    inp.lines().filter_map(|it| it.parse().ok()).collect_vec()
}

fn generate_topo_map(steps: &[Step]) -> HashMap<char, HashSet<char>> {
    let mut prereqs = HashMap::new();

    for step in steps {
        prereqs.entry(step.node).or_insert_with(HashSet::new);
        prereqs
            .entry(step.successor)
            .or_insert_with(HashSet::new)
            .insert(step.node);
    }

    prereqs
}

#[aoc(day07, part1)]
pub fn part1(steps: &[Step]) -> String {
    let mut prereqs = generate_topo_map(steps);

    let mut result = String::new();
    loop {
        if prereqs.is_empty() {
            break;
        }

        let next_node = prereqs
            .iter()
            .filter(|(_, v)| v.is_empty())
            .map(|(k, _)| *k)
            .sorted()
            .next()
            .expect("There has to be an available node!");

        result.push(next_node);

        prereqs.remove(&next_node);
        for v in prereqs.values_mut() {
            v.remove(&next_node);
        }
    }

    result
}

fn char_index(c: char) -> u8 {
    ((c as u8) - b'A') + 1
}

#[aoc(day07, part2)]
pub fn part2(steps: &[Step]) -> u64 {
    let mut prereqs = generate_topo_map(steps);

    let mut workers: Vec<(char, u8)> = Vec::new();
    let mut cur_time = 0;
    loop {
        if prereqs.is_empty() {
            break;
        }

        for (_, w) in &mut workers {
            *w -= 1;
        }

        let mut finished_nodes = Vec::new();

        for &(job, time) in &workers {
            if time == 0 {
                finished_nodes.push(job);
            }
        }

        for finished_node in finished_nodes {
            prereqs.remove(&finished_node);
            for v in prereqs.values_mut() {
                v.remove(&finished_node);
            }

            let pos = workers
                .iter()
                .position(|(k, _)| *k == finished_node)
                .unwrap();
            workers.remove(pos);
        }

        assert!(workers.len() <= 5);

        let num_to_insert = 5 - workers.len();

        let next_nodes = prereqs
            .iter()
            .filter(|(_, v)| v.is_empty())
            .map(|(k, _)| *k)
            .sorted()
            .filter(|it| !workers.iter().any(|(k, _)| *k == *it))
            .take(num_to_insert)
            .collect_vec();

        for node in next_nodes {
            workers.push((node, 60 + char_index(node)));
        }

        cur_time += 1;
    }

    cur_time - 1
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "Step C must be finished before step A can begin.\n\
                              Step C must be finished before step F can begin.\n\
                              Step A must be finished before step B can begin.\n\
                              Step A must be finished before step D can begin.\n\
                              Step B must be finished before step E can begin.\n\
                              Step D must be finished before step E can begin.\n\
                              Step F must be finished before step E can begin.";

    #[test]
    fn test_sample_p1() {
        let gen = generate(TEST_INPUT);
        let res = part1(&gen);

        assert_eq!(res, "CABDFE");
    }
}
