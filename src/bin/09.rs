use std::collections::HashSet;

fn parse(input: &str) -> Vec<(char, i32)> {
    input
        .trim()
        .split("\n")
        .map(|s| {
            let (direction, distance) = s.split_once(" ").unwrap();
            (direction.parse().unwrap(), distance.parse().unwrap())
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<usize> {
    let movements = parse(input);
    let mut visited = HashSet::<[i32; 2]>::new();
    let mut head: [i32; 2] = [0, 0];
    let mut tail: [i32; 2] = [0, 0];
    visited.insert(tail);
    for m in movements {
        match m.0 {
            'U' => head[0] += 1,
            'D' => head[0] -= 1,
            'L' => head[1] -= 1,
            'R' => head[1] += 1,
            _ => panic!("Unrecognised direction {}", m.0),
        }
        while (head[0] - tail[0]).abs() > 1 || (head[1] - tail[1]).abs() > 1 {
            if head[0] == tail[0] || head[1] == tail[1] {
                if (head[0] - tail[0]) > 1 {
                    tail[0] += 1;
                } else if (head[0] - tail[0]) < -1 {
                    tail[0] -= 1;
                }
                if (head[1] - tail[1]) > 1 {
                    tail[1] += 1;
                } else if (head[1] - tail[1]) < -1 {
                    tail[1] -= 1;
                }
            } else {
                if (head[0] - tail[0]) >= 1 {
                    tail[0] += 1;
                } else if (head[0] - tail[0]) <= -1 {
                    tail[0] -= 1;
                }
                if (head[1] - tail[1]) >= 1 {
                    tail[1] += 1;
                } else if (head[1] - tail[1]) <= -1 {
                    tail[1] -= 1;
                }
            }
            visited.insert(tail);
        }
    }
    Some(visited.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let movements = parse(input);
    let mut visited = HashSet::<[i32; 2]>::new();
    let mut positions: [i32; 20] = [0; 20];
    visited.insert(positions[18..].try_into().unwrap());
    for m in movements {
        for _ in 0..m.1 {
            match m.0 {
                'U' => positions[0] += 1,
                'D' => positions[0] -= 1,
                'L' => positions[1] -= 1,
                'R' => positions[1] += 1,
                _ => panic!("Unrecognised direction {}", m.0),
            }
            for i in (2..20).step_by(2) {
                let (head, tail) = positions[i - 2..i + 2].split_at_mut(2);
                while (head[0] - tail[0]).abs() > 1 || (head[1] - tail[1]).abs() > 1 {
                    if head[0] == tail[0] || head[1] == tail[1] {
                        if (head[0] - tail[0]) > 1 {
                            tail[0] += 1;
                        } else if (head[0] - tail[0]) < -1 {
                            tail[0] -= 1;
                        }
                        if (head[1] - tail[1]) > 1 {
                            tail[1] += 1;
                        } else if (head[1] - tail[1]) < -1 {
                            tail[1] -= 1;
                        }
                    } else {
                        if (head[0] - tail[0]) >= 1 {
                            tail[0] += 1;
                        } else if (head[0] - tail[0]) <= -1 {
                            tail[0] -= 1;
                        }
                        if (head[1] - tail[1]) >= 1 {
                            tail[1] += 1;
                        } else if (head[1] - tail[1]) <= -1 {
                            tail[1] -= 1;
                        }
                    }
                    if i == 18 {
                        visited.insert([tail[0], tail[1]]);
                    }
                }
            }
        }
    }
    Some(visited.len())
}

fn main() {
    let input = &aoc::read_file("inputs", 9);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 9);
        assert_eq!(part_two(&input), Some(1));
    }
}
