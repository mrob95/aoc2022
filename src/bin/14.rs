use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
enum Item {
    Rock,
    Sand,
}

fn parse(input: &str) -> HashMap<[usize; 2], Item> {
    let mut result = HashMap::new();
    for line in input.trim().split("\n") {
        let pairs = line.split(" -> ").map(|s| {
            let (x, y) = s.split_once(",").unwrap();
            (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap())
        });
        for pair in pairs.collect::<Vec<(usize, usize)>>().windows(2) {
            let (left, right) = (pair[0], pair[1]);
            if left.0 == right.0 {
                if left.1 <= right.1 {
                    for y in left.1..right.1 + 1 {
                        result.insert([left.0, y], Item::Rock);
                    }
                } else {
                    for y in right.1..left.1 + 1 {
                        result.insert([left.0, y], Item::Rock);
                    }
                }
            } else {
                if left.0 <= right.0 {
                    for x in left.0..right.0 + 1 {
                        result.insert([x, left.1], Item::Rock);
                    }
                } else {
                    for x in right.0..left.0 + 1 {
                        result.insert([x, left.1], Item::Rock);
                    }
                }
            }
        }
    }
    result
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut map = parse(input);
    let abyss = map.keys().max_by_key(|v| v[1]).unwrap()[1];
    let mut result = 0;
    loop {
        let mut sand_pos = [500, 0];
        loop {
            if sand_pos[1] >= abyss {
                return Some(result);
            }
            let mut done = true;
            for pos in [
                [sand_pos[0], sand_pos[1] + 1],
                [sand_pos[0] - 1, sand_pos[1] + 1],
                [sand_pos[0] + 1, sand_pos[1] + 1],
            ] {
                if !map.contains_key(&pos) {
                    sand_pos = pos;
                    done = false;
                    break;
                }
            }
            if done {
                break;
            }
        }
        map.insert(sand_pos, Item::Sand);
        result += 1;
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut map = parse(input);
    let floor = map.keys().max_by_key(|v| v[1]).unwrap()[1] + 2;
    let mut result = 0;
    loop {
        let mut sand_pos = [500, 0];
        if map.contains_key(&sand_pos) {
            break; // done
        }
        loop {
            if sand_pos[1] == floor - 1 {
                break;
            }
            let mut done = true;
            for pos in [
                [sand_pos[0], sand_pos[1] + 1],
                [sand_pos[0] - 1, sand_pos[1] + 1],
                [sand_pos[0] + 1, sand_pos[1] + 1],
            ] {
                if !map.contains_key(&pos) {
                    sand_pos = pos;
                    done = false;
                    break;
                }
            }
            if done {
                break;
            }
        }
        map.insert(sand_pos, Item::Sand);
        result += 1;
    }
    Some(result)
}

fn main() {
    let input = &aoc::read_file("inputs", 14);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 14);
        assert_eq!(part_one(&input), Some(24));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 14);
        assert_eq!(part_two(&input), Some(93));
    }
}
