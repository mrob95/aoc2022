fn parse(input: &str) -> Vec<&str> {
    input.trim().split("\n").collect()
}

fn score(c: char) -> u32 {
    match c.is_ascii_lowercase() {
        true => (c as u32 - 'a' as u32) + 1,
        false => (c as u32 - 'A' as u32) + 27,
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let sacks = parse(input);
    let mut result = 0;
    for (left, right) in sacks.iter().map(|s| s.split_at(s.len() / 2)) {
        for c in left.chars() {
            if right.contains(c) {
                result += score(c);
                break;
            }
        }
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let sacks = parse(input);
    let mut result = 0;
    for group in sacks.chunks(3) {
        let (first, second, third) = (group[0], group[1], group[2]);
        // Theoretically we should collect second and third into HashSet<char> or
        // something to avoid the O(n^2) lookups, but the input lines are short enough
        // that this actually makes the loop far slower, even though it improves the
        // time complexity.
        for c in first.chars() {
            if second.contains(c) && third.contains(c) {
                result += score(c);
                break;
            }
        }
    }
    Some(result)
}

fn main() {
    let input = &aoc::read_file("inputs", 3);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
