use std::collections::HashMap;

use regex::Regex;

#[derive(Debug)]
enum Item {
    Sensor,
    Beacon,
    NotBeacon,
}

fn manhattan_distance(a: &[isize; 2], b: &[isize; 2]) -> isize {
    (a[0] - b[0]).abs() + (a[1] - b[1]).abs()
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

pub fn part_one(input: &str) -> Option<u32> {
    let items = parse(input);
    let target_y = 2000000;
    let mut map = HashMap::<[isize; 2], Item>::new();
    let mut xmin = 0;
    let mut xmax = 0;
    for (sensor, beacon) in items {
        let distance = manhattan_distance(&sensor, &beacon);
        if (sensor[0] - distance) < xmin {
            xmin = sensor[0] - distance;
        } else if (sensor[0] + distance) > xmax {
            xmax = sensor[0] + distance;
        }
        let ydist = (sensor[1] - target_y).abs();
        if ydist <= distance {
            let xdist = distance - ydist;
            for x in (sensor[0] - xdist)..(sensor[0] + xdist + 1) {
                map.insert([x, target_y], Item::NotBeacon);
            }
        }
        if beacon[1] == target_y {
            map.insert([beacon[0], target_y], Item::Beacon);
        }
    }
    let mut result = 0;
    for x in xmin..xmax + 1 {
        if let Some(Item::NotBeacon) = map.get(&[x, target_y]) {
            result += 1;
        }
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let readings = parse(input);
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
        assert_eq!(part_two(&input), None);
    }
}

// for (sensor, beacon) in items {
//     let distance = manhattan_distance(&sensor, &beacon);
//     for y in (-distance..distance + 1) {
//         let xdist = distance - y.abs();
//         for x in -xdist..xdist + 1 {
//             let pos = [sensor[0] + x, sensor[1] + y];
//             if !map.contains_key(&pos) {
//                 map.insert(pos, Item::NotBeacon);
//             }
//             if pos[0] < xmin {
//                 xmin = pos[0];
//             } else if pos[0] > xmax {
//                 xmax = pos[0];
//             }
//         }
//     }
// }
