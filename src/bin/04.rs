#[derive(Debug, Copy, Clone)]
struct Range {
    start: u32,
    end: u32,
}

impl Range {
    fn contains(self: Range, other: Range) -> bool {
        return self.start <= other.start && self.end >= other.end;
    }
    fn overlaps(self: Range, other: Range) -> bool {
        return (self.start >= other.start && self.start <= other.end)
            || (self.end >= other.start && self.end <= other.end)
            || (other.start >= self.start && other.start <= self.end)
            || (other.end >= self.start && other.end <= self.end);
    }
}

fn parse(input: &str) -> Vec<(Range, Range)> {
    input
        .trim()
        .split("\n")
        .map(|s| {
            let mut digits = s
                .split(",")
                .map(|r| r.split("-"))
                .flatten()
                .map(|b| b.parse::<u32>().unwrap())
                .into_iter();
            (
                Range{start: digits.next().unwrap(), end: digits.next().unwrap()},
                Range{start: digits.next().unwrap(), end: digits.next().unwrap()},
            )
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let sections = parse(input);
    let mut result = 0;
    for section in sections {
        if section.0.contains(section.1) || section.1.contains(section.0) {
            result += 1;
        }
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let sections = parse(input);
    let mut result = 0;
    for section in sections {
        if section.0.overlaps(section.1) {
            result += 1;
        }
    }
    Some(result)
}

fn main() {
    let input = &aoc::read_file("inputs", 4);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}
