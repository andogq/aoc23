use std::collections::HashMap;

advent_of_code::solution!(14);

fn cycle(map: &mut Vec<Vec<Option<bool>>>, dx: isize, dy: isize) {
    for mut y in 0..map.len() {
        if dy == 1 {
            y = map.len() - y - 1;
        }

        for mut x in 0..map[y].len() {
            if dx == 1 {
                x = map[y].len() - x - 1;
            }

            if matches!(map[y][x], Some(true)) {
                let mut new_y = y;
                let mut new_x = x;

                while (dx == 0
                    && new_y
                        .checked_add_signed(dy)
                        .map(|y| y < map.len())
                        .unwrap_or_default()
                    && ((new_y == 0 && dy == -1)
                        || (new_y == map.len() - 1 && dy == 1)
                        || map[new_y.checked_add_signed(dy).unwrap()][new_x].is_none()))
                    || (dy == 0
                        && new_x
                            .checked_add_signed(dx)
                            .map(|x| x < map[new_y].len())
                            .unwrap_or_default()
                        && ((new_x == 0 && dx == -1)
                            || (new_x == map[new_y].len() - 1 && dx == 1)
                            || map[new_y][new_x.checked_add_signed(dx).unwrap()].is_none()))
                {
                    new_y = new_y.checked_add_signed(dy).unwrap();
                    new_x = new_x.checked_add_signed(dx).unwrap();
                }

                map[y][x] = None;
                map[new_y][new_x] = Some(true);
            }
        }
    }
}

fn solve(input: &str, rotations: &[(isize, isize)], cycles: u32) -> u32 {
    let mut map = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    'O' => Some(true),
                    '#' => Some(false),
                    '.' => None,
                    _ => unreachable!("invalid character in input"),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut visited = HashMap::new();

    let mut i = 0;
    while i < cycles {
        // Each direction north, west, south, east
        rotations.iter().cloned().for_each(|(dx, dy)| {
            cycle(&mut map, dx, dy);
        });

        if let Some(previous_index) = visited.get(&map) {
            let cycle_length = i - previous_index;

            // Determine how many more cycles remaining
            i = cycles - ((cycles - i - 1) % cycle_length);
        } else {
            visited.insert(map.clone(), i);
            i += 1;
        }
    }

    map.into_iter()
        .rev()
        .enumerate()
        .map(|(weight, row)| {
            row.into_iter().filter(|c| matches!(c, Some(true))).count() * (weight + 1)
        })
        .sum::<usize>() as u32
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(solve(input, &[(0, -1)], 1))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(solve(
        input,
        &[(0, -1), (-1, 0), (0, 1), (1, 0)],
        1_000_000_000,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }
}
