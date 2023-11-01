use utils::read_file_lines;

const WINDOW_SIZE_ONE: i32 = 4;
const WINDOW_SIZE_TWO: i32 = 14;

fn main() {
    let input: String = read_file_lines("input.txt")
        .expect("Unable to read input file")
        .collect();
    println!("Part one: {}", evaluate_buf(&input, WINDOW_SIZE_ONE));
    println!("Part two: {}", evaluate_buf(&input, WINDOW_SIZE_TWO));
}

fn evaluate_buf(input: &str, window_size: i32) -> i32 {
    (input
        .as_bytes()
        .windows(window_size as usize)
        .enumerate()
        .find(|w| w.1.iter().collect::<std::collections::HashSet<&u8>>().len() == w.1.len())
        .unwrap()
        .0 as i32)
        + window_size
}

#[cfg(test)]
mod tests {
    use super::*;

    const BASIC_INPUT_STR: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";

    #[test]
    fn part_one_basic() {
        let expected = 7;
        let actual = evaluate_buf(BASIC_INPUT_STR, WINDOW_SIZE_ONE);
        assert_eq!(expected, actual);
    }

    #[test]
    fn part_two_basic() {
        let expected = 19;
        let actual = evaluate_buf(BASIC_INPUT_STR, WINDOW_SIZE_TWO);
        assert_eq!(expected, actual);
    }
}
