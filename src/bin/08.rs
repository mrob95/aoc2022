fn parse(input: &str) -> Vec<Vec<u8>> {
    input
        .trim()
        .split("\n")
        .map(|s| s.bytes().map(|b| b - '0' as u8).collect())
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let trees = parse(input);
    let width = trees[0].len();
    let height = trees.len();
    let mut num_visible = 0;
    for x in 0..width {
        for y in 0..height {
            let tree_height = trees[y][x];
            if y == 0 || x == 0
            || (0..y).all(|yy| trees[yy][x] < tree_height) // up
            || (y+1..height).all(|yy| trees[yy][x] < tree_height) // down
            || (0..x).all(|xx| trees[y][xx] < tree_height) // left
            || (x+1..width).all(|xx| trees[y][xx] < tree_height) // right
            {
                num_visible += 1;
            }
        }
    }
    Some(num_visible)
}

pub fn part_two(input: &str) -> Option<u32> {
    let trees = parse(input);
    let width = trees[0].len();
    let height = trees.len();
    let mut best = 0;
    for x in 0..width {
        for y in 0..height {
            let mut result = 0;
            let tree_height = trees[y][x];
            let mut num_visible = 0;
            for yy in (0..y).rev() {
                num_visible += 1;
                if trees[yy][x] >= tree_height {
                    break;
                }
            }
            result = num_visible;
            num_visible = 0;
            for yy in y + 1..height {
                num_visible += 1;
                if trees[yy][x] >= tree_height {
                    break;
                }
            }
            result *= num_visible;
            num_visible = 0;
            for xx in (0..x).rev() {
                num_visible += 1;
                if trees[y][xx] >= tree_height {
                    break;
                }
            }
            result *= num_visible;
            num_visible = 0;
            for xx in x + 1..width {
                num_visible += 1;
                if trees[y][xx] >= tree_height {
                    break;
                }
            }
            result *= num_visible;
            if result > best {
                best = result;
            }
        }
    }
    Some(best)
}
fn main() {
    let input = &aoc::read_file("inputs", 8);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}
