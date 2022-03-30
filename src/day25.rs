use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use parse_display::{Display, FromStr};

#[derive(Display, FromStr, Eq, PartialEq, Copy, Clone, Debug)]
#[display("{x},{y},{z},{w}")]
pub struct Point {
    x: i64,
    y: i64,
    z: i64,
    w: i64,
}

impl Point {
    fn manhattan_distance(&self, pt: &Point) -> u64 {
        let diff = |lhs: i64, rhs: i64| -> u64 { (lhs - rhs).abs() as u64 };

        diff(self.x, pt.x) + diff(self.y, pt.y) + diff(self.z, pt.z) + diff(self.w, pt.w)
    }
}

#[derive(Eq, PartialEq, Clone, Debug)]
struct Constellation {
    points: Vec<Point>,
}

impl Constellation {
    fn is_reachable_from_point(&self, pt: &Point) -> bool {
        self.points.iter().any(|it| it.manhattan_distance(pt) <= 3)
    }

    fn is_reachable_from_constellation(&self, constellation: &Self) -> bool {
        constellation
            .points
            .iter()
            .any(|it| self.is_reachable_from_point(it))
    }

    fn add_point(&mut self, pt: Point) {
        self.points.push(pt);
    }
}

#[aoc_generator(day25)]
pub fn generate(inp: &str) -> Vec<Point> {
    inp.lines().filter_map(|it| it.parse().ok()).collect_vec()
}

fn merge_constellations(mut constellations: Vec<Constellation>) -> (Vec<Constellation>, bool) {
    let orig_len = constellations.len();

    let mut result = vec![];

    while let Some(cur_const) = constellations.pop() {
        if let Some((idx, reachable)) = constellations
            .iter()
            .enumerate()
            .find(|&(_, it)| cur_const.is_reachable_from_constellation(it))
        {
            let merged = Constellation {
                points: reachable
                    .points
                    .iter()
                    .chain(cur_const.points.iter())
                    .copied()
                    .collect(),
            };
            result.push(merged);
            constellations.remove(idx);
        } else {
            result.push(cur_const.clone());
        }
    }

    let has_changes = result.len() != orig_len;
    (result, has_changes)
}

#[aoc(day25, part1)]
pub fn part1(points: &[Point]) -> usize {
    let mut constellations = Vec::<Constellation>::new();

    for point in points {
        if let Some(constellation) = constellations
            .iter_mut()
            .find(|it| it.is_reachable_from_point(point))
        {
            constellation.add_point(*point);
        } else {
            constellations.push(Constellation {
                points: vec![*point],
            });
        }
    }

    loop {
        let (merged, changed) = merge_constellations(constellations);
        if !changed {
            return merged.len();
        }

        constellations = merged;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sample_p1_1() {
        let input = "0,0,0,0\n\
                            3,0,0,0\n\
                            0,3,0,0\n\
                            0,0,3,0\n\
                            0,0,0,3\n\
                            0,0,0,6\n\
                            9,0,0,0\n\
                            12,0,0,0";
        let gen = generate(input);
        let res = part1(&gen);
        assert_eq!(res, 2);
    }

    #[test]
    fn test_sample_p1_2() {
        let input = "-1,2,2,0\n\
                            0,0,2,-2\n\
                            0,0,0,-2\n\
                            -1,2,0,0\n\
                            -2,-2,-2,2\n\
                            3,0,2,-1\n\
                            -1,3,2,2\n\
                            -1,0,-1,0\n\
                            0,2,1,-2\n\
                            3,0,0,0";
        let gen = generate(input);
        let res = part1(&gen);
        assert_eq!(res, 4);
    }

    #[test]
    fn test_sample_p1_3() {
        let input = "1,-1,0,1\n\
                            2,0,-1,0\n\
                            3,2,-1,0\n\
                            0,0,3,1\n\
                            0,0,-1,-1\n\
                            2,3,-2,0\n\
                            -2,2,0,0\n\
                            2,-2,0,-1\n\
                            1,-1,0,-1\n\
                            3,2,0,2";
        let gen = generate(input);
        let res = part1(&gen);
        assert_eq!(res, 3);
    }

    #[test]
    fn test_sample_p1_4() {
        let input = "1,-1,-1,-2\n\
                            -2,-2,0,1\n\
                            0,2,1,3\n\
                            -2,3,-2,1\n\
                            0,2,3,-2\n\
                            -1,-1,1,-2\n\
                            0,-2,-1,0\n\
                            -2,2,3,-1\n\
                            1,2,2,0\n\
                            -1,-2,0,-2";
        let gen = generate(input);
        let res = part1(&gen);
        assert_eq!(res, 8);
    }
}
