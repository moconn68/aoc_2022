use utils::read_file_lines;

use std::collections::BTreeSet;

fn main() {
    let input = read_file_lines("input.txt").expect("Unable to read input file");
    let elves = parse_input(input);

    println!("Part one: {}", part_one(&elves));
    println!("Part two: {}", part_two(&elves));
}

fn parse_input(input: impl Iterator<Item = String>) -> BTreeSet<i32> {
    let mut elves: BTreeSet<i32> = Default::default();
    let mut cur_elf: Option<i32> = Default::default();

    for line in input {
        if line.is_empty() {
            elves.insert(cur_elf.expect("cur_elf should be Some"));
            cur_elf = Default::default();
        } else {
            let new_val = line.parse::<i32>().unwrap();
            cur_elf = Some(match cur_elf {
                Some(v) => v + new_val,
                None => new_val,
            });
        }
    }
    if let Some(v) = cur_elf {
        elves.insert(v);
    }
    elves
}

fn part_one(elves: &BTreeSet<i32>) -> i32 {
    *elves.last().unwrap()
}
fn part_two(elves: &BTreeSet<i32>) -> i32 {
    elves.iter().rev().take(3).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const BASIC_INPUT_STR: &str = r"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn part_one_basic() {
        let input = utils::read_to_lines(BASIC_INPUT_STR.as_bytes());
        let data = parse_input(input);
        assert_eq!(part_one(&data), 24_000)
    }

    #[test]
    fn part_two_basic() {
        let input = utils::read_to_lines(BASIC_INPUT_STR.as_bytes());
        let data = parse_input(input);
        assert_eq!(part_two(&data), 45_000);
    }
}
