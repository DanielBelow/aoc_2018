use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[derive(Default, Debug)]
pub struct Header {
    num_children: usize,
    num_metadata: usize,
}

impl Header {
    const fn new(num_children: usize, num_metadata: usize) -> Self {
        Self {
            num_children,
            num_metadata,
        }
    }
}

#[derive(Default, Debug)]
pub struct Node {
    children: Vec<Node>,
    metadata: Vec<usize>,
}

impl Node {
    fn all_metadata(&self) -> Vec<usize> {
        let mut metadata = self.metadata.clone();
        let mut children_metadata = self
            .children
            .iter()
            .flat_map(Self::all_metadata)
            .collect_vec();

        metadata.append(&mut children_metadata);
        metadata
    }

    fn value(&self) -> usize {
        if self.children.is_empty() {
            return self.all_metadata().iter().sum::<usize>();
        }

        let indices = self.metadata.iter().map(|it| it - 1).collect_vec();

        let mut result = 0;
        for idx in indices {
            if idx >= self.children.len() {
                continue;
            }

            result += self.children[idx].value();
        }

        result
    }

    fn from_parts(inp: &mut Vec<usize>) -> Option<Self> {
        if inp.len() < 2 {
            return None;
        }

        // current header
        let header = Header::new(inp.remove(0), inp.remove(0));

        let mut children = vec![];
        let mut metadata = vec![];

        // parse sub-nodes
        for _ in 0..header.num_children {
            if let Some(child_node) = Self::from_parts(inp) {
                children.push(child_node);
            }
        }

        // parse metadata
        for _ in 0..header.num_metadata {
            metadata.push(inp.remove(0));
        }

        Some(Self { children, metadata })
    }
}

#[aoc_generator(day08)]
pub fn generate(inp: &str) -> Option<Node> {
    let mut parts = inp
        .lines()
        .next()?
        .split_ascii_whitespace()
        .filter_map(|it| it.parse::<usize>().ok())
        .collect_vec();

    Node::from_parts(&mut parts)
}

#[aoc(day08, part1)]
pub fn part1(root: &Node) -> usize {
    root.all_metadata().iter().sum::<usize>()
}

#[aoc(day08, part2)]
pub fn part2(root: &Node) -> usize {
    root.value()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";

    #[test]
    fn test_sample_p1() {
        let Some(data) = generate(TEST_INPUT) else {
            panic!("Could not parse test input")
        };

        let res = part1(&data);
        assert_eq!(res, 138);
    }

    #[test]
    fn test_sample_p2() {
        let Some(data) = generate(TEST_INPUT) else {
            panic!("Could not parse test input")
        };

        let res = part2(&data);
        assert_eq!(res, 66);
    }
}
