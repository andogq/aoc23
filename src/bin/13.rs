advent_of_code::solution!(13);

fn find_reflection(
    col_max: usize,
    row_max: usize,
    variance: usize,
    get: impl Fn(usize, usize) -> bool,
) -> Option<usize> {
    (1..row_max).find(|&row| {
        let distance = if row <= row_max / 2 {
            row
        } else {
            row_max - row
        };

        (0..distance)
            .into_iter()
            .map(|d| {
                (0..col_max)
                    .filter(|&col| get(col, row + d) != get(col, row - d - 1))
                    .count()
            })
            .sum::<usize>()
            == variance
    })
}

fn solve(input: &str, variance: usize) -> u32 {
    input
        .split("\n\n")
        .map(|section| {
            section
                .lines()
                .map(|line| {
                    line.chars()
                        .map(|c| match c {
                            '#' => true,
                            '.' => false,
                            _ => unreachable!("invalid character in input"),
                        })
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .map(|section| {
            if let Some(x) = find_reflection(section.len(), section[0].len(), variance, |y, x| {
                section[y][x]
            }) {
                return x as u32;
            }

            if let Some(y) = find_reflection(section[0].len(), section.len(), variance, |x, y| {
                section[y][x]
            }) {
                return y as u32 * 100;
            }

            unreachable!("reflection expected in every input")
        })
        .sum()
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(solve(input, 0))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(solve(input, 1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(400));
    }
}
