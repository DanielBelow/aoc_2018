use aoc_runner_derive::{aoc, aoc_generator};
use num_complex::Complex;
use std::collections::HashMap;

const UP: Complex<i64> = Complex::new(0, -1);
const RIGHT: Complex<i64> = Complex::new(1, 0);
const DOWN: Complex<i64> = Complex::new(0, 1);
const LEFT: Complex<i64> = Complex::new(-1, 0);

fn complex_to_string(compl: Complex<i64>) -> String {
    format!("{},{}", compl.re, compl.im)
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub struct Cart {
    direction: Complex<i64>,
    position: Complex<i64>,
    choice_mod: u8, // [0,2]
    crashed: bool,
}

impl Cart {
    fn char_to_dir(chr: char) -> Complex<i64> {
        match chr {
            '<' => LEFT,
            '>' => RIGHT,
            'v' => DOWN,
            '^' => UP,
            _ => unreachable!(),
        }
    }

    fn new(position: Complex<i64>, chr: char) -> Self {
        let direction = Self::char_to_dir(chr);
        Self {
            direction,
            position,
            choice_mod: 0,
            crashed: false,
        }
    }

    fn tick(&mut self, turns: &HashMap<Complex<i64>, char>) {
        self.position += self.direction;

        let horizontal = self.direction == LEFT || self.direction == RIGHT;
        let vertical = self.direction == UP || self.direction == DOWN;

        if let Some(turn) = turns.get(&self.position) {
            match *turn {
                '/' if horizontal => self.turn_left(),
                '/' if vertical => self.turn_right(),
                '\\' if vertical => self.turn_left(),
                '\\' if horizontal => self.turn_right(),
                '+' => {
                    if self.choice_mod == 0 {
                        self.turn_left();
                    } else if self.choice_mod == 2 {
                        self.turn_right();
                    }

                    self.choice_mod = (self.choice_mod + 1) % 3;
                }
                _ => unreachable!(),
            };
        }
    }

    fn turn_left(&mut self) {
        self.direction *= -Complex::i();
    }

    fn turn_right(&mut self) {
        self.direction *= Complex::i();
    }
}

pub struct Map {
    turns: HashMap<Complex<i64>, char>,
    carts: Vec<Cart>,
}

#[aoc_generator(day13)]
pub fn generate(inp: &str) -> Map {
    let mut carts = vec![];
    let mut turns = HashMap::new();

    for (row_idx, line) in inp.lines().enumerate() {
        for (col_idx, chr) in line.chars().enumerate() {
            let position = Complex::new(col_idx as i64, row_idx as i64);
            match chr {
                '/' | '\\' | '+' => {
                    turns.insert(position, chr);
                }
                '<' | '>' | 'v' | '^' => carts.push(Cart::new(position, chr)),
                _ => {}
            };
        }
    }

    Map { turns, carts }
}

fn has_collision(carts: &mut [Cart], idx: usize) -> Option<&mut Cart> {
    let idx_position = carts[idx].position;

    carts
        .iter_mut()
        .enumerate()
        .find(|(cart_idx, cart)| idx != *cart_idx && cart.position == idx_position && !cart.crashed)
        .map(|(_, it)| it)
}

fn sort_carts(carts: &mut [Cart]) {
    carts.sort_unstable_by(|lhs, rhs| {
        if lhs.position.im == rhs.position.im {
            lhs.position.re.cmp(&rhs.position.re)
        } else {
            lhs.position.im.cmp(&rhs.position.im)
        }
    });
}

#[aoc(day13, part1)]
pub fn part1(map: &Map) -> String {
    let mut carts = map.carts.clone();

    loop {
        sort_carts(&mut carts);

        for idx in 0..carts.len() {
            carts[idx].tick(&map.turns);

            if has_collision(&mut carts, idx).is_some() {
                return complex_to_string(carts[idx].position);
            }
        }
    }
}

#[aoc(day13, part2)]
pub fn part2(map: &Map) -> String {
    let mut carts = map.carts.clone();

    loop {
        if carts.len() == 1 {
            return complex_to_string(carts[0].position);
        }

        sort_carts(&mut carts);

        for idx in 0..carts.len() {
            carts[idx].tick(&map.turns);

            if let Some(other) = has_collision(&mut carts, idx) {
                other.crashed = true;
                carts[idx].crashed = true;
            }
        }

        carts.retain(|it| !it.crashed);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PART1_TEST_DATA: &str = "/-<---\\    \n\
                                   |     |     \n\
                                   |  /--+--\\ \n\
                                   |  |  |  |  \n\
                                   \\--+--/  | \n\
                                      |     |  \n\
                                      \\-----/";

    #[test]
    fn test_parsing() {
        let parsed = generate(PART1_TEST_DATA);
        assert_eq!(parsed.carts.len(), 1);
        assert_eq!(parsed.turns.len(), 10);
    }

    #[test]
    fn test_moves() {
        let mut parsed = generate(PART1_TEST_DATA);
        parsed.carts[0].tick(&parsed.turns);
        assert_eq!(parsed.carts[0].position, Complex::new(1, 0));
        assert_eq!(parsed.carts[0].direction, LEFT);

        parsed.carts[0].tick(&parsed.turns);
        assert_eq!(parsed.carts[0].position, Complex::new(0, 0));
        assert_eq!(parsed.carts[0].direction, DOWN);
    }
}
