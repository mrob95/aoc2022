use std::u64::MAX;

fn p1(lines: &Vec<&str>, mut i: usize, acc: &mut usize) -> (usize, usize) {
    let mut size = 0;
    let len = lines.len();
    while i < len {
        let parts: Vec<&str> = lines[i].split(" ").collect();
        match parts[0] {
            "$" => match parts[1] {
                "ls" => {}
                "cd" => {
                    if parts[2] == ".." {
                        return (i, size);
                    } else {
                        let (new_i, dsize) = p1(lines, i + 1, acc);
                        i = new_i;
                        if dsize <= 100000 {
                            *acc += dsize;
                        }
                        size += dsize;
                    }
                }
                _ => panic!("Unknown command {}", parts[1]),
            },
            "dir" => {}
            _ => {
                size += parts[0].parse::<usize>().unwrap();
            }
        }
        i += 1;
    }
    return (i, size);
}

fn p2(lines: &Vec<&str>, mut i: usize, best: &mut usize, target: usize) -> (usize, usize) {
    let mut size = 0;
    let len = lines.len();
    while i < len {
        let parts: Vec<&str> = lines[i].split(" ").collect();
        match parts[0] {
            "$" => match parts[1] {
                "ls" => {}
                "cd" => {
                    if parts[2] == ".." {
                        return (i, size);
                    } else {
                        let (new_i, dsize) = p2(lines, i + 1, best, target);
                        i = new_i;
                        if dsize >= target && dsize < *best {
                            *best = dsize;
                        }
                        size += dsize;
                    }
                }
                _ => panic!("Unknown command {}", parts[1]),
            },
            "dir" => {}
            _ => {
                size += parts[0].parse::<usize>().unwrap();
            }
        }
        i += 1;
    }
    return (i, size);
}

pub fn part_one(input: &str) -> Option<usize> {
    let inp = input.trim().split("\n").collect();
    let mut acc = 0;
    p1(&inp, 0, &mut acc);
    Some(acc)
}

pub fn part_two(input: &str) -> Option<usize> {
    let inp: Vec<&str> = input.trim().split("\n").collect();
    let total_size: usize = inp
        .iter()
        .map(|line| {
            let (first, _) = line.split_once(" ").unwrap();
            if let Ok(size) = first.parse::<usize>() {
                size
            } else {
                0
            }
        })
        .sum();
    let target = total_size - 40000000;
    let mut best = MAX as usize;
    p2(&inp, 0, &mut best, target);
    Some(best)
}

fn main() {
    let input = &aoc::read_file("inputs", 7);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }
}
