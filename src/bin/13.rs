use std::{cmp::Ordering, str};

#[derive(Debug, Clone, PartialEq)]
enum Item {
    Integer(u8),
    List(Vec<Item>),
}

fn parse_list(cs: &Vec<u8>, pos: &mut usize) -> Item {
    let mut result: Vec<Item> = vec![];
    if cs[*pos] != b'[' {
        panic!("Trying to parse non-list starting at {}", *pos);
    }
    *pos += 1;
    while cs[*pos] != b']' {
        match cs[*pos] {
            b'[' => {
                let item = parse_list(cs, pos);
                result.push(item);
                *pos += 1;
            }
            b',' => *pos += 1,
            _ => {
                let start = *pos;
                while cs[*pos].is_ascii_digit() {
                    *pos += 1;
                }
                let int_slice = &cs[start..*pos];
                let int_str = str::from_utf8(int_slice).unwrap();
                let item = int_str.parse::<u8>().unwrap();
                result.push(Item::Integer(item));
            }
        }
    }
    Item::List(result)
}

fn parse(input: &str) -> Vec<(Item, Item)> {
    input
        .trim()
        .split("\n\n")
        .map(|pair| {
            let (left, right) = pair.split_once("\n").unwrap();
            let left_bytes = left.bytes().collect();
            let right_bytes = right.bytes().collect();
            let mut left_pos = 0;
            let mut right_pos = 0;
            (
                parse_list(&left_bytes, &mut left_pos),
                parse_list(&right_bytes, &mut right_pos),
            )
        })
        .collect()
}

fn compare(left: &Item, right: &Item) -> Ordering {
    match (left, right) {
        (Item::Integer(i), Item::List(v)) => {
            let new_list = Item::List(vec![Item::Integer(*i)]);
            return compare(&new_list, right);
        }
        (Item::List(v), Item::Integer(i)) => {
            let new_list = Item::List(vec![Item::Integer(*i)]);
            return compare(left, &new_list);
        }
        (Item::Integer(l), Item::Integer(r)) => {
            return l.cmp(&r);
        }
        (Item::List(l), Item::List(r)) => {
            for i in 0..l.len() {
                if i >= r.len() {
                    return Ordering::Greater;
                }
                let result = compare(&l[i], &r[i]);
                match result {
                    Ordering::Equal => {}
                    _ => return result,
                }
            }
            if l.len() < r.len() {
                return Ordering::Less;
            }
            return Ordering::Equal;
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let pairs = parse(input);
    let mut result = 0;
    let mut index = 1;
    for (left, right) in pairs {
        if compare(&left, &right) == Ordering::Less {
            result += index;
        }
        index += 1;
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let pairs = parse(input);
    let mut packets = vec![];
    for (left, right) in pairs {
        packets.push(left);
        packets.push(right);
    }
    let divider_1 = Item::List(vec![Item::List(vec![Item::Integer(2)])]);
    let divider_2 = Item::List(vec![Item::List(vec![Item::Integer(6)])]);
    packets.push(divider_1.clone());
    packets.push(divider_2.clone());
    packets.sort_by(compare);
    let mut result = 1;
    for (i, packet) in packets.iter().enumerate() {
        if *packet == divider_1 || *packet == divider_2 {
            result *= i + 1;
        }
    }
    Some(result as u32)
}

fn main() {
    let input = &aoc::read_file("inputs", 13);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 13);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 13);
        assert_eq!(part_two(&input), Some(140));
    }
}
