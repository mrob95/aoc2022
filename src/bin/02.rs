fn parse(input: &str) -> Vec<(&str, &str)> {
    input
        .trim()
        .split("\n")
        .map(|s| s.split_once(" ").unwrap())
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let games = parse(input);
    let mut result = 0;
    for game in games {
        result += match game.0 {
            "A" => match game.1 {
                "X" => 1 + 3,
                "Y" => 2 + 6,
                "Z" => 3 + 0,
                _ => panic!("Unexpected response {}", game.1),
            },
            "B" => match game.1 {
                "X" => 1 + 0,
                "Y" => 2 + 3,
                "Z" => 3 + 6,
                _ => panic!("Unexpected response {}", game.1),
            },
            "C" => match game.1 {
                "X" => 1 + 6,
                "Y" => 2 + 0,
                "Z" => 3 + 3,
                _ => panic!("Unexpected response {}", game.1),
            },
            _ => panic!("Unexpected play {}", game.1),
        }
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let games = parse(input);
    let mut result = 0;
    for game in games {
        result += match game.0 {
            "A" => match game.1 {
                "X" => 3 + 0,
                "Y" => 1 + 3,
                "Z" => 2 + 6,
                _ => panic!("Unexpected response {}", game.1),
            },
            "B" => match game.1 {
                "X" => 1 + 0,
                "Y" => 2 + 3,
                "Z" => 3 + 6,
                _ => panic!("Unexpected response {}", game.1),
            },
            "C" => match game.1 {
                "X" => 2 + 0,
                "Y" => 3 + 3,
                "Z" => 1 + 6,
                _ => panic!("Unexpected response {}", game.1),
            },
            _ => panic!("Unexpected play {}", game.1),
        }
    }
    Some(result)
}

fn main() {
    let input = &aoc::read_file("inputs", 2);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
