use std::{collections::{HashMap, HashSet}, any};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Point(isize, isize);

fn parse(input: &str) -> HashSet<Point> {
    let mut result = HashSet::new();
    for (y, line) in input.trim().split("\n").enumerate() {
        for (x, b) in line.bytes().enumerate() {
            if b == b'#' {
                result.insert(Point(x as isize, y as isize));
            }
        }
    }
    result
}

fn do_round(points: &mut HashSet<Point>, iteration: i32) -> bool {
    let mut movers = HashMap::<Point, Vec<Point>>::new();
    let mut new_points = HashSet::with_capacity(movers.len());
    let mut any_moved = false;
    // 0 = north, 1 = south, 2 = west, 3 = east
    for pt in points.iter() {
        let (x, y) = (pt.0, pt.1);
        let n = &Point(x, y - 1);
        let ne = &Point(x + 1, y - 1);
        let e = &Point(x + 1, y);
        let se = &Point(x + 1, y + 1);
        let s = &Point(x, y + 1);
        let sw = &Point(x - 1, y + 1);
        let w = &Point(x - 1, y);
        let nw = &Point(x - 1, y - 1);

        if [n, ne, e, se, s, sw, w, nw]
            .iter()
            .all(|p| !points.contains(p))
        {
            new_points.insert(pt.clone());
            continue;
        }

        let mut moved = false;
        for i in 0..4 {
            let direction = (iteration + i).rem_euclid(4);
            let (checks, mv) = match direction {
                0 => ([nw, n, ne], n),
                1 => ([sw, s, se], s),
                2 => ([sw, w, nw], w),
                3 => ([ne, e, se], e),
                _ => unreachable!(),
            };
            if checks.iter().all(|ch| !points.contains(ch)) {
                movers.entry(mv.clone()).or_insert(vec![]).push(pt.clone());
                moved = true;
                break;
            }
        }
        if !moved {
            new_points.insert(pt.clone());
        }
    }
    for (to, froms) in movers.iter() {
        if froms.len() == 1 {
            new_points.insert(to.clone());
            any_moved = true;
        } else {
            for from in froms {
                new_points.insert(from.clone());
            }
        }
    }
    assert_eq!(points.len(), new_points.len());
    *points = new_points;
    return any_moved;
}

pub fn part_one(input: &str) -> Option<isize> {
    let mut points = parse(input);
    for iteration in 0..10 {
        do_round(&mut points, iteration);
    }
    let min_x = points.iter().map(|pt| pt.0).min().unwrap();
    let max_x = points.iter().map(|pt| pt.0).max().unwrap();
    let min_y = points.iter().map(|pt| pt.1).min().unwrap();
    let max_y = points.iter().map(|pt| pt.1).max().unwrap();
    let rect_size = (max_y + 1 - min_y) * (max_x + 1 - min_x);
    Some(rect_size - points.len() as isize)
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut points = parse(input);
    let mut iteration = 0;
    loop {
        let moved = do_round(&mut points, iteration);
        iteration += 1;
        if !moved {
            return Some(iteration)
        }
    }
}

fn main() {
    let input = &aoc::read_file("inputs", 23);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 23);
        assert_eq!(part_one(&input), Some(110));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 23);
        assert_eq!(part_two(&input), Some(20));
    }
}
