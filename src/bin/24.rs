use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: isize,
    y: isize,
}

#[derive(Debug)]
struct Grid {
    map: HashMap<Point, Vec<u8>>, // position, directions
    height: isize,
    width: isize,
}

impl Grid {
    fn next_grid(self: &Self) -> Self {
        let mut new = HashMap::new();
        for (pt, dirs) in self.map.iter() {
            for dir in dirs {
                let next = match *dir {
                    0 => Point {
                        x: pt.x,
                        y: (pt.y - 1).rem_euclid(self.height),
                    },
                    1 => Point {
                        x: (pt.x + 1).rem_euclid(self.width),
                        y: pt.y,
                    },
                    2 => Point {
                        x: pt.x,
                        y: (pt.y + 1).rem_euclid(self.height),
                    },
                    3 => Point {
                        x: (pt.x - 1).rem_euclid(self.width),
                        y: pt.y,
                    },
                    _ => unreachable!(),
                };
                new.entry(next).or_insert(vec![]).push(*dir);
            }
        }
        Grid {
            map: new,
            height: self.height,
            width: self.width,
        }
    }

    fn next_moves(self: &Self, pt: Point, target: &Point) -> Vec<Point> {
        let mut result = vec![];
        for next in [
            Point {
                x: pt.x + 1,
                y: pt.y,
            },
            Point {
                x: pt.x,
                y: pt.y + 1,
            },
            Point {
                x: pt.x,
                y: pt.y - 1,
            },
            Point { x: pt.x, y: pt.y },
            Point {
                x: pt.x - 1,
                y: pt.y,
            },
        ] {
            if next == *target {
                return vec![target.clone()];
            }
            if (next.x == 0 && next.y == -1) || (next.x == self.width - 1 && next.y == self.height)
            {
                result.push(next);
                continue;
            }
            if next.x < 0
                || next.x >= self.width
                || next.y < 0
                || next.y >= self.height
                || self.map.contains_key(&next)
            {
                continue;
            }
            result.push(next);
        }
        return result;
    }
}

fn parse(input: &str) -> Grid {
    let lines: Vec<Vec<u8>> = input
        .trim()
        .split("\n")
        .map(|s| s.bytes().collect())
        .collect();
    let mut map = HashMap::new();
    for (y, line) in lines.iter().enumerate() {
        for (x, b) in line.iter().enumerate() {
            if *b == b'#' || *b == b'.' {
                continue;
            }
            let pt = Point {
                x: x as isize - 1,
                y: y as isize - 1,
            };
            let direction = match *b {
                b'^' => 0,
                b'>' => 1,
                b'v' => 2,
                b'<' => 3,
                _ => unreachable!(),
            };
            map.insert(pt, vec![direction]);
        }
    }
    Grid {
        map,
        height: lines.len() as isize - 2,
        width: lines[0].len() as isize - 2,
    }
}

fn shortest_path(
    grids: &mut Vec<Grid>,
    time: usize,
    position: Point,
    target: &Point,
    best: &mut usize,
    cache: &mut HashMap<(Point, usize), usize>,
) -> usize {
    let mut result = 999;
    if time >= *best {
        return result;
    }
    let new_time = time + 1;
    if new_time == grids.len() {
        let next = grids.last().unwrap().next_grid();
        grids.push(next);
    }
    let grid = &grids[new_time];
    for mv in grid.next_moves(position, target) {
        if mv == *target {
            *best = (*best).min(new_time);
            return new_time;
        }
        if let Some(cached) = cache.get(&(mv, new_time)) {
            *best = (*best).min(*cached);
            continue;
        }
        let shortest = shortest_path(grids, new_time, mv, target, best, cache);
        result = result.min(shortest);
    }
    cache.insert((position, time), result);
    return result;
}

pub fn part_one(input: &str) -> Option<usize> {
    let grid = parse(input);
    let width = grid.width;
    let height = grid.height;
    let mut grids = vec![grid];
    let start = Point { x: 0, y: -1 };
    let target = Point {
        x: width - 1,
        y: height,
    };
    let mut cache = HashMap::new();
    let result = shortest_path(&mut grids, 0, start, &target, &mut 999, &mut cache);
    Some(result)
}

pub fn part_two(input: &str) -> Option<usize> {
    let grid = parse(input);
    let width = grid.width;
    let height = grid.height;
    let mut grids = vec![grid];
    let start = Point { x: 0, y: -1 };
    let end = Point {
        x: width - 1,
        y: height,
    };
    let mut cache = HashMap::new();

    let first = shortest_path(&mut grids, 0, start, &end, &mut 999, &mut cache);
    cache.clear();
    let second = shortest_path(&mut grids, first, end, &start, &mut 999, &mut cache);
    cache.clear();
    let third = shortest_path(&mut grids, second, start, &end, &mut 999, &mut cache);

    Some(third)
}

fn main() {
    let input = &aoc::read_file("inputs", 24);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 24);
        assert_eq!(part_one(&input), Some(18));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 24);
        assert_eq!(part_two(&input), Some(54));
    }
}
