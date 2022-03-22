use aoc_runner_derive::{aoc, aoc_generator};
use itertools::{iproduct, Itertools};
use parse_display::{Display, FromStr};

#[derive(Display, FromStr, Copy, Clone, Debug)]
#[display("#{id} @ {x_pos},{y_pos}: {width}x{height}")]
pub struct Claim {
    id: usize,
    x_pos: usize,
    y_pos: usize,
    width: usize,
    height: usize,
}

impl Claim {
    fn overlaps_1d((l_min, l_max): (usize, usize), (r_min, r_max): (usize, usize)) -> bool {
        l_max > r_min && r_max > l_min
    }

    pub fn overlaps_with(&self, other: &Claim) -> bool {
        Claim::overlaps_1d(
            (self.x_pos, self.x_pos + self.width),
            (other.x_pos, other.x_pos + other.width),
        ) && Claim::overlaps_1d(
            (self.y_pos, self.y_pos + self.height),
            (other.y_pos, other.y_pos + other.height),
        )
    }
}

#[aoc_generator(day03)]
pub fn generate(inp: &str) -> Vec<Claim> {
    inp.lines().filter_map(|it| it.parse().ok()).collect_vec()
}

#[aoc(day03, part1)]
pub fn part1(claims: &[Claim]) -> usize {
    let mut fabric = vec![vec![0usize; 1_000]; 1_000];

    for claim in claims {
        for (x, y) in iproduct!(0..claim.width, 0..claim.height) {
            fabric[claim.x_pos + x][claim.y_pos + y] += 1;
        }
    }

    fabric
        .iter()
        .flat_map(Clone::clone)
        .fold(0, |acc, it| acc + if it > 1 { 1 } else { 0 })
}

fn has_no_overlap(claim: &Claim, other_claims: &[&Claim]) -> bool {
    other_claims.iter().all(|it| !claim.overlaps_with(it))
}

#[aoc(day03, part2)]
pub fn part2(claims: &[Claim]) -> usize {
    for claim in claims {
        let other_claims = claims.iter().filter(|it| it.id != claim.id).collect_vec();
        assert_eq!(other_claims.len(), claims.len() - 1);
        if has_no_overlap(claim, &other_claims) {
            return claim.id;
        }
    }

    unreachable!("Everything overlaps somehow")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_p1() {
        const INP: &str = "#1 @ 1,3: 4x4\n\
                           #2 @ 3,1: 4x4\n\
                           #3 @ 5,5: 2x2";

        let data = generate(INP);
        let res = part1(&data);

        assert_eq!(res, 4);
    }

    #[test]
    fn test_sample_p2() {
        const INP: &str = "#1 @ 1,3: 4x4\n\
                           #2 @ 3,1: 4x4\n\
                           #3 @ 5,5: 2x2";

        let data = generate(INP);
        let res = part2(&data);

        assert_eq!(res, 3);
    }
}
