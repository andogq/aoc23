use std::collections::HashMap;

advent_of_code::solution!(14);

pub fn part_one(input: &str) -> Option<u32> {
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

    let mut score = 0;

    // Move rocks north
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            let mut new_y = y;

            if matches!(map[y][x], Some(true)) {
                while new_y > 0 && map[new_y - 1][x].is_none() {
                    new_y -= 1;
                }

                map[y][x] = None;
                map[new_y][x] = Some(true);

                score += map.len() - new_y;
            }
        }
    }

    Some(score as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
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

    let mut skip_cycle_check = false;

    const CYCLES: u32 = 1000000000;
    let mut i = 0;
    while i < CYCLES {
        // Each direction north, west, south, east
        [(0isize, -1isize), (-1, 0), (0, 1), (1, 0)]
            .into_iter()
            .for_each(|(dx, dy)| {
                let y_iter = if dy == 1 {
                    Box::new((0..map.len()).rev().into_iter()) as Box<dyn Iterator<Item = usize>>
                } else {
                    Box::new((0..map.len()).into_iter())
                };

                for y in y_iter {
                    let x_iter = if dx == 1 {
                        Box::new((0..map[0].len()).rev().into_iter())
                            as Box<dyn Iterator<Item = usize>>
                    } else {
                        Box::new((0..map[0].len()).into_iter())
                    };

                    for x in x_iter {
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
                                        || map[new_y][new_x.checked_add_signed(dx).unwrap()]
                                            .is_none()))
                            {
                                new_y = new_y.checked_add_signed(dy).unwrap();
                                new_x = new_x.checked_add_signed(dx).unwrap();
                            }

                            map[y][x] = None;
                            map[new_y][new_x] = Some(true);
                        }
                    }
                }
            });

        // map.iter().for_each(|line| {
        //     line.iter().for_each(|c| {
        //         print!(
        //             "{}",
        //             match c {
        //                 Some(true) => 'O',
        //                 Some(false) => '#',
        //                 None => '.',
        //             }
        //         )
        //     });
        //     println!("");
        // });
        // println!("");

        if let (false, Some(previous_index)) = (skip_cycle_check, visited.get(&map)) {
            let cycle_length = i - previous_index;

            // Determine how many more cycles remaining
            println!("found cycle: {cycle_length}");

            dbg!(cycle_length, i);

            i += 1;
            i = CYCLES - ((CYCLES - i) % cycle_length);

            skip_cycle_check = true;
        } else {
            if !skip_cycle_check {
                visited.insert(map.clone(), i);
            }
            i += 1;
        }
    }

    let mut score = 0;

    // Score
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if matches!(map[y][x], Some(true)) {
                score += map.len() - y;
            }
        }
    }

    Some(score as u32)
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
