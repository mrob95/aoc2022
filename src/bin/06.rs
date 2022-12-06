fn parse(input: &str) -> Vec<u8> {
    input.trim().bytes().collect()
}

pub fn part_one(input: &str) -> Option<usize> {
    let chars = parse(input);
    for (i, window) in chars.windows(4).enumerate() {
        if !(window[1..].contains(&window[0])
            || window[2..].contains(&window[1])
            || window[3..].contains(&window[2]))
        {
            return Some(i + 4);
        }
    }
    None
}

pub fn part_two(input: &str) -> Option<usize> {
    let chars = parse(input);
    for (i, window) in chars.windows(14).enumerate() {
        if !(0..13).any(|j| window[(j + 1)..].contains(&window[j])) {
            return Some(i + 14);
        }
    }
    None
}

fn main() {
    let input = &aoc::read_file("inputs", 6);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}
