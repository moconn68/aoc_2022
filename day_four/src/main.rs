use utils::read_file_lines;

fn main() {
    let input: Vec<String> = read_file_lines("input.txt")
        .expect("Unable to read input file")
        .collect();
    println!("Part one: {}", part_one(input.iter()));
    println!("Part two: {}", part_two(input.iter()));
}

fn part_one<'a>(input: impl Iterator<Item = &'a String>) -> i32 {
    input
        .filter(|line| {
            let groups: Vec<(i32, i32)> = line
                .split(',')
                .map(|g| {
                    let nums: Vec<i32> = g.split('-').map(|c| c.parse::<i32>().unwrap()).collect();
                    (*nums.first().unwrap(), *nums.last().unwrap())
                })
                .collect();

            let (a, b) = (groups.first().unwrap(), groups.last().unwrap());

            (a.0 <= b.0 && a.1 >= b.1) || (b.0 <= a.0 && b.1 >= a.1)
        })
        .count() as i32
}

fn part_two<'a>(input: impl Iterator<Item = &'a String>) -> i32 {
    input
        .filter(|line| {
            let groups: Vec<(i32, i32)> = line
                .split(',')
                .map(|g| {
                    let nums: Vec<i32> = g.split('-').map(|c| c.parse::<i32>().unwrap()).collect();
                    (*nums.first().unwrap(), *nums.last().unwrap())
                })
                .collect();

            let (a, b) = (groups.first().unwrap(), groups.last().unwrap());

            (a.0..=a.1).contains(&b.0)
                || (a.0..=a.1).contains(&b.1)
                || (b.0..=b.1).contains(&a.0)
                || (b.0..=b.1).contains(&a.1)
        })
        .count() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    const BASIC_INPUT_STR: &str = r"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn part_one_basic() {
        let input: Vec<String> = utils::read_to_lines(BASIC_INPUT_STR.as_bytes()).collect();

        let expected = 2;
        let actual = part_one(input.iter());

        assert_eq!(expected, actual);
    }

    #[test]
    fn part_two_basic() {
        let input: Vec<String> = utils::read_to_lines(BASIC_INPUT_STR.as_bytes()).collect();

        let expected = 4;
        let actual = part_two(input.iter());

        assert_eq!(expected, actual);
    }
}
