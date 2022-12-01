fn parse(input: &str) -> Vec<u32> {
    input
        .split("\n\n")
        .map(|group| {
            group
                .trim()
                .split("\n")
                .map(|food| food.trim().parse::<u32>().unwrap())
                .sum()
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let totals = parse(input);
    totals.iter().max().cloned()
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut totals = parse(input);
    totals.sort();
    Some(totals.iter().rev().take(3).sum())
}

fn main() {
    let input = &aoc::read_file("inputs", 1);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}
