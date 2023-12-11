use std::collections::{HashMap, HashSet, VecDeque};

use num::Integer;

advent_of_code::solution!(10);

#[derive(Clone, Copy, Debug)]
enum Tile {
    Vertical,
    Horizontal,
    NE,
    NW,
    SW,
    SE,
    Start,
    Ground,
}

impl TryFrom<char> for Tile {
    type Error = ();

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '|' => Ok(Self::Vertical),
            '-' => Ok(Self::Horizontal),
            'L' => Ok(Self::NE),
            'J' => Ok(Self::NW),
            '7' => Ok(Self::SW),
            'F' => Ok(Self::SE),
            'S' => Ok(Self::Start),
            '.' => Ok(Self::Ground),
            _ => Err(()),
        }
    }
}

impl Tile {
    fn adjacent_coords(self: &Tile) -> &[(isize, isize)] {
        match self {
            Self::Vertical => &[(0, 1), (0, -1)],
            Self::Horizontal => &[(1, 0), (-1, 0)],
            Self::NE => &[(0, -1), (1, 0)],
            Self::NW => &[(0, -1), (-1, 0)],
            Self::SW => &[(0, 1), (-1, 0)],
            Self::SE => &[(0, 1), (1, 0)],
            Self::Start => &[(0, 1), (1, 0), (0, -1), (-1, 0)],
            Self::Ground => &[],
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let map = input
        .lines()
        .map(|line| {
            line.chars()
                .map(Tile::try_from)
                .collect::<Result<Vec<_>, _>>()
        })
        .collect::<Result<Vec<_>, _>>()
        .expect("valid pipe map");

    let start = map
        .iter()
        .enumerate()
        .find_map(|(y, line)| {
            line.iter()
                .enumerate()
                .find_map(|(x, tile)| {
                    if let Tile::Start = tile {
                        Some(x)
                    } else {
                        None
                    }
                })
                .map(|x| (x, y))
        })
        .expect("starting location");

    let mut tile_scores = HashMap::new();

    let mut current_positions = VecDeque::from_iter([(start, Tile::Start, 0, None)].into_iter());
    tile_scores.insert(start, 0);

    while let Some((pos, tile, score, previous)) = current_positions.pop_front() {
        let adjacent_tiles = tile
            .adjacent_coords()
            .into_iter()
            .cloned()
            .flat_map(|(dx, dy)| {
                Some((pos.0.checked_add_signed(dx)?, pos.1.checked_add_signed(dy)?))
            })
            .flat_map(|(x, y)| Some(((x, y), *map.get(y)?.get(x)?)))
            // .filter(|(_, tile)| !matches!(tile, Tile::Ground))
            .collect::<Vec<_>>();

        let adjacent_tile_count = adjacent_tiles.len();

        let new_tiles = adjacent_tiles
            .into_iter()
            .filter(|(pos, _)| !tile_scores.contains_key(pos))
            .filter(|(pos, _)| previous.map(|previous| previous != *pos).unwrap_or(true))
            .map(|(pos, tile)| (pos, tile, score + 1))
            .collect::<Vec<_>>();

        // if adjacent_tile_count == 0 && new_tiles.is_empty() {
        //     // Found a loop
        //     dbg!(pos);
        //     break;
        // }

        for (new_pos, new_tile, new_score) in new_tiles {
            // Insert the new scores into the map and add more tiles to search
            tile_scores.insert(new_pos, new_score);

            current_positions.push_back((new_pos, new_tile, new_score, Some(pos)));
        }
    }

    tile_scores.values().max().cloned()
}

pub fn part_two(input: &str) -> Option<u32> {
    let loop_tiles = {
        let map = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(Tile::try_from)
                    .collect::<Result<Vec<_>, _>>()
            })
            .collect::<Result<Vec<_>, _>>()
            .expect("valid pipe map");

        let start = map
            .iter()
            .enumerate()
            .find_map(|(y, line)| {
                line.iter()
                    .enumerate()
                    .find_map(|(x, tile)| {
                        if let Tile::Start = tile {
                            Some(x)
                        } else {
                            None
                        }
                    })
                    .map(|x| (x, y))
            })
            .expect("starting location");

        let mut loop_tiles = HashSet::new();

        let mut current_positions =
            VecDeque::from_iter([(start, Tile::Start, 0, None)].into_iter());
        loop_tiles.insert(start);

        while let Some((pos, tile, score, previous)) = current_positions.pop_front() {
            let adjacent_tiles = tile
                .adjacent_coords()
                .into_iter()
                .cloned()
                .flat_map(|(dx, dy)| {
                    Some((pos.0.checked_add_signed(dx)?, pos.1.checked_add_signed(dy)?))
                })
                .flat_map(|(x, y)| Some(((x, y), *map.get(y)?.get(x)?)))
                // .filter(|(_, tile)| !matches!(tile, Tile::Ground))
                .collect::<Vec<_>>();

            let new_tiles = adjacent_tiles
                .into_iter()
                .filter(|(pos, _)| !loop_tiles.contains(pos))
                .filter(|(pos, _)| previous.map(|previous| previous != *pos).unwrap_or(true))
                .map(|(pos, tile)| (pos, tile, score + 1))
                .collect::<Vec<_>>();

            // if adjacent_tile_count == 0 && new_tiles.is_empty() {
            //     // Found a loop
            //     dbg!(pos);
            //     break;
            // }

            for (new_pos, new_tile, new_score) in new_tiles {
                // Insert the new scores into the map and add more tiles to search
                loop_tiles.insert(new_pos);

                current_positions.push_back((new_pos, new_tile, new_score, Some(pos)));
            }
        }

        loop_tiles
    };

    let map = {
        let mut map = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(Tile::try_from)
                    .collect::<Result<Vec<_>, _>>()
            })
            .collect::<Result<Vec<_>, _>>()
            .expect("valid pipe map");

        // Find and replace start with the relevant piece
        let start_pos = map
            .iter()
            .enumerate()
            .find_map(|(y, line)| {
                line.iter()
                    .enumerate()
                    .find_map(|(x, tile)| {
                        if matches!(tile, Tile::Start) {
                            Some(x)
                        } else {
                            None
                        }
                    })
                    .map(|x| (x, y))
            })
            .expect("start position");

        let start_tile = [
            ((0isize, 1isize), (0isize, -1isize), Tile::Vertical),
            ((1, 0), (-1, 0), Tile::Horizontal),
            ((1, 0), (0, -1), Tile::NE),
            ((-1, 0), (0, -1), Tile::NW),
            ((1, 0), (0, 1), Tile::SE),
            ((-1, 0), (0, 1), Tile::SW),
        ]
        .into_iter()
        .flat_map(|(offset_1, offset_2, tile)| {
            Some((
                (
                    start_pos.0.checked_add_signed(offset_1.0)?,
                    start_pos.1.checked_add_signed(offset_1.1)?,
                ),
                (
                    start_pos.0.checked_add_signed(offset_2.0)?,
                    start_pos.1.checked_add_signed(offset_2.1)?,
                ),
                tile,
            ))
        })
        .flat_map(|(offset_1, offset_2, tile)| {
            Some((
                map.get(offset_1.1)?.get(offset_1.0)?,
                map.get(offset_2.1)?.get(offset_2.0)?,
                tile,
            ))
        })
        .filter(|(tile_1, tile_2, _)| {
            !matches!(tile_1, Tile::Ground) && !matches!(tile_2, Tile::Ground)
        })
        .map(|(_, _, tile)| tile)
        .next()
        .expect("equivalent start tile");

        map[start_pos.1][start_pos.0] = dbg!(start_tile);

        map
    };

    let mut count = 0;

    for (y, line) in map.iter().enumerate() {
        // Whether currently tracing a line
        let mut inside = false;

        // Tile that was used to enter a line
        let mut line_entry = None;

        for (x, tile) in line.iter().enumerate() {
            match (tile, loop_tiles.contains(&(x, y))) {
                (Tile::Ground, _) | (_, false) => {
                    if inside {
                        // dbg!((x, y));
                        count += 1;
                    }

                    if line_entry.is_some() {
                        line_entry = None;
                    }
                }
                (Tile::Horizontal | Tile::Vertical, true) => {
                    if line_entry.is_some() {
                        continue;
                    } else {
                        inside = !inside;
                        dbg!(inside, (x, y));
                    }
                }
                (Tile::NE | Tile::NW | Tile::SE | Tile::SW, true) => {
                    if let Some(entry) = line_entry {
                        if ((matches!(entry, Tile::NW) || matches!(entry, Tile::NE))
                            && (matches!(tile, Tile::SE) || matches!(tile, Tile::SW)))
                            || ((matches!(entry, Tile::SW) || matches!(entry, Tile::SE))
                                && (matches!(tile, Tile::NE) || matches!(tile, Tile::NW)))
                        {
                            // Entry is pointing opposite direction to exit, changing shape side
                            inside = !inside;
                            dbg!(inside, (x, y));
                        }

                        line_entry = None;
                    } else {
                        line_entry = Some(*tile);
                    }
                }
                (Tile::Start, true) => unreachable!("start tile should be removed"),
            }
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
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10));
    }
}
