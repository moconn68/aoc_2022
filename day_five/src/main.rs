use utils::read_file_lines;

use std::collections::VecDeque;

type CrateGrid = Vec<VecDeque<char>>;

/// Convenience struct for parsing "crates" from the input.
#[derive(Clone, Copy)]
struct Crate([u8; 3]);

impl Crate {
    /// Parses a byte array into a [`Crate`].
    ///
    /// Parameters:
    /// * bytes: byte array reference.
    ///
    /// Returns a parsed [`Crate`] if valid, or [`None`] if the provided byte array
    /// does not match the format of '[x]` where x is some byte.
    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        if bytes.len() != 3 || bytes[0] != b'[' || bytes[2] != b']' {
            None
        } else {
            Some(Self([bytes[0], bytes[1], bytes[2]]))
        }
    }

    /// Gets the character associated with the curent [`Crate`]'s byte value.
    pub fn get_char(&self) -> char {
        self.0[1] as char
    }
}

/// Holds the numeric values from a given command.
/// A command refers to a line which is formatted as "move x from y to z" where
/// * x = quantity of crates to move
/// * y = source stack number
/// * z = destination stack number
#[derive(Clone, Copy, Debug, PartialEq)]
struct Command {
    quantity: i32,
    source: i32,
    destination: i32,
}

impl Command {
    fn new(quantity: i32, source: i32, destination: i32) -> Self {
        Self {
            quantity,
            source,
            destination,
        }
    }

    /// Parses a [`Command`] from a given line of input text.
    fn from_line(line: &str) -> Self {
        let nums: &[i32] = &line
            .split(' ')
            // We don't care about the words, just the numbers...
            .filter_map(|x| x.parse::<i32>().ok())
            // ...which are always in the same order
            .collect::<Vec<i32>>()[0..=2];

        Self::new(nums[0], nums[1], nums[2])
    }
}

fn main() {
    let input_iter = read_file_lines("input.txt").expect("Unable to read input file");
    let (mut grid_one, commands) = parse_input(input_iter);
    // Clone original grid state for use in part two
    let mut grid_two = grid_one.clone();

    for command in &commands {
        process_move_one(command, &mut grid_one);
        process_move_two(command, &mut grid_two);
    }

    println!("Part one: {}", get_tops(&grid_one));
    println!("Part two: {}", get_tops(&grid_two));
}

/// Parses lines of input into the requisite data needed to solve the question.
///
/// Parameters:
/// * input: iterable over [`String`]s for the lines of input
///
/// Returns ([`CrateGrid`], [`Vec<Command>`]) for use in solving the question.
fn parse_input(input: impl Iterator<Item = impl Into<String>>) -> (CrateGrid, Vec<Command>) {
    let mut crates_grid: CrateGrid = Default::default();
    let mut commands = vec![];

    for line in input.map(Into::into) {
        if let Some(first_char) = line.chars().next() {
            // Empty line separates initial state from command list
            if line.is_empty() {
                continue;
            }

            match first_char {
                // Line that represents a row of crates (including column numbers, ignored later)
                '[' | ' ' => fill_crates(&line, &mut crates_grid),
                // Line that represents a move: "move x from y to z"
                'm' => commands.push(Command::from_line(&line)),
                // All lines will start with one of the above characters
                _ => (),
            }
        }
    }

    (crates_grid, commands)
}

/// Parses a line of input and populates the [`CrateGrid`] with values from the given row.
///
/// Parameters:
/// * `line` - line of input text representing the given row of crates
/// * `grid` - mutable reference to grid of crates ([`CrateGrid`]) to populate with parsed row data
fn fill_crates(line: &str, grid: &mut CrateGrid) {
    let chars = line.as_bytes();
    for (col, idx) in (0..chars.len() - 2).step_by(4).enumerate() {
        if grid.get(col).is_none() {
            grid.push(Default::default());
        }

        if let Some(c) = Crate::from_bytes(&chars[idx..idx + 3]) {
            grid[col].push_front(c.get_char());
        }
    }
}

/// Parses a line of input text for a given move as described per part one.
///
/// Parameters:
/// * `cmd` - reference to the [`Command`] to be processed.
/// * `grid` - mutable reference to [`CrateGrid`] of crates to manipulate based on the given move
fn process_move_one(cmd: &Command, grid: &mut CrateGrid) {
    for _ in 0..cmd.quantity {
        if let Some(c) = grid[cmd.source as usize - 1].pop_back() {
            grid[cmd.destination as usize - 1].push_back(c);
        }
    }
}

/// Parses a line of input text for a given move as described per part two.
///
/// Parameters:
/// * `cmd` - reference to the [`Command`] to be processed.
/// * `grid` - mutable reference to [`CrateGrid`] of crates to manipulate based on the given move
fn process_move_two(cmd: &Command, grid: &mut CrateGrid) {
    let mut tmp = VecDeque::default();
    for _ in 0..cmd.quantity {
        if let Some(c) = grid[cmd.source as usize - 1].pop_back() {
            tmp.push_front(c);
        }
    }

    for c in tmp {
        grid[cmd.destination as usize - 1].push_back(c);
    }
}

/// Gets the characters topmost crates from each column, concatenated into a [`String`].
fn get_tops(grid: &CrateGrid) -> String {
    grid.iter().filter_map(|col| col.back()).collect::<String>()
}

#[cfg(test)]
mod tests {
    use crate::parse_input;

    use super::*;

    const BASIC_GRID_STR: &str = r"
    [D]
[N] [C]
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn parse_crate() {
        let expected_char = 'A';
        let crate_str = format!("[{}]", expected_char);
        let crate_some = Crate::from_bytes(crate_str.as_bytes());

        assert!(crate_some.is_some());
        assert_eq!(expected_char, crate_some.unwrap().get_char(),);

        let bad_bytes = [0, 1, 2];
        let crate_none = Crate::from_bytes(&bad_bytes);
        assert!(crate_none.is_none());
    }

    #[test]
    fn parse_command() {
        let (q, s, d) = (1, 2, 3);
        let line = format!("move {} from {} to {}", q, s, d);

        let command = Command::from_line(line.as_str());

        assert_eq!(q, command.quantity);
        assert_eq!(s, command.source);
        assert_eq!(d, command.destination);
    }

    #[test]
    fn parse_input_basic() {
        let input = BASIC_GRID_STR.split('\n').map(|line| line.to_owned());

        let expected_grid = Vec::from([
            VecDeque::from(['Z', 'N']),
            VecDeque::from(['M', 'C', 'D']),
            VecDeque::from(['P']),
        ]);
        let expected_commands = Vec::from([
            Command::new(1, 2, 1),
            Command::new(3, 1, 3),
            Command::new(2, 2, 1),
            Command::new(1, 1, 2),
        ]);
        let (actual_grid, actual_commands) = parse_input(input);

        assert_eq!(expected_grid, actual_grid);
        assert_eq!(expected_commands, actual_commands);
    }

    #[test]
    fn part_one_basic() {
        let input = utils::read_to_lines(BASIC_GRID_STR.as_bytes());
        let (mut grid, commands) = parse_input(input);
        let expected = "CMZ";
        for command in commands {
            process_move_one(&command, &mut grid);
        }
        let actual = get_tops(&grid);
        assert_eq!(expected, actual);
    }

    #[test]
    fn part_two_basic() {
        let input = utils::read_to_lines(BASIC_GRID_STR.as_bytes());
        let (mut grid, commands) = parse_input(input);
        let expected = "MCD";
        for command in commands {
            process_move_two(&command, &mut grid);
        }
        let actual = get_tops(&grid);
        assert_eq!(expected, actual);
    }
}
