use std::collections::HashMap;

fn parse(input: &str) -> Vec<(i64, i64)> {
    let numbers = input.trim().split("\n").map(|s| s.trim().parse::<i64>().unwrap());
    let mut result: Vec<(i64, i64)> = vec![];
    let mut counts = HashMap::new();
    for number in numbers {
        if let Some(count) = counts.get_mut(&number) {
            *count += 1;
            result.push((number, *count));
        } else {
            counts.insert(number, 0);
            result.push((number, 0));
        }
    }
    return result;
}

fn find_index_of(nums: &Vec<(i64, i64)>, a: (i64, i64)) -> usize {
    let mut i = 0;
    while nums[i] != a {
        i += 1;
    }
    return i;
}

fn mix(order: &Vec<(i64, i64)>, numbers: &mut Vec<(i64, i64)>) {
    for num in order {
        let mut position = find_index_of(numbers, *num);
        // If the length of the list is 7, it takes 6 moves
        // in either direction to get back to where we started,
        // so we only need to worry about the moves after that
        // has happened n times.
        let num_moves = num.0 % (numbers.len() as i64 - 1);
        for _ in 0..num_moves.abs() {
            let to: usize;
            if num_moves < 0 {
                to = (position as i64 - 1).rem_euclid(numbers.len() as i64) as usize;
            } else {
                to = (position + 1) % numbers.len();
            }
            numbers.swap(position, to);
            position = to;
        }
    }
}

fn get_result(numbers: &Vec<(i64, i64)>) -> i64 {
    let i = find_index_of(&numbers, (0, 0));
    [1000, 2000, 3000].iter().map(|val| numbers[(i + val) % numbers.len()].0).sum()
}

pub fn part_one(input: &str) -> Option<i64> {
    let order = parse(input);
    let mut numbers = order.clone();
    mix(&order, &mut numbers);
    Some(get_result(&numbers))
}

pub fn part_two(input: &str) -> Option<i64> {
    let mut order = parse(input);
    order.iter_mut().for_each(|n| n.0 *= 811589153);
    let mut numbers = order.clone();

    for _ in 0..10 {
        mix(&order, &mut numbers);
    }
    Some(get_result(&numbers))
}

fn main() {
    let input = &aoc::read_file("inputs", 20);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 20);
        assert_eq!(part_one(&input), Some(3));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 20);
        assert_eq!(part_two(&input), Some(1623178306));
    }
}
