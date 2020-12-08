#![feature(test)]
#![feature(str_split_once)]

use std::io::Read;

fn main() {
    let mut f = std::fs::File::open("src/bin/aoc08.txt").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();

    let program = parse(&s);

    let (state, _) = run_program(&program).unwrap_err();

    eprintln!("Final CPU state {:?}", state);
    assert_eq!(1134, state.acc);
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

#[cfg(test)]
mod tests {
    extern crate test;

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
    fn test_program() {
        let program = parse(EXAMPLE);

        let (state, _) = run_program(&program).unwrap_err();

        eprintln!("Final CPU state: {:?}", state);
        assert_eq!(5, state.acc);
    }
}
