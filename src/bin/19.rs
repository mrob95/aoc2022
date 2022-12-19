use std::collections::HashMap;

use regex::Regex;

#[derive(Debug)]
struct Blueprint {
    ore: u16,           // ore
    clay: u16,          // ore
    obsidian: [u16; 2], // ore, clay
    geode: [u16; 2],    // ore, obsidian
}

fn parse(input: &str) -> Vec<Blueprint> {
    let re = Regex::new(r"\d+").unwrap();

    input
        .trim()
        .split("\n")
        .map(|s| {
            let mut numbers = re.find_iter(s).skip(1);
            let ore = numbers.next().unwrap().as_str().parse().unwrap();
            let clay = numbers.next().unwrap().as_str().parse().unwrap();
            let ob_ore = numbers.next().unwrap().as_str().parse().unwrap();
            let ob_clay = numbers.next().unwrap().as_str().parse().unwrap();
            let ge_ore = numbers.next().unwrap().as_str().parse().unwrap();
            let ge_ob = numbers.next().unwrap().as_str().parse().unwrap();
            Blueprint {
                ore,
                clay,
                obsidian: [ob_ore, ob_clay],
                geode: [ge_ore, ge_ob],
            }
        })
        .collect()
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct State {
    robots: [u16; 4],    // ore, clay, obsidian, geode
    resources: [u16; 4], // ore, clay, obsidian, geode
    time: u8,
}

fn next_to_build(robots: &[u16; 4], blueprint: &Blueprint) -> Vec<u8> {
    let mut result = Vec::new();
    if robots[2] > 0 {
        result.push(3);
    }
    if robots[1] > 0 && robots[2] < blueprint.geode[1] {
        result.push(2);
    }
    if robots[1] < blueprint.obsidian[1] {
        result.push(1);
    }
    if robots[0] < blueprint.ore
        || robots[0] < blueprint.clay
        || robots[0] < blueprint.obsidian[0]
        || robots[0] < blueprint.geode[0]
    {
        result.push(0);
    }
    return result;
}

fn can_build(blueprint: &Blueprint, resources: &[u16; 4], rock: u8) -> bool {
    match rock {
        0 => resources[0] >= blueprint.ore,
        1 => resources[0] >= blueprint.clay,
        2 => resources[0] >= blueprint.obsidian[0] && resources[1] >= blueprint.obsidian[1],
        3 => resources[0] >= blueprint.geode[0] && resources[2] >= blueprint.geode[1],
        _ => panic!("Unrecognised rock"),
    }
}
impl State {
    fn next_states(self: &Self, blueprint: &Blueprint, finish: u8) -> Vec<State> {
        let mut result = Vec::new();
        let mut did_wait_state = false;

        for next in next_to_build(&self.robots, blueprint) {
            let mut time = self.time;
            let mut new_resources = self.resources.clone();
            let mut new_robots = self.robots.clone();

            while !can_build(blueprint, &new_resources, next) && time < finish {
                time += 1;
                for i in 0..4 {
                    new_resources[i] += self.robots[i];
                }
            }

            if time >= finish {
                if !did_wait_state {
                    result.push(State {
                        robots: new_robots,
                        resources: new_resources,
                        time,
                    });
                    did_wait_state = true;
                }
                continue;
            }

            time += 1;
            for i in 0..4 {
                new_resources[i] += self.robots[i];
            }

            if next == 0 {
                // Create ore robot
                new_resources[0] -= blueprint.ore;
                new_robots[0] += 1;
            } else if next == 1 {
                // Create clay robot
                new_resources[0] -= blueprint.clay;
                new_robots[1] += 1;
            } else if next == 2 {
                // Create obsidian robot
                new_resources[0] -= blueprint.obsidian[0];
                new_resources[1] -= blueprint.obsidian[1];
                new_robots[2] += 1;
            } else if next == 3 {
                // Create geodeo robot
                new_resources[0] -= blueprint.geode[0];
                new_resources[2] -= blueprint.geode[1];
                new_robots[3] += 1;
            }

            let state = State {
                robots: new_robots,
                resources: new_resources,
                time,
            };
            result.push(state);
        }

        return result;
    }
}

fn best(
    blueprint: &Blueprint,
    state: State,
    cache: &mut HashMap<State, u16>,
    finish: u8,
) -> u16 {
    if state.time >= finish || (state.time == (finish - 1) && state.robots[3] == 0) {
        return state.resources[3];
    }
    if let Some(result) = cache.get(&state) {
        return *result;
    }
    let mut result = 0;
    for next in state.next_states(blueprint, finish) {
        result = result.max(best(blueprint, next, cache, finish));
    }
    cache.insert(state, result);
    return result;
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut result = 0;
    let blueprints = parse(input);
    for (i, blueprint) in blueprints.iter().enumerate() {
        let state = State {
            robots: [1, 0, 0, 0],
            resources: [0, 0, 0, 0],
            time: 0,
        };
        let mut cache = HashMap::new();
        let blueprint_result = best(blueprint, state, &mut cache, 24);
        result += (i + 1) * blueprint_result as usize;
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut result = 1;
    let blueprints = parse(input);
    for blueprint in blueprints.iter().take(3) {
        let state = State {
            robots: [1, 0, 0, 0],
            resources: [0, 0, 0, 0],
            time: 0,
        };
        let mut cache = HashMap::new();
        let blueprint_result = best(blueprint, state, &mut cache, 32);
        result *= blueprint_result as usize;
    }
    Some(result)
}

fn main() {
    let input = &aoc::read_file("inputs", 19);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 19);
        assert_eq!(part_one(&input), Some(33));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 19);
        assert_eq!(part_two(&input), Some(62 * 56));
    }
}
