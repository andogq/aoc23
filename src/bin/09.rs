advent_of_code::solution!(9);

fn solve(input: &str, operation: fn(sequence: &[i64], next_difference: i64) -> i64) -> i64 {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse().expect("valid number"))
                .collect::<Vec<_>>()
        })
        .map(|sequence| {
            let mut sequences = vec![sequence];

            while !sequences
                .last()
                .expect("sequence present")
                .iter()
                .all(|&n| n == 0)
            {
                sequences.push(
                    sequences
                        .last()
                        .expect("sequence present")
                        .windows(2)
                        .map(|window| window[1] - window[0])
                        .collect(),
                );
            }

            sequences
                .iter()
                .rev()
                .fold(0, |next_value, sequence| operation(&sequence, next_value))
        })
        .sum()
}

pub fn part_one(input: &str) -> Option<i64> {
    Some(solve(input, |sequence, next_difference| {
        sequence.last().expect("last value in sequence") + next_difference
    }))
}

pub fn part_two(input: &str) -> Option<i64> {
    Some(solve(input, |sequence, next_difference| {
        sequence.first().expect("first value in sequence") - next_difference
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
