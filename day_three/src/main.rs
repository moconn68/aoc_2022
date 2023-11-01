use utils::read_file_lines;

use std::collections::HashSet;

fn main() {
    let input: Vec<String> = read_file_lines("input.txt")
        .expect("Unable to read input file")
        .collect();

    println!("Part one: {}", part_one(input.iter()));
    println!("Part two: {}", part_two(&input));
}

fn get_char_priority(c: char) -> i32 {
    if c.is_ascii_lowercase() {
        (c as i32) - 96
    } else if c.is_ascii_uppercase() {
        (c as i32) - 38
    } else {
        panic!("invalid char!")
    }
}

fn part_one<'a>(input: impl Iterator<Item = &'a String>) -> i32 {
    input
        .map(|line| {
            let (left, right) = line.split_at(line.len() / 2);

            let left_set: HashSet<char> = left.chars().collect();
            let mut right_iter = right.chars();
            let common_char: char = loop {
                let rc = right_iter.next().unwrap();
                if left_set.contains(&rc) {
                    break rc;
                }
            };
            get_char_priority(common_char)
        })
        .sum()
}

fn part_two(input: &[String]) -> i32 {
    input
        .chunks(3)
        .map(|group| {
            let set_a: HashSet<char> = group.get(0).unwrap().chars().collect();
            let set_b: HashSet<char> = group.get(1).unwrap().chars().collect();
            let mut v = None;
            for c in group.last().unwrap().chars() {
                if set_a.contains(&c) && set_b.contains(&c) {
                    v = Some(c);
                }
            }
            get_char_priority(v.unwrap())
        })
        .sum()
}

#[cfg(test)]
mod tests {

    use super::*;

    const BASIC_INPUT_STR: &str = r"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn part_one_basic() {
        let input: Vec<String> = utils::read_to_lines(BASIC_INPUT_STR.as_bytes()).collect();

        let expected = 157;
        let actual = part_one(input.iter());
        assert_eq!(expected, actual);
    }

    #[test]
    fn part_two_basic() {
        let input: Vec<String> = utils::read_to_lines(BASIC_INPUT_STR.as_bytes()).collect();

        let expected = 70;
        let actual = part_two(&input);
        assert_eq!(expected, actual);
    }
}
