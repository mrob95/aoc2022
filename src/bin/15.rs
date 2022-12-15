use std::collections::HashMap;

use regex::Regex;

#[derive(Debug)]
enum Item {
    Beacon,
    NotBeacon,
}

fn manhattan_distance(a: &[isize; 2], b: &[isize; 2]) -> isize {
    (a[0] - b[0]).abs() + (a[1] - b[1]).abs()
}

fn find_x_extents(items: &Vec<([isize; 2], [isize; 2])>) -> (isize, isize) {
    let mut xmin = 0;
    let mut xmax = 0;
    for (sensor, beacon) in items {
        let distance = manhattan_distance(&sensor, &beacon);
        if (sensor[0] - distance) < xmin {
            xmin = sensor[0] - distance;
        } else if (sensor[0] + distance) > xmax {
            xmax = sensor[0] + distance;
        }
    }
    (xmin, xmax)
}

fn parse(input: &str) -> Vec<([isize; 2], [isize; 2])> {
    let re =
        Regex::new(r"^Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)$")
            .unwrap();

    input
        .trim()
        .split("\n")
        .map(|s| {
            let caps = re.captures(s).unwrap();
            let xs = caps.get(1).unwrap().as_str();
            let ys = caps.get(2).unwrap().as_str();
            let xb = caps.get(3).unwrap().as_str();
            let yb = caps.get(4).unwrap().as_str();
            (
                [xs.parse().unwrap(), ys.parse().unwrap()],
                [xb.parse().unwrap(), yb.parse().unwrap()],
            )
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    // This is a mess. Given time I would rework this using part 2's solution
    let items = parse(input);
    let y = if cfg!(test) { 10 } else { 2000000 };
    let mut map = HashMap::<[isize; 2], Item>::new();
    let (xmin, xmax) = find_x_extents(&items);
    for (sensor, beacon) in items {
        let distance = manhattan_distance(&sensor, &beacon);
        let ydist = (sensor[1] - y).abs();
        if ydist <= distance {
            let xdist = distance - ydist;
            for x in (sensor[0] - xdist)..(sensor[0] + xdist + 1) {
                map.insert([x, y], Item::NotBeacon);
            }
        }
        if beacon[1] == y {
            map.insert([beacon[0], y], Item::Beacon);
        }
    }
    let mut result = 0;
    for x in xmin..xmax + 1 {
        if let Some(Item::NotBeacon) = map.get(&[x, y]) {
            result += 1;
        }
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut items = parse(input);
    items.sort_by_key(|(sensor, _)| sensor[0]);
    let slice = items.as_slice();
    let ymax = if cfg!(test) { 20 } else { 4000000 };
    for y in 0..ymax + 1 {
        // For each y coordinate, go through the sensors in horizontal order,
        // bumping x to the rightmost extent of the current sensor's coverage.
        // If we go through all of the sensors without reaching the edge of
        // the area, we have found the hole.
        let mut x = 0;
        for (sensor, beacon) in slice {
            let distance = manhattan_distance(&sensor, &beacon);
            let ydist = (sensor[1] - y).abs();
            if ydist <= distance {
                let xdist = distance - ydist;
                // x is within range of this sensor
                if x >= (sensor[0] - xdist) && x <= (sensor[0] + xdist) {
                    x = sensor[0] + xdist + 1;
                }
            }
            if x > ymax {
                // out of bounds
                break;
            }
        }
        if x <= ymax {
            return Some((x as u64) * 4000000 + (y as u64));
        }
    }
    None
}

fn main() {
    let input = &aoc::read_file("inputs", 15);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 15);
        assert_eq!(part_one(&input), Some(26));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 15);
        assert_eq!(part_two(&input), Some(56000011));
    }
}
