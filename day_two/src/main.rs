use utils::read_file_lines;

#[derive(Clone, Copy)]
enum Choice {
    Rock = 1,
    Paper,
    Scissors,
}

impl TryFrom<char> for Choice {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' | 'X' => Ok(Self::Rock),
            'B' | 'Y' => Ok(Self::Paper),
            'C' | 'Z' => Ok(Self::Scissors),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Copy)]
enum Outcome {
    Lose = 0,
    Draw = 3,
    Win = 6,
}

impl TryFrom<char> for Outcome {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'X' => Ok(Self::Lose),
            'Y' => Ok(Self::Draw),
            'Z' => Ok(Self::Win),
            _ => Err(()),
        }
    }
}

impl Outcome {
    fn get_outcome(theirs: &Choice, mine: &Choice) -> Self {
        match theirs {
            Choice::Rock => match mine {
                Choice::Rock => Self::Draw,
                Choice::Paper => Self::Win,
                Choice::Scissors => Self::Lose,
            },
            Choice::Paper => match mine {
                Choice::Rock => Self::Lose,
                Choice::Paper => Self::Draw,
                Choice::Scissors => Self::Win,
            },
            Choice::Scissors => match mine {
                Choice::Rock => Self::Win,
                Choice::Paper => Self::Lose,
                Choice::Scissors => Self::Draw,
            },
        }
    }
    fn determine_my_choice(&self, theirs: &Choice) -> Choice {
        match self {
            Outcome::Lose => match theirs {
                Choice::Rock => Choice::Scissors,
                Choice::Paper => Choice::Rock,
                Choice::Scissors => Choice::Paper,
            },
            Outcome::Draw => *theirs,
            Outcome::Win => match theirs {
                Choice::Rock => Choice::Paper,
                Choice::Paper => Choice::Scissors,
                Choice::Scissors => Choice::Rock,
            },
        }
    }
    fn get_score(&self, player_choice: &Choice) -> i32 {
        *self as i32 + *player_choice as i32
    }
}

fn main() {
    let choices = parse_input(read_file_lines("input.txt").expect("Unable to read input file"));
    println!("Part One: {}", part_one(choices.iter()));
    println!("Part Two: {}", part_two(choices.iter()));
}

fn parse_input(input: impl Iterator<Item = impl Into<String>>) -> Vec<(char, char)> {
    let mut choices = vec![];
    for line in input.map(Into::<String>::into) {
        let mut iter = line.chars().step_by(2);
        let first = iter.next().expect("should have a value for first");
        let second = iter.next().expect("should have a value for second");

        choices.push((first, second));
    }
    choices
}

fn part_one<'a>(choices: impl Iterator<Item = &'a (char, char)>) -> i32 {
    choices
        .map(|(first, second)| {
            let theirs = Choice::try_from(*first).expect("should be valid conversion");
            let mine = Choice::try_from(*second).expect("should be valid conversion");
            let outcome = Outcome::get_outcome(&theirs, &mine);
            outcome.get_score(&mine)
        })
        .sum()
}

fn part_two<'a>(choices: impl Iterator<Item = &'a (char, char)>) -> i32 {
    choices
        .map(|(first, second)| {
            let theirs = Choice::try_from(*first).expect("should be valid conversion");
            let outcome = Outcome::try_from(*second).expect("should be valid conversion");
            let mine = outcome.determine_my_choice(&theirs);
            outcome.get_score(&mine)
        })
        .sum()
}

#[cfg(test)]
mod tests {

    use super::*;

    const BASIC_INPUT_STR: &str = r"A Y
B X
C Z";

    #[test]
    fn part_one_basic() {
        let input = utils::read_to_lines(BASIC_INPUT_STR.as_bytes());
        let data = parse_input(input);

        let expected = 15;
        let actual = part_one(data.iter());
        assert_eq!(expected, actual);
    }

    #[test]
    fn part_two_basic() {
        let input = utils::read_to_lines(BASIC_INPUT_STR.as_bytes());
        let data = parse_input(input);

        let expected = 12;
        let actual = part_two(data.iter());
        assert_eq!(expected, actual);
    }
}
