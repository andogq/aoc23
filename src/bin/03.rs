advent_of_code::solution!(3);

const ADJACENT: &[(isize, isize)] = &[
    (0, 1),
    (1, 0),
    (0, -1),
    (-1, 0),
    (1, 1),
    (1, -1),
    (-1, 1),
    (-1, -1),
];

pub fn part_one(input: &str) -> Option<u32> {
    let schematic = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    Some(
        schematic
            .iter()
            .enumerate()
            .flat_map(|(y, line)| {
                let mut line = line.iter().enumerate().peekable();

                let mut numbers = Vec::new();
                let mut current_number: Option<(u32, bool)> = None;

                while let Some((x, c)) = line.next() {
                    if c.is_numeric() {
                        let adjacent = ADJACENT
                            .iter()
                            .cloned()
                            .map(|(dx, dy)| {
                                (x.saturating_add_signed(dx), y.saturating_add_signed(dy))
                            })
                            .filter_map(|(x, y)| schematic.get(y).and_then(|line| line.get(x)))
                            .any(|&c| c != '.' && !c.is_numeric());

                        let n = c.to_digit(10).unwrap();

                        current_number = Some({
                            let (current_n, current_adjacent) = current_number.unwrap_or_default();

                            (n + (current_n * 10), adjacent | current_adjacent)
                        });

                        if line.peek().map(|(_, &c)| !c.is_numeric()).unwrap_or(true) {
                            if let Some((n, true)) = current_number {
                                // Only save the number if it's adjacent
                                numbers.push(n);
                            }

                            current_number = None;
                        }
                    }
                }

                numbers
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let schematic = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut gear_numbers = schematic
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            let mut line = line.iter().enumerate().peekable();

            let mut numbers = Vec::new();
            let mut current_number: Option<(u32, Option<(usize, usize)>)> = None;

            while let Some((x, c)) = line.next() {
                if c.is_numeric() {
                    // Check if number is adjacent to any gears
                    let adjacent_gear = ADJACENT
                        .iter()
                        .cloned()
                        .map(|(dx, dy)| (x.saturating_add_signed(dx), y.saturating_add_signed(dy)))
                        .filter_map(|(x, y)| {
                            schematic
                                .get(y)
                                .and_then(|line| line.get(x))
                                .map(|c| (x, y, c))
                        })
                        .find(|(_, _, &c)| c == '*')
                        .map(|(x, y, _)| (x, y));

                    let n = c.to_digit(10).unwrap();

                    current_number = Some({
                        let (current_n, current_adjacent) = current_number.unwrap_or_default();

                        (n + (current_n * 10), current_adjacent.or(adjacent_gear))
                    });

                    if line.peek().map(|(_, &c)| !c.is_numeric()).unwrap_or(true) {
                        if let Some((n, Some(gear))) = current_number {
                            // Only save the number if it's adjacent
                            numbers.push((gear, n));
                        }

                        current_number = None;
                    }
                }
            }

            numbers
        })
        .collect::<Vec<_>>();

    gear_numbers.sort_unstable_by_key(|(gear, _)| gear.clone());

    let mut total = 0;
    let mut iter = gear_numbers.into_iter().peekable();

    while let Some((gear, n)) = iter.next() {
        if iter
            .peek()
            .map(|(other_gear, _)| gear == *other_gear)
            .unwrap_or_default()
        {
            if let Some((_, other_n)) = iter.next() {
                total += n * other_n;
            }
        }
    }

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
