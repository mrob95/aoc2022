fn parse(input: &str) -> Vec<Vec<i32>> {
    input.trim().split("\n").map(|line| {
        line.chars().rev().map(|d| {
            match d {
                '=' => -2,
                '-' => -1,
                '0' => 0,
                '1' => 1,
                '2' => 2,
                _ => unreachable!()
            }
        }).collect()
    }).collect()
}

pub fn part_one(input: &str) -> Option<String> {
    let amounts = parse(input);
    let mut result = Vec::new();
    for amount in amounts {
        for (i,d) in amount.iter().enumerate() {
            if i == result.len() {
                result.push(*d);
            } else {
                result[i] += d;
            }
        }
    }
    loop {
        let mut changed = false;
        for i in 0..result.len() {
            while result[i] > 2 {
                changed = true;
                result[i] -= 5;
                if i+1 == result.len() {
                    result.push(1);
                } else {
                    result[i+1] += 1;
                }
            }
            while result[i] < -2 {
                changed = true;
                result[i] += 5;
                if i+1 == result.len() {
                    result.push(-1);
                } else {
                    result[i+1] -= 1;
                }
            }
        }
        if !changed {
            break;
        }
    }
    let mut buf = String::new();
    for d in result.iter().rev() {
        buf.push(match d {
            -2 => '=',
            -1 => '-',
            0 => '0',
            1 => '1',
            2 => '2',
            _ => unreachable!()
        });
    }
    Some(buf)
}

pub fn part_two(input: &str) -> Option<String> {
    let readings = parse(input);
    None
}

fn main() {
    let input = &aoc::read_file("inputs", 25);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 25);
        assert_eq!(part_one(&input), Some("2=-1=0".to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 25);
        assert_eq!(part_two(&input), None);
    }
}
