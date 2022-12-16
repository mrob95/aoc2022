use std::collections::{HashMap, HashSet};

use regex::Regex;

struct Map {
    rates: HashMap<[u8; 2], usize>,
    tunnels: HashMap<[u8; 2], Vec<[u8; 2]>>,
    paths: HashMap<[u8; 2], Vec<([u8; 2], u8)>>,
    num_valves: u8,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct State1 {
    on: [[bool; 26]; 26],
    time: usize,
    pos: [u8; 2],
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct State2 {
    on: [[bool; 26]; 26],
    num_on: u8,
    my_pos: [u8; 2],
    my_time: u8,
    el_pos: [u8; 2],
    el_time: u8,
}

fn min_value(distances: &HashMap<[u8; 2], u8>) -> ([u8; 2], u8) {
    let (position, curr_tentative) = distances.iter().min_by_key(|entry| entry.1).unwrap();
    (position.clone(), curr_tentative.clone())
}

fn shortest_path(
    tunnels: &HashMap<[u8; 2], Vec<[u8; 2]>>,
    start: [u8; 2],
    end: [u8; 2],
) -> Option<u8> {
    let mut visited: HashSet<[u8; 2]> = HashSet::new();
    let mut distances = HashMap::<[u8; 2], u8>::new();
    distances.insert(start, 0);
    loop {
        if distances.len() == 0 {
            return None;
        }
        let (curr_pos, curr_tentative) = min_value(&distances);
        if curr_pos == end {
            return Some(*distances.get(&curr_pos).unwrap());
        }
        let next_distance = curr_tentative + 1;
        for step in tunnels.get(&curr_pos).unwrap() {
            let step_cloned = step.clone();
            if visited.contains(step) {
                continue;
            }
            if let Some(tentative) = distances.get_mut(&step_cloned) {
                if *tentative > next_distance {
                    *tentative = next_distance;
                }
            } else {
                distances.insert(step_cloned, next_distance);
            }
        }
        visited.insert(curr_pos);
        distances.remove(&curr_pos);
    }
}

fn parse(input: &str) -> Map {
    let re =
        Regex::new(r"^Valve ([A-Z]+) has flow rate=(\d+); tunnels? leads? to valves? ([A-Z,\s]+)$")
            .unwrap();

    let mut rates = HashMap::new();
    let mut tunnels: HashMap<[u8; 2], Vec<[u8; 2]>> = HashMap::new();
    let mut num_valves = 0;

    for line in input.trim().split("\n") {
        let caps = re.captures(line).unwrap();
        let mut valve_b = caps.get(1).unwrap().as_str().bytes();
        let valve = [
            valve_b.next().unwrap() - b'A',
            valve_b.next().unwrap() - b'A',
        ];
        let rate = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
        let targets = caps
            .get(3)
            .unwrap()
            .as_str()
            .split(", ")
            .map(|s| {
                let mut b = s.bytes();
                [b.next().unwrap() - b'A', b.next().unwrap() - b'A']
            })
            .collect();
        rates.insert(valve.clone(), rate);
        tunnels.insert(valve, targets);
        if rate != 0 {
            num_valves += 1;
        }
    }
    for tunnel in tunnels.values_mut() {
        tunnel.sort_by_key(|v| rates.get(v).unwrap());
        tunnel.reverse();
    }

    // Use Dijskstra to cut the graph down to only distances between non-zero valves
    let mut paths = HashMap::new();
    for start in tunnels
        .keys()
        .filter(|s| rates.get(*s).unwrap() > &0)
        .cloned()
    {
        let mut distances = vec![];
        for end in tunnels
            .keys()
            .filter(|s| rates.get(*s).unwrap() > &0)
            .cloned()
        {
            if start == end {
                continue;
            }
            let distance = shortest_path(&tunnels, start, end).unwrap();
            distances.push((end, distance))
        }
        paths.insert(start, distances);
    }
    // Add distance from start to every valve
    let start = [0, 0];
    let mut distances = vec![];
    for end in tunnels
        .keys()
        .filter(|s| rates.get(*s).unwrap() > &0)
        .cloned()
    {
        let distance = shortest_path(&tunnels, start, end).unwrap();
        distances.push((end, distance))
    }
    paths.insert(start, distances);

    Map {
        rates,
        tunnels,
        paths,
        num_valves,
    }
}

fn best1(map: &Map, state: State1, cache: &mut HashMap<State1, usize>) -> usize {
    if let Some(cached) = cache.get(&state) {
        return *cached;
    }
    let mut result = 0;
    let flow = map.rates.get(&state.pos).unwrap();
    let tunnels = map.tunnels.get(&state.pos).unwrap();
    let flow_total = flow * (30 - (state.time + 1));
    let (a, b) = (state.pos[0], state.pos[1]);
    for tunnel in tunnels {
        if *flow != 0 && !state.on[a as usize][b as usize] && (state.time + 2) < 30 {
            // activate valve then move
            let mut new_state = state.clone();
            new_state.on[a as usize][b as usize] = true;
            new_state.time = state.time + 2;
            new_state.pos = tunnel.to_owned();
            let best = flow_total + best1(map, new_state, cache);
            if best > result {
                result = best;
            }
        }
        if (state.time + 1) < 30 {
            // just move
            let mut new_state = state.clone();
            new_state.time = state.time + 1;
            new_state.pos = tunnel.to_owned();
            let best = best1(map, new_state, cache);
            if best > result {
                result = best;
            }
        }
    }
    cache.insert(state, result);
    return result;
}

pub fn part_one(input: &str) -> Option<usize> {
    let map = parse(input);
    let mut cache = HashMap::<State1, usize>::new();
    let on = [[false; 26]; 26];
    let state = State1 {
        on: on,
        time: 0,
        pos: [0, 0],
    };
    let result = best1(&map, state, &mut cache);
    Some(result)
}

fn best2(map: &Map, state: State2) -> usize {
    if state.num_on == map.num_valves {
        // All on already
        return 0;
    }
    if state.my_time >= 25 && state.el_time >= 25 {
        return 0;
    }
    let mut result = 0;

    if state.my_time <= state.el_time {
        // I move
        let my_tunnels = map.paths.get(&state.my_pos).unwrap();
        for (my_next, my_distance) in my_tunnels {
            if state.my_time + my_distance >= 25 {
                continue;
            }
            if state.on[my_next[0] as usize][my_next[1] as usize] {
                continue;
            }

            let my_flow = map.rates.get(my_next).unwrap();
            let my_flow_total =
                my_flow * (26 - (state.my_time as usize + *my_distance as usize + 1));
            let mut new_state = state.clone();
            new_state.on[my_next[0] as usize][my_next[1] as usize] = true;
            new_state.my_time += my_distance + 1;
            new_state.my_pos = my_next.clone();
            new_state.num_on += 1;
            let best = my_flow_total + best2(map, new_state);
            result = result.max(best);
        }
    } else {
        // elephant move
        let el_tunnels = map.paths.get(&state.el_pos).unwrap();
        for (el_next, el_distance) in el_tunnels {
            if state.el_time + el_distance >= 25 {
                continue;
            }
            if state.on[el_next[0] as usize][el_next[1] as usize] {
                continue;
            }
            let el_flow = map.rates.get(el_next).unwrap();
            let el_flow_total =
                el_flow * (26 - (state.el_time as usize + *el_distance as usize + 1));
            let mut new_state = state.clone();
            new_state.on[el_next[0] as usize][el_next[1] as usize] = true;
            new_state.el_time += el_distance + 1;
            new_state.el_pos = el_next.clone();
            new_state.num_on += 1;
            let best = el_flow_total + best2(map, new_state);
            result = result.max(best);
        }
    }
    return result;
}

pub fn part_two(input: &str) -> Option<usize> {
    let map = parse(input);
    let state = State2 {
        on: [[false; 26]; 26],
        num_on: 0,
        my_pos: [0, 0],
        my_time: 0,
        el_pos: [0, 0],
        el_time: 0,
    };
    let result = best2(&map, state);
    Some(result)
}

fn main() {
    let input = &aoc::read_file("inputs", 16);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 16);
        assert_eq!(part_one(&input), Some(1651));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 16);
        assert_eq!(part_two(&input), Some(1707));
    }
}
