use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum Regex {
    Empty,
    Terminal(String),
    Nested(Vec<Regex>),
    Choice(Vec<Regex>),
}

impl Regex {
    fn longest_match(&self) -> usize {
        match self {
            Regex::Empty => 0,
            Regex::Terminal(s) => s.len(),
            Regex::Nested(res) => res.iter().map(Regex::longest_match).sum(),
            Regex::Choice(choices) => {
                if choices.iter().any(|it| matches!(it, Regex::Empty)) {
                    0
                } else {
                    choices.iter().map(Regex::longest_match).max().unwrap_or(0)
                }
            }
        }
    }
}

struct ParseState {
    parsed: Regex,
    remaining: String,
}

fn parse_regex_choices(inp: &str) -> ParseState {
    assert!(inp.starts_with('('));
    let mut inp = inp.chars().dropping(1).collect::<String>();

    let mut choices = vec![];

    while !inp.is_empty() && !inp.starts_with(')') {
        if inp.starts_with('(') {
            let nested_state = parse_regex_choices(&inp);
            inp = nested_state.remaining;
            choices.push(nested_state.parsed);
        } else if !inp.starts_with('|') {
            let mut nested = vec![];

            while !inp.starts_with('|') && !inp.starts_with(')') {
                let s = if inp.starts_with('(') {
                    parse_regex_choices(&inp)
                } else {
                    parse_regex(&inp)
                };

                inp = s.remaining;
                nested.push(s.parsed);
            }

            let nested_state = parse_regex(&inp);
            inp = nested_state.remaining;

            if nested.len() > 1 {
                choices.push(Regex::Nested(nested));
            } else {
                choices.push(nested[0].clone());
            }
        } else if inp.starts_with("|)") {
            inp.drain(..1);
            choices.push(Regex::Empty);
        } else {
            assert!(inp.starts_with('|'));
            inp.drain(..1);
        }
    }

    if inp.starts_with(')') {
        inp.drain(..1);
    }

    ParseState {
        parsed: Regex::Choice(choices),
        remaining: inp,
    }
}

fn parse_regex(inp: &str) -> ParseState {
    let mut inp = inp.to_string();

    let plain = inp
        .chars()
        .take_while(|it| *it != '(' && *it != ')' && *it != '|')
        .join("");
    inp.drain(..plain.len());

    ParseState {
        parsed: Regex::Terminal(plain),
        remaining: inp,
    }
}

#[aoc_generator(day20)]
pub fn generate(inp: &str) -> Vec<Regex> {
    let mut inp = inp.chars().dropping(1).dropping_back(1).join("");
    let mut regexes = vec![];

    while !inp.is_empty() {
        let state = if inp.starts_with('(') {
            parse_regex_choices(&inp)
        } else {
            parse_regex(&inp)
        };

        inp = state.remaining;
        regexes.push(state.parsed);
    }

    regexes
}

fn count_doors_shortest_path(regexes: &[Regex]) -> usize {
    regexes.iter().map(Regex::longest_match).sum()
}

#[aoc(day20, part1)]
pub fn part1(regexes: &[Regex]) -> usize {
    count_doors_shortest_path(regexes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_simple_regex() {
        let inp = "^NSWE$";
        let re = generate(inp);
        assert_eq!(re.len(), 1);
        assert_eq!(re[0], Regex::Terminal("NSWE".to_string()));
    }

    #[test]
    fn parse_regex_choice() {
        let inp = "^N(S|W)E$";
        let re = generate(inp);
        assert_eq!(re.len(), 3);
        assert_eq!(re[0], Regex::Terminal("N".to_string()));
        assert_eq!(
            re[1],
            Regex::Choice(vec![
                Regex::Terminal("S".to_string()),
                Regex::Terminal("W".to_string()),
            ])
        );
        assert_eq!(re[2], Regex::Terminal("E".to_string()));
    }

    #[test]
    fn parse_regex_empty_choice() {
        let inp = "^N(S|W|)E$";
        let re = generate(inp);
        assert_eq!(re.len(), 3);
        assert_eq!(re[0], Regex::Terminal("N".to_string()));
        assert_eq!(
            re[1],
            Regex::Choice(vec![
                Regex::Terminal("S".to_string()),
                Regex::Terminal("W".to_string()),
                Regex::Empty,
            ])
        );
        assert_eq!(re[2], Regex::Terminal("E".to_string()));
    }

    #[test]
    fn nested_choices() {
        let inp = "^(NEEE|SSE(EE|N))$";
        let re = generate(inp);
        assert_eq!(re.len(), 1);
        assert_eq!(
            re[0],
            Regex::Choice(vec![
                Regex::Terminal("NEEE".to_string()),
                Regex::Nested(vec![
                    Regex::Terminal("SSE".to_string()),
                    Regex::Choice(vec![
                        Regex::Terminal("EE".to_string()),
                        Regex::Terminal("N".to_string())
                    ])
                ])
            ])
        );
    }

    #[test]
    fn test_sample_p1() {
        let inp = "^ENWWW(NEEE|SSE(EE|N))$";
        let re = generate(inp);

        let result = part1(&re);
        assert_eq!(result, 10);
    }
}
