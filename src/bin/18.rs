use std::collections::HashMap;

advent_of_code::solution!(18);

#[derive(Clone, Copy, Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl From<Direction> for (isize, isize) {
    fn from(direction: Direction) -> Self {
        match direction {
            Direction::Up => (0, -1),
            Direction::Right => (1, 0),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
        }
    }
}

impl TryFrom<char> for Direction {
    type Error = ();

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'U' => Ok(Self::Up),
            'R' => Ok(Self::Right),
            'D' => Ok(Self::Down),
            'L' => Ok(Self::Left),
            _ => Err(()),
        }
    }
}

impl TryFrom<u32> for Direction {
    type Error = ();

    fn try_from(c: u32) -> Result<Self, Self::Error> {
        match c {
            0 => Ok(Self::Right),
            1 => Ok(Self::Down),
            2 => Ok(Self::Left),
            3 => Ok(Self::Up),
            _ => Err(()),
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let corners = input
        .lines()
        .map(|line| {
            let mut iter = line.split_whitespace();

            let direction =
                Direction::try_from(iter.next().expect("direction").chars().next().unwrap())
                    .unwrap();
            let distance = iter
                .next()
                .expect("distance")
                .parse::<u32>()
                .expect("valid distance");

            (direction, distance)
        })
        .scan((0isize, 0isize), |(x, y), (direction, distance)| {
            let (dx, dy) = direction.into();

            let result = Some(((*x, *y), direction, distance));

            *x += dx * distance as isize;
            *y += dy * distance as isize;

            result
        })
        .collect::<Vec<_>>();

    // Find min bounds
    let (min_x, min_y) = corners
        .iter()
        .fold((0, 0), |(min_x, min_y), ((x, y), _, _)| {
            ((*x).min(min_x), (*y).min(min_y))
        });

    // Offset everything to make min (0, 0)
    let corners = corners
        .into_iter()
        .map(|((x, y), direction, distance)| {
            (
                ((x - min_x) as usize, (y - min_y) as usize),
                direction,
                distance,
            )
        })
        .collect::<Vec<_>>();

    // Find max bounds
    let (max_x, max_y) = corners
        .iter()
        .fold((0, 0), |(max_x, max_y), ((x, y), _, _)| {
            ((*x).max(max_x), (*y).max(max_y))
        });

    let mut map = vec![vec![false; max_x + 1]; max_y + 1];

    // Trace out map
    for ((x, y), direction, distance) in &corners {
        for i in 1..=*distance {
            let (dx, dy) = (*direction).into();

            let x = x.checked_add_signed(dx * i as isize).unwrap();
            let y = y.checked_add_signed(dy * i as isize).unwrap();

            map[y][x] = true;
        }
    }

    let mut count = 0;
    let mut previous_row = Vec::new();

    for row in map.iter() {
        let mut inside = false;
        let mut line_entry = None;

        let mut new_row = Vec::new();

        for (x, cell) in row.iter().enumerate() {
            let mut filled = false;

            if *cell {
                // Line
                filled = true;

                let previous_empty = x
                    .checked_add_signed(-1)
                    .and_then(|x| row.get(x))
                    .map(|cell| !cell)
                    .unwrap_or(true);
                let next_empty = row.get(x + 1).map(|cell| !cell).unwrap_or(true);

                let is_corner = (previous_empty || next_empty) && !(previous_empty && next_empty);

                if is_corner {
                    let corner_type = previous_row.get(x).cloned().unwrap_or_default();

                    if let Some(entry) = line_entry {
                        if corner_type != entry {
                            // Corners pointing in opposite directions
                            inside = !inside;
                        }

                        line_entry = None;
                    } else {
                        line_entry = Some(corner_type);
                    }
                } else if line_entry.is_none() {
                    inside = !inside;
                }
            } else {
                // Empty
                if inside {
                    filled = true;
                }

                line_entry = None;
            }

            new_row.push(*cell);
            if filled {
                count += 1;
            }

            print!("{}", if filled { "#" } else { "." });
        }

        previous_row = new_row;
        println!("");
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<u128> {
    let mut min_x = 0;
    let mut max_x = 0;

    let corners = {
        let mut corners = input
            .lines()
            .map(|line| {
                let mut iter = line.split_whitespace();

                let mut encoded = iter
                    .nth(2)
                    .expect("encoded value")
                    .chars()
                    .skip(2)
                    .take(6)
                    .map(|c| c.to_digit(16).expect("valid hex digit"));

                let distance = (0u32..5)
                    .rev()
                    .map(|i| encoded.next().expect("distance digit") * 16u32.pow(i))
                    .sum::<u32>();
                let direction = Direction::try_from(encoded.next().expect("direction digit"))
                    .expect("valid direction");

                (direction, distance)
            })
            .scan((0isize, 0isize), |(x, y), (direction, distance)| {
                let (dx, dy) = direction.into();

                let result = Some((*x, *y));

                *x += dx * distance as isize;
                *y += dy * distance as isize;

                result
            })
            .inspect(|&(x, _)| {
                max_x = max_x.max(x);
                min_x = min_x.min(x);
            })
            .fold(
                HashMap::<isize, Vec<isize>>::new(),
                |mut corners, (x, y)| {
                    corners.entry(y).or_default().push(x);

                    corners
                },
            )
            .into_iter()
            .map(|(y, mut x_positions)| {
                x_positions.sort_unstable();

                (y, x_positions)
            })
            .collect::<Vec<_>>();

        corners.sort_unstable_by_key(|(y, _)| *y);

        corners
    };

    let mut active = vec![false; (max_x - min_x) as usize + 1];
    let mut count = 0u128;
    let mut last_y = corners[0].0;

    for (y, x_positions) in corners {
        let (add, remove) = x_positions
            .chunks_exact(2)
            .map(|chunk| ((chunk[0] - min_x) as usize, (chunk[1] - min_x) as usize))
            .map(|(mut start, mut end)| {
                // Modify start/end depending on where other active cells are

                // RULES:
                // If nothing next to it, leave in place
                // If something next to it, decrease the range

                if start
                    .checked_sub(1)
                    .and_then(|start| active.get(start))
                    .cloned()
                    .unwrap_or_default()
                {
                    start += 1;
                }

                if active.get(end + 1).cloned().unwrap_or_default() {
                    end -= 1;
                }

                (start, end)
            })
            .flat_map(|(start, end)| start..=end)
            .fold((Vec::new(), Vec::new()), |(mut add, mut remove), i| {
                if active[i] {
                    remove.push(i);
                } else {
                    add.push(i);
                }

                (add, remove)
            });

        let current_count = active.iter().filter(|&&active| active).count() as u128;
        while last_y < y {
            // Update count with the currently active cells
            count += current_count;
            last_y += 1;

            // active
            //     .iter()
            //     .for_each(|&x| print!("{}", if x { "#" } else { "." }));
            // println!("");
        }

        // Activate indexes as required
        for x in add {
            active[x] = true;
        }

        // active
        //     .iter()
        //     .for_each(|&x| print!("{}", if x { "#" } else { "." }));
        // println!("");

        last_y += 1;
        count += active.iter().filter(|&&active| active).count() as u128;

        for x in remove {
            active[x] = false;
        }
    }

    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(62));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(952408144115));
    }
}
