use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::ops::{BitAnd, BitOr};

pub struct InstructionSamples {
    before: [usize; 4],
    inst: [usize; 4],
    after: [usize; 4],
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
enum Instruction {
    Addr,
    Addi,

    Mulr,
    Muli,

    Banr,
    Bani,

    Borr,
    Bori,

    Setr,
    Seti,

    Gtir,
    Gtri,
    Gtrr,

    Eqir,
    Eqri,
    Eqrr,
}

fn execute_inst(mut regs: [usize; 4], inst: [usize; 4], kind: Instruction) -> [usize; 4] {
    let [_, a, b, c] = inst;

    regs[c] = match kind {
        Instruction::Addr => regs[a] + regs[b],
        Instruction::Addi => regs[a] + b,
        Instruction::Mulr => regs[a] * regs[b],
        Instruction::Muli => regs[a] * b,
        Instruction::Banr => regs[a].bitand(regs[b]),
        Instruction::Bani => regs[a].bitand(b),
        Instruction::Borr => regs[a].bitor(regs[b]),
        Instruction::Bori => regs[a].bitor(b),
        Instruction::Setr => regs[a],
        Instruction::Seti => a,
        Instruction::Gtir => usize::from(a > regs[b]),
        Instruction::Gtri => usize::from(regs[a] > b),
        Instruction::Gtrr => usize::from(regs[a] > regs[b]),
        Instruction::Eqir => usize::from(a == regs[b]),
        Instruction::Eqri => usize::from(regs[a] == b),
        Instruction::Eqrr => usize::from(regs[a] == regs[b]),
    };

    regs
}

fn parse_register_values(line: &str, prefix: &str) -> [usize; 4] {
    let res = line
        .strip_prefix(prefix)
        .unwrap()
        .replace(['[', ']'], "")
        .split(", ")
        .filter_map(|it| it.trim().parse::<usize>().ok())
        .collect_vec();

    [res[0], res[1], res[2], res[3]]
}

fn parse_instruction(line: &str) -> [usize; 4] {
    let res = line
        .split(' ')
        .filter_map(|it| it.parse::<usize>().ok())
        .collect_vec();
    [res[0], res[1], res[2], res[3]]
}

#[aoc_generator(day16)]
pub fn generate(inp: &str) -> (Vec<InstructionSamples>, Vec<[usize; 4]>) {
    let inp = inp.split("\n\n\n").collect_vec();
    assert_eq!(inp.len(), 2);

    if let [input_output, instructions] = inp[..2] {
        let input_lines = input_output
            .lines()
            .filter(|it| !it.is_empty())
            .collect_vec();

        let mut result = vec![];

        for chunk in input_lines.chunks(3) {
            if let [before, inst, after] = chunk {
                let before = parse_register_values(before, "Before: ");
                let after = parse_register_values(after, "After: ");
                let inst = inst
                    .split_ascii_whitespace()
                    .filter_map(|it| it.parse::<usize>().ok())
                    .collect_vec();

                result.push(InstructionSamples {
                    before,
                    inst: [inst[0], inst[1], inst[2], inst[3]],
                    after,
                });
            }
        }

        let mut insts = vec![];
        for inst in instructions.lines().filter(|it| !it.is_empty()) {
            let parsed_inst = parse_instruction(inst);
            insts.push(parsed_inst);
        }

        return (result, insts);
    }

    unreachable!("Couldn't parse input")
}

fn all_instruction_kinds() -> [Instruction; 16] {
    [
        Instruction::Addr,
        Instruction::Addi,
        Instruction::Mulr,
        Instruction::Muli,
        Instruction::Banr,
        Instruction::Bani,
        Instruction::Borr,
        Instruction::Bori,
        Instruction::Setr,
        Instruction::Seti,
        Instruction::Gtir,
        Instruction::Gtri,
        Instruction::Gtrr,
        Instruction::Eqir,
        Instruction::Eqri,
        Instruction::Eqrr,
    ]
}

fn count_matching_insts(inp: &InstructionSamples) -> Vec<Instruction> {
    let mut result = vec![];

    for kind in all_instruction_kinds() {
        let evaluated = execute_inst(inp.before, inp.inst, kind);
        if evaluated == inp.after {
            result.push(kind);
        }
    }

    result
}

#[aoc(day16, part1)]
pub fn part1((samples, _): &(Vec<InstructionSamples>, Vec<[usize; 4]>)) -> usize {
    let mut result = 0;

    for sample in samples {
        let matching_insts = count_matching_insts(sample);
        if matching_insts.len() >= 3 {
            result += 1;
        }
    }

    result
}

#[aoc(day16, part2)]
pub fn part2((samples, program): &(Vec<InstructionSamples>, Vec<[usize; 4]>)) -> usize {
    let mut map = HashMap::new();

    for sample in samples {
        let matching_insts = count_matching_insts(sample);
        let entry = map.entry(sample.inst[0]).or_insert_with(HashSet::new);
        for inst in matching_insts {
            entry.insert(inst);
        }
    }

    loop {
        if map.values().all(|it| it.len() == 1) {
            break;
        }

        let known_insts = map
            .values()
            .filter(|it| it.len() == 1)
            .flatten()
            .copied()
            .collect_vec();

        for inst in known_insts {
            for hs in map
                .values_mut()
                .filter(|it| it.contains(&inst) && it.len() > 1)
            {
                hs.remove(&inst);
            }
        }
    }

    let inst_map = map
        .iter()
        .map(|(k, v)| (*k, *v.iter().exactly_one().unwrap()))
        .collect::<HashMap<_, _>>();

    let mut regs = [0usize; 4];
    for inst in program {
        let inst_to_execute = inst_map.get(&inst[0]).unwrap();
        regs = execute_inst(regs, *inst, *inst_to_execute);
    }

    regs[0]
}
