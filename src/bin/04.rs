use std::collections::VecDeque;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .flat_map(|line| line.split_once(": "))
            .flat_map(|(_, line)| line.split_once(" | "))
            .flat_map(|(selected, valid)| {
                Some((
                    selected
                        .split_whitespace()
                        .map(|n| n.parse::<u32>().ok())
                        .collect::<Option<Vec<_>>>()?,
                    valid
                        .split_whitespace()
                        .map(|n| n.parse::<u32>().ok())
                        .collect::<Option<Vec<_>>>()?,
                ))
            })
            .map(|(mut selected, mut valid)| {
                selected.sort_unstable();
                valid.sort_unstable();

                let mut selected = selected.into_iter();
                let mut valid = valid.into_iter().peekable();

                let mut winning = Vec::new();

                while let Some(n) = selected.next() {
                    while valid.peek().map(|&next| next < n).unwrap_or_default() {
                        valid.next();
                    }

                    if valid.peek().map(|&next| next == n).unwrap_or_default() {
                        winning.push(n);
                    }
                }

                winning
            })
            .map(|winning| {
                if winning.is_empty() {
                    0
                } else {
                    2u32.pow(winning.len() as u32 - 1)
                }
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .flat_map(|line| line.split_once(": "))
            .flat_map(|(_, line)| line.split_once(" | "))
            .flat_map(|(selected, valid)| {
                Some((
                    selected
                        .split_whitespace()
                        .map(|n| n.parse::<u32>().ok())
                        .collect::<Option<Vec<_>>>()?,
                    valid
                        .split_whitespace()
                        .map(|n| n.parse::<u32>().ok())
                        .collect::<Option<Vec<_>>>()?,
                ))
            })
            .map(|(mut selected, mut valid)| {
                selected.sort_unstable();
                valid.sort_unstable();

                let mut selected = selected.into_iter();
                let mut valid = valid.into_iter().peekable();

                let mut winning = Vec::new();

                while let Some(n) = selected.next() {
                    while valid.peek().map(|&next| next < n).unwrap_or_default() {
                        valid.next();
                    }

                    if valid.peek().map(|&next| next == n).unwrap_or_default() {
                        winning.push(n);
                    }
                }

                winning.len() as u32
            })
            .fold(
                (0u32, VecDeque::<u32>::new()),
                |(mut card_count, mut multipliers), winning| {
                    let multiplier = multipliers.pop_front().unwrap_or(1);
                    card_count += multiplier;

                    while (multipliers.len() as u32) < winning {
                        multipliers.push_back(1);
                    }

                    (0..winning).into_iter().for_each(|i| {
                        multipliers[i as usize] += multiplier;
                    });

                    (card_count, multipliers)
                },
            )
            .0,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
