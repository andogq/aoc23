use std::collections::{HashMap, HashSet, VecDeque};

advent_of_code::solution!(16);

enum Tile {
    Empty,        // .
    HSplit,       // -
    VSplit,       // |
    ForwardSlash, // /
    BackSlash,    // \
}

impl TryFrom<char> for Tile {
    type Error = ();

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '.' => Ok(Self::Empty),
            '-' => Ok(Self::HSplit),
            '|' => Ok(Self::VSplit),
            '/' => Ok(Self::ForwardSlash),
            '\\' => Ok(Self::BackSlash),
            _ => Err(()),
        }
    }
}

fn move_beam(
    (mut x, mut y): (usize, usize),
    (dx, dy): (isize, isize),
    x_max: usize,
    y_max: usize,
) -> Option<(usize, usize)> {
    x = x.checked_add_signed(dx)?;
    y = y.checked_add_signed(dy)?;

    if x < x_max && y < y_max {
        Some((x, y))
    } else {
        None
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut map = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| (Tile::try_from(c).expect("valid character"), vec![]))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut beams: VecDeque<((usize, usize), (isize, isize))> =
        VecDeque::from_iter([((0, 0), (1, 0))]);

    while let Some(((x, y), (dx, dy))) = beams.pop_front() {
        let (tile, visited_directions) = map
            .get_mut(y)
            .expect("y bound to be valid")
            .get_mut(x)
            .expect("x bound to be valid");

        // Tile is now energised since light has passed through it
        if !visited_directions.contains(&(dx, dy)) {
            visited_directions.push((dx, dy));
        } else {
            continue;
        }

        match tile {
            Tile::Empty => {
                // Continue along in the same direction
                if let Some(beam) = move_beam((x, y), (dx, dy), map[0].len(), map.len()) {
                    beams.push_back((beam, (dx, dy)));
                }
            }
            Tile::HSplit => {
                if dx == 0 && dy != 0 {
                    // Approaching from above or below, split
                    let beam_1_d = (-1, 0);
                    if let Some(beam_1) = move_beam((x, y), beam_1_d, map[0].len(), map.len()) {
                        beams.push_back((beam_1, beam_1_d));
                    }

                    let beam_2_d = (1, 0);
                    if let Some(beam_2) = move_beam((x, y), beam_2_d, map[0].len(), map.len()) {
                        beams.push_back((beam_2, beam_2_d));
                    }
                } else {
                    // Continue on in the same direction
                    if let Some(beam) = move_beam((x, y), (dx, dy), map[0].len(), map.len()) {
                        beams.push_back((beam, (dx, dy)));
                    }
                }
            }
            Tile::VSplit => {
                if dx != 0 && dy == 0 {
                    // Approaching from left or right, split
                    let beam_1_d = (0, -1);
                    if let Some(beam_1) = move_beam((x, y), beam_1_d, map[0].len(), map.len()) {
                        beams.push_back((beam_1, beam_1_d));
                    }

                    let beam_2_d = (0, 1);
                    if let Some(beam_2) = move_beam((x, y), beam_2_d, map[0].len(), map.len()) {
                        beams.push_back((beam_2, beam_2_d));
                    }
                } else {
                    // Continue on in the same direction
                    if let Some(beam) = move_beam((x, y), (dx, dy), map[0].len(), map.len()) {
                        beams.push_back((beam, (dx, dy)));
                    }
                }
            }
            Tile::ForwardSlash => {
                // Swap and negate dx and dy
                let (dx, dy) = (dy * -1, dx * -1);

                // Continue in new direction
                if let Some(beam) = move_beam((x, y), (dx, dy), map[0].len(), map.len()) {
                    beams.push_back((beam, (dx, dy)));
                }
            }
            Tile::BackSlash => {
                // Swap dx and dy
                let (dx, dy) = (dy, dx);

                // Continue in new direction
                if let Some(beam) = move_beam((x, y), (dx, dy), map[0].len(), map.len()) {
                    beams.push_back((beam, (dx, dy)));
                }
            }
        }
    }

    Some(
        map.into_iter()
            .map(|line| {
                line.into_iter()
                    .filter(|(_, energised)| !energised.is_empty())
                    .count() as u32
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| Tile::try_from(c).expect("valid character"))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    fn solve(
        map: &Vec<Vec<Tile>>,
        cache: &mut HashMap<((usize, usize), (isize, isize)), Option<HashSet<(usize, usize)>>>,
        (x, y): (usize, usize),
        (dx, dy): (isize, isize),
    ) -> HashSet<(usize, usize)> {
        if let Some(maybe_path) = cache.get(&((x, y), (dx, dy))) {
            if let Some(path) = maybe_path {
                return path.clone();
            } else {
                // This branch is already being tested, may be in a loop
                return HashSet::new();
            }
        } else {
            // Mark that this is being tested to prevent loops
            cache.insert(((x, y), (dx, dy)), None);

            let tile = &map[y][x];

            let mut path = HashSet::from_iter([(x, y)]);

            match tile {
                Tile::Empty => {
                    if let Some(beam) = move_beam((x, y), (dx, dy), map[0].len(), map.len()) {
                        path.extend(solve(map, cache, beam, (dx, dy)));
                    }
                }
                Tile::HSplit => {
                    if dx == 0 && dy != 0 {
                        // Approaching from above or below, split
                        let beam_1_d = (-1, 0);
                        if let Some(beam_1) = move_beam((x, y), beam_1_d, map[0].len(), map.len()) {
                            path.extend(solve(map, cache, beam_1, beam_1_d));
                        }

                        let beam_2_d = (1, 0);
                        if let Some(beam_2) = move_beam((x, y), beam_2_d, map[0].len(), map.len()) {
                            path.extend(solve(map, cache, beam_2, beam_2_d));
                        }
                    } else {
                        // Continue on in the same direction
                        if let Some(beam) = move_beam((x, y), (dx, dy), map[0].len(), map.len()) {
                            path.extend(solve(map, cache, beam, (dx, dy)));
                        }
                    }
                }
                Tile::VSplit => {
                    if dx != 0 && dy == 0 {
                        // Approaching from above or below, split
                        let beam_1_d = (0, -1);
                        if let Some(beam_1) = move_beam((x, y), beam_1_d, map[0].len(), map.len()) {
                            path.extend(solve(map, cache, beam_1, beam_1_d));
                        }

                        let beam_2_d = (0, 1);
                        if let Some(beam_2) = move_beam((x, y), beam_2_d, map[0].len(), map.len()) {
                            path.extend(solve(map, cache, beam_2, beam_2_d));
                        }
                    } else {
                        // Continue on in the same direction
                        if let Some(beam) = move_beam((x, y), (dx, dy), map[0].len(), map.len()) {
                            path.extend(solve(map, cache, beam, (dx, dy)));
                        }
                    }
                }
                Tile::ForwardSlash => {
                    // Swap and negate dx and dy
                    let (dx, dy) = (dy * -1, dx * -1);

                    // Continue in new direction
                    if let Some(beam) = move_beam((x, y), (dx, dy), map[0].len(), map.len()) {
                        path.extend(solve(map, cache, beam, (dx, dy)));
                    }
                }
                Tile::BackSlash => {
                    // Swap dx and dy
                    let (dx, dy) = (dy, dx);

                    // Continue in new direction
                    if let Some(beam) = move_beam((x, y), (dx, dy), map[0].len(), map.len()) {
                        path.extend(solve(map, cache, beam, (dx, dy)));
                    }
                }
            }

            cache.insert(((x, y), (dx, dy)), Some(path.clone()));

            path
        }
    }

    // let mut cache = HashMap::new();

    // Some(solve(&map, &mut cache, (3, 0), (0, 1)).len() as u32)

    let max = [
        (0..map[0].len())
            .map(|x| ((x, 0), (0, 1)))
            .collect::<Vec<_>>(),
        (0..map[0].len())
            .map(|x| ((x, map.len() - 1), (0, -1)))
            .collect::<Vec<_>>(),
        (0..map.len()).map(|y| ((0, y), (1, 0))).collect::<Vec<_>>(),
        (0..map.len())
            .map(|y| ((map[0].len() - 1, y), (-1, 0)))
            .collect::<Vec<_>>(),
    ]
    .into_iter()
    .flatten()
    // .inspect(|dir| {
    //     dbg!(dir);
    // })
    .map(|((x, y), (dx, dy))| {
        (
            (x, y),
            (dx, dy),
            solve(&map, &mut HashMap::new(), (x, y), (dx, dy)),
        )
    })
    .max_by_key(|(_, _, score)| score.len());

    max.map(|max| max.2.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(51));
    }
}
