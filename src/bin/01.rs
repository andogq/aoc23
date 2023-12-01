advent_of_code::solution!(1);

fn find_calibration_value<S: AsRef<str>>(lines: impl IntoIterator<Item = S>) -> u32 {
    lines
        .into_iter()
        .flat_map(|line| {
            let line = line.as_ref().chars();

            let first = line
                .clone()
                .find(|c| c.is_numeric())
                .and_then(|first| first.to_digit(10));
            let last = line
                .rev()
                .find(|c| c.is_numeric())
                .and_then(|first| first.to_digit(10));

            if let (Some(first), Some(last)) = (first, last) {
                Some((first, last))
            } else {
                None
            }
        })
        .map(|(tens, ones)| (tens * 10) + ones)
        .sum()
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(find_calibration_value(input.lines()))
}

const DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

pub fn part_two(input: &str) -> Option<u32> {
    fn match_line(line: &str) -> String {
        let mut matches = DIGITS
            .into_iter()
            .enumerate()
            .flat_map(|(value, digit)| {
                (0..line.len())
                    .filter(move |&i| line[i..].starts_with(digit))
                    .map(move |index| (index, (value + 1).to_string()))
            })
            .collect::<Vec<_>>();

        matches.sort_unstable_by_key(|(index, _)| index.clone());

        matches
            .into_iter()
            .map(|(_, digit)| digit)
            .collect::<String>()
    }

    Some(find_calibration_value(input.lines().map(|line| {
        let chars = line.chars();

        let (line, current_word) = chars.clone().fold(
            ("".to_string(), "".to_string()),
            |(mut line, mut current_word), c| {
                if c.is_numeric() {
                    line += &match_line(&current_word);
                    line.push(c);
                    current_word = "".to_string();
                } else {
                    current_word.push(c);
                }

                (line, current_word)
            },
        );

        line + &match_line(&current_word)
    })))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(281));
    }
}
