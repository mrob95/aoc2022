use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug)]
struct CPU {
    reg: i32,
    ip: usize,
    clock: i32,
    iclock: i32,
    ops: Vec<Op>,
}

impl CPU {
    fn new(ops: Vec<Op>) -> Self {
        CPU {
            reg: 1,
            ip: 0,
            clock: 0,
            iclock: 0,
            ops: ops,
        }
    }
    fn step_clock(self: &mut Self) {
        self.clock += 1;
    }
    fn run_op(self: &mut Self) {
        let op = self.ops[self.ip];
        match op {
            Op::Noop => {
                self.ip += 1;
            }
            Op::Addx(n) => match self.iclock {
                0 => self.iclock += 1,
                1 => {
                    self.iclock = 0;
                    self.reg += n;
                    self.ip += 1;
                }
                _ => panic!("Unexpected instruction clock {}", self.iclock),
            },
        }
    }
    fn running(self: &Self) -> bool {
        self.ip < self.ops.len()
    }
}

#[derive(Debug, Copy, Clone)]
enum Op {
    Noop,
    Addx(i32),
}

impl FromStr for Op {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(" ").collect();
        match parts[0] {
            "noop" => Ok(Op::Noop),
            "addx" => Ok(Op::Addx(parts[1].parse()?)),
            _ => panic!("Unknown op {}", parts[0]),
        }
    }
}

fn parse(input: &str) -> Vec<Op> {
    input
        .trim()
        .split("\n")
        .map(|s| s.parse().unwrap())
        .collect()
}

pub fn part_one(input: &str) -> Option<i32> {
    let ops = parse(input);
    let mut cpu = CPU::new(ops);
    let mut result = 0;
    while cpu.running() {
        cpu.step_clock();
        if [20, 60, 100, 140, 180, 220].contains(&cpu.clock) {
            result += cpu.clock * cpu.reg;
        }
        cpu.run_op();
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let ops = parse(input);
    let mut cpu = CPU::new(ops);
    while cpu.running() {
        cpu.step_clock();
        let pos = (cpu.clock - 1) % 40;
        if pos == 0 {
            print!("\n")
        }
        if pos >= cpu.reg - 1 && pos <= cpu.reg + 1 {
            print!("#")
        } else {
            print!(".")
        }
        cpu.run_op();
    }
    None
}

fn main() {
    let input = &aoc::read_file("inputs", 10);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(13140));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 10);
        assert_eq!(part_two(&input), None);
    }
}
