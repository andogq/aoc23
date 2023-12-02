advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .flat_map(|line| {
                let line = line.chars();

                let first = line
                    .clone()
                    .find(|c| c.is_numeric())
                    .and_then(|first| first.to_digit(10));
                let last = line
                    .rev()
                    .find(|c| c.is_numeric())
                    .and_then(|last| last.to_digit(10));

                if let (Some(first), Some(last)) = (first, last) {
                    Some((first, last))
                } else {
                    None
                }
            })
            .map(|(tens, ones)| (tens * 10) + ones)
            .sum(),
    )
}

const DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

pub fn part_two(input: &str) -> Option<u32> {
    let digit_values = DIGITS
        .into_iter()
        .enumerate()
        .map(|(value, digit)| (value as u32 + 1, digit.to_string()))
        .collect::<Vec<_>>();
    let digit_rev_values = DIGITS
        .into_iter()
        .enumerate()
        .map(|(value, digit)| (value as u32 + 1, digit.chars().rev().collect::<String>()))
        .collect::<Vec<_>>();

    Some(
        input
            .lines()
            .flat_map(|line| {
                let line_rev = line.chars().rev().collect::<String>();

                let mut iter = [
                    (line.to_string(), &digit_values),
                    (line_rev, &digit_rev_values),
                ]
                .into_iter()
                .flat_map(|(line, digit_values)| {
                    let digit = line
                        .chars()
                        .enumerate()
                        .find(|(_, c)| c.is_numeric())
                        .and_then(|(index, c)| c.to_digit(10).map(|d| (index, d)));

                    let word = digit_values
                        .iter()
                        .flat_map(|(value, d)| {
                            line[..digit.map(|(index, _)| index).unwrap_or(line.len())]
                                .find(d)
                                .map(|index| (index, *value))
                        })
                        .min_by_key(|(index, _)| *index);

                    match (word, digit) {
                        (Some((word_index, word_value)), Some((digit_index, digit_value))) => {
                            Some(if word_index < digit_index {
                                word_value
                            } else {
                                digit_value
                            })
                        }
                        (Some((_, value)), None) | (None, Some((_, value))) => Some(value),
                        (None, None) => None,
                    }
                })
                .take(2);

                Some(iter.next()? * 10 + iter.next()?)
            })
            .sum(),
    )
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
