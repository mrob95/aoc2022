use std::collections::VecDeque;

#[derive(Debug)]
struct Monkey {
    items: VecDeque<u64>,
    operation: Op,
    test: u64,
    if_true: usize,
    if_false: usize,
    inspected: u64,
}

#[derive(Debug)]
enum Op {
    Mul(u64),
    Add(u64),
    Square,
}

fn parse(input: &str) -> Vec<Monkey> {
    input
        .trim()
        .split("\n\n")
        .map(|m| {
            let mut lines = m.split("\n").skip(1);

            let items_str = lines.next().unwrap().split_once(": ").unwrap().1;
            let items = items_str
                .split(", ")
                .map(|s| s.parse::<u64>().unwrap())
                .collect();

            let op_str = lines.next().unwrap().split_once(" = ").unwrap().1;
            let op_parts: Vec<&str> = op_str.split(" ").collect();
            let operation = match op_parts[1] {
                "*" => {
                    if op_parts[2] == "old" {
                        Op::Square
                    } else {
                        Op::Mul(op_parts[2].parse::<u64>().unwrap())
                    }
                }
                "+" => Op::Add(op_parts[2].parse::<u64>().unwrap()),
                _ => panic!("Unrecognised operation {}", op_parts[1]),
            };

            let test = lines
                .next()
                .unwrap()
                .split_once("by ")
                .unwrap()
                .1
                .parse::<u64>()
                .unwrap();

            let if_true = lines
                .next()
                .unwrap()
                .split_once("monkey ")
                .unwrap()
                .1
                .parse()
                .unwrap();
            let if_false = lines
                .next()
                .unwrap()
                .split_once("monkey ")
                .unwrap()
                .1
                .parse()
                .unwrap();

            Monkey {
                items,
                operation,
                test,
                if_true,
                if_false,
                inspected: 0,
            }
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut monkeys = parse(input);
    for _ in 0..20 {
        for i in 0..monkeys.len() {
            let if_true = monkeys[i].if_true;
            let if_false = monkeys[i].if_false;
            while monkeys[i].items.len() > 0 {
                let mut worry = monkeys[i].items.pop_front().unwrap();
                worry = match monkeys[i].operation {
                    Op::Add(n) => worry + n,
                    Op::Mul(n) => worry * n,
                    Op::Square => worry * worry,
                };
                worry = worry / 3;
                if worry % monkeys[i].test == 0 {
                    monkeys[if_true].items.push_back(worry);
                } else {
                    monkeys[if_false].items.push_back(worry);
                }
                monkeys[i].inspected += 1;
            }
        }
    }
    let mut inspected_counts = monkeys.iter().map(|m| m.inspected).collect::<Vec<u64>>();
    inspected_counts.sort_by(|a, b| b.cmp(a));
    Some(inspected_counts[0] * inspected_counts[1])
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut monkeys = parse(input);
    // We need to avoid levels of worry growing unbounded and overflowing.
    // Since all tests are for divisibility and we don't care about absolute worry levels,
    // we can mod everything by the product of all of the divisibility test values and
    // preserve the results of those tests.
    // e.g. if tests are "divisible by 2" and "divisible by 3", mod everything by 6.
    // 15 => false, true
    // 15 % 6 = 3 => false, true
    let field_bound = monkeys.iter().map(|m| m.test).fold(1, |acc, e| acc * e);
    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            let if_true = monkeys[i].if_true;
            let if_false = monkeys[i].if_false;
            while monkeys[i].items.len() > 0 {
                let mut worry = monkeys[i].items.pop_front().unwrap();
                worry = match monkeys[i].operation {
                    Op::Add(n) => worry + n,
                    Op::Mul(n) => worry * n,
                    Op::Square => worry * worry,
                };
                worry = worry % field_bound;
                if worry % monkeys[i].test == 0 {
                    monkeys[if_true].items.push_back(worry);
                } else {
                    monkeys[if_false].items.push_back(worry);
                }
                monkeys[i].inspected += 1;
            }
        }
    }
    let mut inspected_counts = monkeys.iter().map(|m| m.inspected).collect::<Vec<u64>>();
    inspected_counts.sort_by(|a, b| b.cmp(a));
    Some(inspected_counts[0] * inspected_counts[1])}

fn main() {
    let input = &aoc::read_file("inputs", 11);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 11);
        assert_eq!(part_one(&input), Some(10605));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 11);
        assert_eq!(part_two(&input), Some(2713310158));
    }
}
