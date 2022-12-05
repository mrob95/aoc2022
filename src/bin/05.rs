#[derive(Debug, Default)]
struct Crates {
    positions: Vec<Vec<char>>,
    moves: Vec<(usize, usize, usize)>,
}

fn parse(input: &str) -> Crates {
    let (pos, mvs) = input.split_once("\n\n").unwrap();
    let mut positions = Vec::new();
    let mut moves = Vec::new();
    for line in pos.split("\n").collect::<Vec<&str>>().iter().rev().skip(1) {
        for (i, crat) in line.chars().collect::<Vec<char>>().chunks(4).enumerate() {
            if positions.len() <= i {
                positions.push(vec![]);
            }
            if crat[1] != ' ' {
                positions[i].push(crat[1]);
            }
        }
    }
    for line in mvs.trim().split("\n") {
        let items: Vec<&str> = line.split(" ").collect();
        moves.push((
            items[1].parse().unwrap(),
            items[3].parse().unwrap(),
            items[5].parse().unwrap(),
        ))
    }
    Crates { positions, moves }
}

pub fn part_one(input: &str) -> Option<String> {
    let crates = parse(input);
    let mut positions = crates.positions.clone();
    for mv in crates.moves {
        let (from, to) = (mv.1 - 1, mv.2 - 1);
        for _ in 0..mv.0 {
            let c = positions[from].pop().unwrap();
            positions[to].push(c);
        }
    }
    let result: String = positions
        .iter()
        .map(|p| p.last().unwrap().clone())
        .collect();
    Some(result)
}

pub fn part_two(input: &str) -> Option<String> {
    let crates = parse(input);
    let mut positions = crates.positions.clone();
    for mv in crates.moves {
        let (from, to) = (mv.1 - 1, mv.2 - 1);
        let depth = positions[from].len() - mv.0;
        let movers = positions[from].split_off(depth);
        positions[to].extend(movers);
    }
    let result: String = positions
        .iter()
        .map(|p| p.last().unwrap().clone())
        .collect();
    Some(result)
}

fn main() {
    let input = &aoc::read_file("inputs", 5);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 5);
        assert_eq!(part_one(&input), Some("CMZ".to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 5);
        assert_eq!(part_two(&input), Some("MCD".to_string()));
    }
}
