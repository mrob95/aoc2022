use std::collections::{HashMap, HashSet};

fn potential_steps(heights: &Vec<Vec<u8>>, position: &[usize; 2]) -> Vec<[usize; 2]> {
    let width = heights[0].len();
    let height = heights.len();
    let mut result = vec![];
    let x = position[0];
    let y = position[1];
    let pos_height = heights[y][x];
    if x != 0 && heights[y][x - 1] <= pos_height + 1  {
        result.push([x - 1, y])
    }
    if x < (width - 1) && heights[y][x + 1] <= pos_height + 1  {
        result.push([x + 1, y])
    }
    if y != 0 && heights[y - 1][x] <= pos_height + 1 {
        result.push([x, y - 1])
    }
    if y < (height - 1) && heights[y + 1][x] <= pos_height + 1  {
        result.push([x, y + 1])
    }
    result
}

fn min_value(distances: &HashMap<[usize; 2], u32>) -> ([usize; 2], u32) {
    let (position, curr_tentative) = distances
            .iter()
            .min_by_key(|entry| entry.1)
            .unwrap();
    (position.clone(), curr_tentative.clone())
}

fn shortest_path(heights: &Vec<Vec<u8>>, start: [usize; 2], end: [usize; 2]) -> Option<u32> {
    let mut visited: HashSet<[usize; 2]> = HashSet::new();
    let mut distances = HashMap::<[usize; 2], u32>::new();
    distances.insert(start, 0);
    loop {
        if distances.len() == 0 {
            return None
        }
        let (curr_pos, curr_tentative) = min_value(&distances);
        if curr_pos == end {
            return Some(*distances.get(&curr_pos).unwrap());
        }
        let next_distance = curr_tentative + 1;
        for step in potential_steps(heights, &curr_pos) {
            if visited.contains(&step) {
                continue;
            }
            if let Some(tentative) = distances.get_mut(&step) {
                if *tentative > next_distance {
                    *tentative = next_distance;
                }
            } else {
                distances.insert(step, next_distance);
            }
        }
        visited.insert(curr_pos);
        distances.remove(&curr_pos);
    }
}

fn parse(input: &str) -> (Vec<Vec<u8>>, [usize; 2], [usize; 2]) {
    let mut start_position = [0; 2];
    let mut end_position = [0; 2];
    let heights = input
        .trim()
        .split("\n")
        .enumerate()
        .map(|(y, s)| {
            s.bytes()
                .enumerate()
                .map(|(x, b)| match b {
                    b'S' => {
                        start_position = [x, y];
                        0
                    }
                    b'E' => {
                        end_position = [x, y];
                        25
                    },
                    _ => b - b'a',
                })
                .collect()
        })
        .collect();
    return (heights, start_position, end_position);
}

pub fn part_one(input: &str) -> Option<u32> {
    let (heights, start, end) = parse(input);
    let result = shortest_path(&heights, start, end);
    result
}

pub fn part_two(input: &str) -> Option<u32> {
    let (heights, start, end) = parse(input);
    let mut result = 0;
    for (y, line) in heights.iter().enumerate() {
        for (x, height) in line.iter().enumerate() {
            if *height != 0 {
                continue
            }
            if let Some(path_length) = shortest_path(&heights, [x, y], end) {
                if result == 0 || path_length < result {
                    result = path_length;
                }
            }
        }
    }
    Some(result)
}

fn main() {
    let input = &aoc::read_file("inputs", 12);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 12);
        assert_eq!(part_one(&input), Some(31));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 12);
        assert_eq!(part_two(&input), Some(29));
    }
}
