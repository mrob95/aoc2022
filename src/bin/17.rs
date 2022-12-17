//  0123456
// |..@@@@.| 4
// |.......| y = 3
// |.......| y = 2
// |.......| y = 1
// +-------+

// to get starting position, x+=2, y+=ymax+4
// #### - [0, 0], [1, 0], [2, 0], [3, 0]

// .#.
// ###
// .#. - [0, 1], [1, 2], [1, 1], [1, 0], [2, 1]

// ..#
// ..#
// ### - [0, 0], [1, 0], [2, 0], [2, 1], [2, 2]

// #
// #
// #
// # - [0, 0], [0, 1], [0, 2], [0, 3]

// ##
// ## - [0, 0], [0, 1], [1, 0], [1, 1]

// Move right: if any(x=6) pass else x+=1
// Move left:  if any(x=0) pass else x-=1

// Move down:
// if any(y-1) are occupied or any(y=0) rest, else y-=1
// rest:
// Add positions to map, bump ymax to highest + 1

use std::collections::HashSet;

#[derive(Debug)]
struct State {
    map: HashSet<[i32; 2]>,
    ymax: i32,
    rocks: u32,
}

fn parse(input: &str) -> Vec<u8> {
    input.trim().bytes().collect()
}

fn try_jet(state: &State, shape: &mut Vec<[i32; 2]>, jet: u8) {
    let xdiff = match jet {
        b'<' => -1,
        b'>' => 1,
        _ => panic!("Unrecognised jet {}", jet),
    };
    let blocked = shape.iter().any(|pos|  {
        let new_pos = [pos[0] + xdiff, pos[1]];
        new_pos[0] < 0 || new_pos[0] > 6 || state.map.contains(&new_pos)
    });
    if blocked {
        return;
    }
    for pos in shape {
        pos[0] += xdiff;
    }
}

fn move_rock_down(state: &mut State, shape: &mut Vec<[i32; 2]>) -> bool {
    let can_move = shape.iter().all(|pos| {
        let new_pos = [pos[0], pos[1] - 1];
        new_pos[1] > 0 && !state.map.contains(&new_pos)
    });
    if can_move {
        for pos in shape.iter_mut() {
            pos[1] -= 1;
        }
    } else {
        let mut new_ymax = 0;
        for pos in shape.iter() {
            state.map.insert(pos.clone());
            new_ymax = new_ymax.max(pos[1]);
        }
        state.ymax = state.ymax.max(new_ymax);
        state.rocks += 1;
        return true;
    }
    return false;
}

fn detect_repetition(heights: &Vec<i32>) -> Option<usize> {
    for i in 1..10000 {
        // Originally I was looking for cycles with lengths that were a multiple of the product of
        // the number of shapes and the number of jets, to guarantee that the shape/jet state
        // was the same at the beginning of each cycle. That took too long though, so
        // I tried 10 and that also worked... not sure why
        let rocks_per_cycle = i * 10;
        if heights.len() <= rocks_per_cycle * 2 {
            continue;
        }
        let last = heights.len() - 1;
        let target_difference = heights[last] - heights[last - rocks_per_cycle];
        let cycle_detected = (1..rocks_per_cycle + 1).all(|pos| {
            let test_difference = heights[last - pos] - heights[last - pos - rocks_per_cycle];
            test_difference == target_difference
        });
        if cycle_detected {
            return Some(rocks_per_cycle);
        }
    }
    None
}

fn height_after_cycles(n: u64, all_jets: &Vec<u8>) -> u64 {
    let mut jets = all_jets.iter().cycle();
    let mut state = State {
        map: HashSet::new(),
        ymax: 0,
        rocks: 0,
    };
    let all_shapes = vec![
        vec![[0, 0], [1, 0], [2, 0], [3, 0]],
        vec![[0, 1], [1, 2], [1, 1], [1, 0], [2, 1]],
        vec![[0, 0], [1, 0], [2, 0], [2, 1], [2, 2]],
        vec![[0, 0], [0, 1], [0, 2], [0, 3]],
        vec![[0, 0], [0, 1], [1, 0], [1, 1]],
    ];
    let mut heights = vec![];
    let mut shapes = all_shapes.iter().cycle();
    while (state.rocks as u64) <= n {
        let mut shape = shapes.next().unwrap().clone();
        for pos in shape.iter_mut() {
            pos[0] += 2;
            pos[1] += state.ymax + 4;
        }
        loop {
            let jet = jets.next().unwrap().clone();
            try_jet(&state, &mut shape, jet);
            if move_rock_down(&mut state, &mut shape) {
                break;
            }
        }

        heights.push(state.ymax);
        if let Some(rocks_per_cycle) = detect_repetition(&heights) {
            let mut height = heights.last().unwrap().clone() as u64;
            let cycle_start = heights.len() - 1 - rocks_per_cycle;
            let difference_per_cycle = height - heights[cycle_start] as u64;
            let rocks_to_go = n - heights.len() as u64;
            let num_cycles = rocks_to_go / rocks_per_cycle as u64;
            let remainder = rocks_to_go % rocks_per_cycle as u64;
            let remainder_height = heights[cycle_start + remainder as usize] - heights[cycle_start];
            height += num_cycles * difference_per_cycle;
            height += remainder_height as u64;
            return height;
        }
    }
    return state.ymax as u64;
}

pub fn part_one(input: &str) -> Option<u64> {
    let all_jets = parse(input);
    let result = height_after_cycles(2022, &all_jets);
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let all_jets = parse(input);
    let result = height_after_cycles(1000000000000, &all_jets);
    Some(result)
}

fn main() {
    let input = &aoc::read_file("inputs", 17);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 17);
        assert_eq!(part_one(&input), Some(3068));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 17);
        assert_eq!(part_two(&input), Some(1514285714288));
    }
}
