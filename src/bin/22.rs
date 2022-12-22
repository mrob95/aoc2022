#[derive(Debug, Clone)]
struct Point(isize, isize);

#[derive(Debug)]
struct Map {
    grid: Vec<Vec<char>>,
    max_width: isize,
}

impl Map {
    fn at(self: &Self, pt: &Point) -> char {
        if pt.1 >= self.grid.len() as isize || pt.1 < 0 {
            ' '
        } else if pt.0 >= self.grid[pt.1 as usize].len() as isize || pt.0 < 0 {
            ' '
        } else {
            self.grid[pt.1 as usize][pt.0 as usize]
        }
    }
}

#[derive(Debug)]
struct State {
    position: Point,
    direction: isize,
}

impl State {
    fn new(map: &Map) -> State {
        let mut pt = Point(0, 0);
        while map.at(&pt) != '.' {
            pt.0 += 1;
        }
        State {
            position: pt,
            direction: 0,
        }
    }

    fn move_one_p1(self: &mut Self, map: &Map) -> bool {
        let mut new = self.position.clone();
        let (xdiff, ydiff) = match self.direction {
            0 => (1, 0),
            1 => (0, 1),
            2 => (-1, 0),
            3 => (0, -1),
            _ => panic!("Unknown direction {}", self.direction),
        };
        new.0 = (new.0 + xdiff).rem_euclid(map.max_width);
        new.1 = (new.1 + ydiff).rem_euclid(map.grid.len() as isize);
        while map.at(&new) == ' ' {
            new.0 = (new.0 + xdiff).rem_euclid(map.max_width);
            new.1 = (new.1 + ydiff).rem_euclid(map.grid.len() as isize);
        }
        if map.at(&new) == '#' {
            return false;
        } else {
            self.position = new;
            return true;
        }
    }

    fn move_one_p2(self: &mut Self, map: &Map) -> bool {
        let mut new = self.position.clone();
        let (xdiff, ydiff) = match self.direction {
            0 => (1, 0),
            1 => (0, 1),
            2 => (-1, 0),
            3 => (0, -1),
            _ => panic!("Unknown direction {}", self.direction),
        };
        new.0 = new.0 + xdiff;
        new.1 = new.1 + ydiff;
        if map.at(&new) == ' ' {
            if cfg!(test) {
                let x_along = new.0.rem_euclid(4);
                let y_down = new.1.rem_euclid(4);
                // |  x
                // |xxx
                // |  xx
                let (new_direction, new_x, new_y) = match (self.direction, new.0, new.1) {
                    // right
                    (0, _, 0..=3) => (2, 15, 8 + (3 - new.1)),
                    (0, _, 4..=7) => (1, 12 + 3 - y_down, 8),
                    (0, _, 8..=11) => (2, 11, 3 - y_down),
                    // Down
                    (1, 0..=3, _) => (3, 8 + 3 - x_along, 11),
                    (1, 4..=7, _) => (0, 8, 8 + 3 - x_along),
                    (1, 8..=11, _) => (3, 0 + 3 - x_along, 7),
                    (1, 12..=15, _) => (0, 0, 4 + 3 - x_along),
                    // left
                    (2, _, 0..=3) => (1, 4 + y_down, 4),
                    (2, _, 4..=7) => (3, 12 + 3 - y_down, 11),
                    (2, _, 8..=11) => (3, 4 + 3 - y_down, 7),
                    // Up
                    (3, 0..=3, _) => (1, 8 + 3 - x_along, 0),
                    (3, 4..=7, _) => (0, 8, 0 + x_along),
                    (3, 8..=11, _) => (3, 3 - x_along, 7),
                    (3, 12..=15, _) => (0, 0, 4 + 3 - x_along),
                    _ => panic!("unreachable?"),
                };
                new.0 = new_x;
                new.1 = new_y;
                if map.at(&new) == '#' {
                    return false;
                } else {
                    self.direction = new_direction;
                    self.position.0 = new_x;
                    self.position.1 = new_y;
                    return true;
                }
            } else {
                let x_along = new.0.rem_euclid(50);
                let y_down = new.1.rem_euclid(50);
                // | xx
                // | x
                // |xx
                // |x
                let (new_direction, new_x, new_y) = match (self.direction, new.0, new.1) {
                    // right
                    (0, _, 0..=49) => (2, 99, 149 - y_down),
                    (0, _, 50..=99) => (3, 100 + y_down, 49),
                    (0, _, 100..=149) => (2, 149, 49-y_down),
                    (0, _, 150..=199) => (3, 50+y_down, 149),
                    // Down
                    (1, 0..=49, _) => (1, 100+x_along, 0),
                    (1, 50..=99, _) => (2, 49, 150+x_along),
                    (1, 100..=149, _) => (2, 99, 50+x_along),
                    // left
                    (2, _, 0..=49) => (0, 0, 149-y_down),
                    (2, _, 50..=99) => (1, 0+y_down, 100),
                    (2, _, 100..=149) => (0, 50, 49-y_down),
                    (2, _, 150..=199) => (1, 50+y_down, 0),
                    // Up
                    (3, 0..=49, _) => (0, 50, 50+x_along),
                    (3, 50..=99, _) => (0, 0, 150+ x_along),
                    (3, 100..=149, _) => (3, 0+x_along, 199),
                    _ => panic!("unreachable?"),
                };
                if new_x == 29 && new_y == 199 {
                    println!("here");
                }
                new.0 = new_x;
                new.1 = new_y;
                if map.at(&new) == '#' {
                    return false;
                } else {
                    self.direction = new_direction;
                    self.position.0 = new_x;
                    self.position.1 = new_y;
                    return true;
                }
            }
        }
        if map.at(&new) == '#' {
            return false;
        } else {
            self.position = new;
            return true;
        }
    }

    fn result(self: &Self) -> isize {
        let row = self.position.1 + 1;
        let col = self.position.0 + 1;
        1000 * row + 4 * col + self.direction
    }
}

#[derive(Debug)]
enum Command {
    Move(isize),
    Turn(isize),
}

fn parse(input: &str) -> (Map, Vec<Command>) {
    let (raw_map, raw_commands) = input.split_once("\n\n").unwrap();
    let grid: Vec<Vec<char>> = raw_map.split("\n").map(|s| s.chars().collect()).collect();
    let mut digit = String::new();
    let mut commands = Vec::new();
    for c in raw_commands.trim().chars() {
        if c.is_alphabetic() {
            commands.push(Command::Move(digit.parse().unwrap()));
            digit.clear();
            match c {
                'L' => commands.push(Command::Turn(-1)),
                'R' => commands.push(Command::Turn(1)),
                _ => panic!("Unrecognised direction {}", c),
            };
        } else {
            digit.push(c);
        }
    }
    if digit.len() > 0 {
        commands.push(Command::Move(digit.parse().unwrap()));
    }
    let max_width = grid.iter().map(|l| l.len()).max().unwrap() as isize;
    (Map { grid, max_width }, commands)
}

pub fn part_one(input: &str) -> Option<isize> {
    let (map, commands) = parse(input);
    let mut state = State::new(&map);
    for command in commands {
        match command {
            Command::Turn(val) => state.direction = (state.direction + val).rem_euclid(4),
            Command::Move(val) => {
                for _ in 0..val {
                    if !state.move_one_p1(&map) {
                        break;
                    }
                }
            }
        }
    }
    Some(state.result())
}

pub fn part_two(input: &str) -> Option<isize> {
    let (map, commands) = parse(input);
    let mut state = State::new(&map);
    for command in commands {
        match command {
            Command::Turn(val) => state.direction = (state.direction + val).rem_euclid(4),
            Command::Move(val) => {
                for _ in 0..val {
                    if !state.move_one_p2(&map) {
                        break;
                    }
                }
            }
        }
    }
    Some(state.result())
}

fn main() {
    let input = &aoc::read_file("inputs", 22);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 22);
        assert_eq!(part_one(&input), Some(6032));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 22);
        assert_eq!(part_two(&input), Some(5031));
    }
}
