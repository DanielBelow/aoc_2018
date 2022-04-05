use aoc_runner_derive::aoc;
use std::collections::HashSet;

fn run_program(part_one: bool) -> Option<usize> {
    let r0 = 0;

    let mut seen = HashSet::new();
    let mut prev = 0;

    // ip = 5 - seti 0 7 1
    let mut r1 = 0;

    loop {
        // LABEL4: ip = 6 - bori 1 65536 2
        let mut r2 = r1 | 65_536_usize;

        // ip = 7 - seti 8725355 6 1
        r1 = 8_725_355;

        loop {
            // ip = 9 - addr 1 5 1
            r1 += r2 & 255;

            // ip = 10 - bani 1 16777215 1
            r1 &= 16_777_215;

            // ip = 11 - muli 1 65899 1
            r1 *= 65899;

            // ip = 12 - bani 1 16777215 1
            r1 &= 16_777_215;

            if r2 < 256 {
                break;
            }

            r2 /= 256;
        }

        if part_one {
            return Some(r1);
        } else if seen.contains(&r1) {
            return Some(prev);
        }

        seen.insert(r1);
        prev = r1;

        // ip = 28 - eqrr 1 0 5
        if r1 == r0 {
            return None;
        }
    }
}

#[aoc(day21, part1)]
pub fn part1(_: &str) -> Option<usize> {
    run_program(true)
}

#[aoc(day21, part2)]
pub fn part2(_: &str) -> Option<usize> {
    run_program(false)
}
