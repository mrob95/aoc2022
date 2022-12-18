use std::collections::HashSet;

fn parse(input: &str) -> HashSet<[i8; 3]> {
    input
        .trim()
        .split("\n")
        .map(|s| {
            let points = s
                .split(",")
                .map(|p| p.parse().unwrap())
                .collect::<Vec<i8>>();
            [points[0], points[1], points[2]]
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let cubes = parse(input);
    let mut result = 0;
    for cube in &cubes {
        for i in 0..3 {
            for delta in [-1, 1] {
                let mut adjacent = cube.clone();
                adjacent[i] += delta;
                if !cubes.contains(&adjacent) {
                    result += 1;
                }
            }
        }
    }
    Some(result)
}

fn can_reach_edge(
    cubes: &HashSet<[i8; 3]>,
    outside: &mut HashSet<[i8; 3]>,
    pockets: &mut HashSet<[i8; 3]>,
    point: [i8; 3],
    min: i8,
    max: i8,
) -> bool {
    if pockets.contains(&point) {
        return false;
    }
    if outside.contains(&point) {
        return true;
    }
    let mut visited: HashSet<[i8; 3]> = HashSet::new();
    visited.insert(point);
    loop {
        let mut point_added = false;
        for pos in &visited.clone() {
            for i in 0..3 {
                for delta in [-1, 1] {
                    let mut adjacent = pos.clone();
                    adjacent[i] += delta;
                    if adjacent[i] <= min || adjacent[i] >= max {
                        // Reached an edge
                        outside.extend(&visited);
                        return true;
                    }
                    if visited.contains(&adjacent) || cubes.contains(&adjacent) {
                        continue;
                    } else {
                        point_added = true;
                        visited.insert(adjacent);
                    }
                }
            }
        }
        if !point_added {
            // In a pocket
            pockets.extend(visited);
            return false;
        }
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let cubes = parse(input);
    let mut outside = HashSet::new();
    let mut pockets = HashSet::new();
    let mut result = 0;
    let (min, max) = (-1, 23);
    for cube in &cubes {
        for i in 0..3 {
            for delta in [-1, 1] {
                let mut adjacent = cube.clone();
                adjacent[i] += delta;
                if !cubes.contains(&adjacent)
                    && can_reach_edge(&cubes, &mut outside, &mut pockets, adjacent, min, max)
                {
                    result += 1;
                }
            }
        }
    }
    Some(result)
}

fn main() {
    let input = &aoc::read_file("inputs", 18);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 18);
        assert_eq!(part_one(&input), Some(64));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 18);
        assert_eq!(part_two(&input), Some(58));
    }
}
