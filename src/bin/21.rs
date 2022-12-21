use std::collections::HashMap;

struct Operation {
    result: String,
    left: String,
    op: char,
    right: String,
    resolved: bool,
}

fn parse(input: &str) -> (HashMap<String, i64>, Vec<Operation>) {
    let mut operations = Vec::new();
    let mut values = HashMap::new();
    for line in input.trim().split("\n") {
        let (result, command) = line.split_once(": ").unwrap();
        if let Ok(val) = command.parse::<i64>() {
            values.insert(result.to_string(), val);
        } else {
            let mut parts = command.split(" ");
            let left = parts.next().unwrap().to_string();
            let op = parts.next().unwrap().chars().next().unwrap();
            let right = parts.next().unwrap().to_string();
            operations.push(Operation {
                result: result.to_string(),
                left,
                op,
                right,
                resolved: false,
            });
        }
    }
    (values, operations)
}

pub fn part_one(input: &str) -> Option<i64> {
    let (mut values, mut operations) = parse(input);
    while !values.contains_key("root") {
        for operation in operations.iter_mut() {
            if !operation.resolved
                && values.contains_key(&operation.left)
                && values.contains_key(&operation.right)
            {
                let left_value = values.get(&operation.left).unwrap();
                let right_value = values.get(&operation.right).unwrap();
                let result = match operation.op {
                    '+' => left_value + right_value,
                    '-' => left_value - right_value,
                    '*' => left_value * right_value,
                    '/' => left_value / right_value,
                    _ => panic!("Unrecognised op {}", operation.op),
                };
                values.insert(operation.result.clone(), result);
                operation.resolved = true;
            }
        }
    }
    Some(*values.get("root").unwrap())
}

pub fn part_two(input: &str) -> Option<i64> {
    let (mut values, mut operations) = parse(input);
    values.remove("humn");
    while !values.contains_key("humn") {
        for operation in operations.iter_mut().filter(|op| !op.resolved) {
            if values.contains_key(&operation.left) && values.contains_key(&operation.right) {
                let left_value = values.get(&operation.left).unwrap();
                let right_value = values.get(&operation.right).unwrap();
                let result = match operation.op {
                    '+' => left_value + right_value,
                    '-' => left_value - right_value,
                    '*' => left_value * right_value,
                    '/' => left_value / right_value,
                    _ => panic!("Unrecognised op {}", operation.op),
                };
                values.insert(operation.result.clone(), result);
                operation.resolved = true;
            } else if values.contains_key(&operation.result) && values.contains_key(&operation.left)
            {
                let left_value = values.get(&operation.left).unwrap();
                let result = values.get(&operation.result).unwrap();
                let right_value = match operation.op {
                    '+' => result - left_value,
                    '-' => left_value - result,
                    '*' => result / left_value,
                    '/' => left_value / result,
                    _ => panic!("Unrecognised op {}", operation.op),
                };
                values.insert(operation.right.clone(), right_value);
                operation.resolved = true;
            } else if values.contains_key(&operation.result)
                && values.contains_key(&operation.right)
            {
                let right_value = values.get(&operation.right).unwrap();
                let result = values.get(&operation.result).unwrap();
                let left_value = match operation.op {
                    '+' => result - right_value,
                    '-' => right_value + result,
                    '*' => result / right_value,
                    '/' => right_value * result,
                    _ => panic!("Unrecognised op {}", operation.op),
                };
                values.insert(operation.left.clone(), left_value);
                operation.resolved = true;
            } else if operation.result == "root" {
                if values.contains_key(&operation.left) {
                    let left_value = values.get(&operation.left).unwrap();
                    values.insert(operation.right.clone(), *left_value);
                    operation.resolved = true;
                } else if values.contains_key(&operation.right) {
                    let right_value = values.get(&operation.right).unwrap();
                    values.insert(operation.left.clone(), *right_value);
                    operation.resolved = true;
                }
            }
        }
    }
    Some(*values.get("humn").unwrap())
}

fn main() {
    let input = &aoc::read_file("inputs", 21);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 21);
        assert_eq!(part_one(&input), Some(152));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 21);
        assert_eq!(part_two(&input), Some(301));
    }
}
