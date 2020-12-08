#![feature(test)]
#![feature(str_split_once)]

use std::collections::VecDeque;
use std::io::Read;

fn main() {
    let mut f = std::fs::File::open("src/bin/aoc08.txt").unwrap();
    let mut input = String::new();
    f.read_to_string(&mut input).unwrap();

    let program = parse(&input);

    let (_, trace) = run_program(&program).unwrap_err();

    let corrupted_pc = find_corrupted_pc(&program, &trace);
    let program = flip_instruction(&program, corrupted_pc);

    let (state, _) = run_program(&program).unwrap();

    eprintln!("Final CPU state: {:?}", state);
    assert_eq!(1205, state.acc);
}

#[derive(Copy, Clone, Debug)]
enum Op {
    Nop(i64),
    Acc(i64),
    Jmp(i64),
}

#[derive(Copy, Clone, Debug, Default)]
struct CpuState {
    pc: usize,
    acc: i64,
}

fn run_program(program: &[Op]) -> Result<(CpuState, Vec<usize>), (CpuState, Vec<usize>)> {
    let mut state = CpuState::default();
    let mut trace = vec![0usize; program.len()];
    let exit_pc = program.len();
    loop {
        if state.pc == exit_pc {
            return Ok((state, trace));
        }
        if trace[state.pc] != 0 {
            return Err((state, trace));
        }
        trace[state.pc] += 1;
        state = execute_op(state, &program[state.pc]);
    }
}

fn execute_op(cpu: CpuState, op: &Op) -> CpuState {
    match op {
        Op::Nop(_) => CpuState {
            pc: pc_offset(cpu.pc, 1),
            acc: cpu.acc,
        },
        Op::Acc(n) => CpuState {
            pc: pc_offset(cpu.pc, 1),
            acc: cpu.acc + n,
        },
        Op::Jmp(n) => CpuState {
            pc: pc_offset(cpu.pc, *n),
            acc: cpu.acc,
        },
    }
}

fn pc_offset(pc: usize, offset: i64) -> usize {
    ((pc as i64) + offset) as usize
}

fn parse(s: &str) -> Vec<Op> {
    fn parse_line(line: &str) -> Op {
        let (op_name, op_arg) = line.split_once(' ').unwrap();
        match op_name {
            "nop" => Op::Nop(op_arg.parse().unwrap()),
            "acc" => Op::Acc(op_arg.parse().unwrap()),
            "jmp" => Op::Jmp(op_arg.parse().unwrap()),
            _ => unreachable!(),
        }
    }

    s.lines().map(parse_line).collect()
}

fn build_reverse_pc_tables(program: &[Op]) -> (Vec<Vec<usize>>, Vec<Vec<usize>>) {
    let mut unmodified = vec![vec![]; program.len() + 1];
    let mut modified = vec![vec![]; program.len() + 1];
    for pc in 0..program.len() {
        match program[pc] {
            Op::Nop(n) => {
                unmodified[pc_offset(pc, 1)].push(pc);
                modified[pc_offset(pc, n)].push(pc)
            }
            Op::Acc(_) => unmodified[pc_offset(pc, 1)].push(pc),
            Op::Jmp(n) => {
                unmodified[pc_offset(pc, n)].push(pc);
                modified[pc_offset(pc, 1)].push(pc)
            }
        }
    }

    (unmodified, modified)
}

fn find_corrupted_pc(program: &[Op], trace: &[usize]) -> usize {
    let (unmodified, modified) = build_reverse_pc_tables(program);
    let mut candidates: VecDeque<usize> = VecDeque::new();
    candidates.push_front(program.len());

    loop {
        let pc = candidates.pop_back().unwrap();
        if let Some(modified_pc) = modified[pc].iter().find(|pc| trace[**pc] > 0) {
            return *modified_pc;
        }
        candidates.extend(&unmodified[pc]);
    }
}

fn flip_instruction(program: &[Op], pc: usize) -> Vec<Op> {
    let mut new_program = program.to_vec();
    new_program[pc] = match new_program[pc] {
        Op::Nop(n) => Op::Jmp(n),
        Op::Jmp(n) => Op::Nop(n),
        _ => unreachable!(),
    };
    new_program
}

#[cfg(test)]
mod tests {
    extern crate test;

    use test::bench::Bencher;

    use crate::*;

    const EXAMPLE: &str = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6
";

    #[test]
    fn test_program_fix() {
        let program = parse(EXAMPLE);
        let (_, trace) = run_program(&program).unwrap_err();

        let corrupted_pc = find_corrupted_pc(&program, &trace);
        let program = flip_instruction(&program, corrupted_pc);

        let (state, _) = run_program(&program).unwrap();

        eprintln!("Final CPU state: {:?}", state);
        assert_eq!(8, state.acc);
    }

    #[bench]
    fn bench_program_fix(bencher: &mut Bencher) {
        let mut f = std::fs::File::open("src/bin/aoc08.txt").unwrap();
        let mut input = String::new();
        f.read_to_string(&mut input).unwrap();


        bencher.iter(move || {
            let program = parse(&input);
            let (_, trace) = run_program(&program).unwrap_err();

            let corrupted_pc = find_corrupted_pc(&program, &trace);
            let program = flip_instruction(&program, corrupted_pc);

            let (state, _) = run_program(&program).unwrap();
            state
        });
    }
}
