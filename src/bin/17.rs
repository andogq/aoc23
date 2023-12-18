use std::collections::{HashMap, VecDeque};

advent_of_code::solution!(17);

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn flip(&self) -> [Self; 2] {
        match self {
            Self::North | Self::South => [Self::East, Self::West],
            Self::East | Self::West => [Self::North, Self::South],
        }
    }
}

impl From<Direction> for (isize, isize) {
    fn from(direction: Direction) -> Self {
        match direction {
            Direction::North => (0, -1),
            Direction::East => (1, 0),
            Direction::South => (0, 1),
            Direction::West => (-1, 0),
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let map = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).expect("valid digit"))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let (goal_x, goal_y) = (map[0].len() - 1, map.len() - 1);

    let mut pointers = VecDeque::from_iter([
        ((0, 0), 0, 1, Direction::East),
        ((0, 0), 0, 1, Direction::South),
    ]);

    let mut tile_min_scores = HashMap::<((usize, usize), u32, Direction), u32>::from_iter([
        (((0, 0), 1, Direction::East), 0),
        (((0, 0), 1, Direction::South), 0),
    ]);

    let mut min_score = None;

    while let Some(((x, y), score, distance, direction)) = pointers.pop_front() {
        // Check if end is reached
        if x == goal_x
            && y == goal_y
            && min_score.map(|min_score| score < min_score).unwrap_or(true)
        {
            min_score = Some(score);
        } else {
            pointers.extend(
                // Rotate direction and continue in both directions
                direction
                    .flip()
                    .into_iter()
                    .map(|direction| {
                        let (dx, dy) = direction.into();

                        ((dx, dy), 1, direction)
                    })
                    .chain(if distance < 3 {
                        // Continue in same direction
                        let (dx, dy) = direction.into();

                        Some(((dx, dy), distance + 1, direction))
                    } else {
                        None
                    })
                    .filter_map(|((dx, dy), distance, direction)| {
                        // Make sure that doesn't move below zero
                        let (x, y) = (x.checked_add_signed(dx)?, y.checked_add_signed(dy)?);

                        // Make sure doesn't go past bounds of map
                        if x < map[0].len() && y < map.len() {
                            Some(((x, y), distance, direction))
                        } else {
                            None
                        }
                    })
                    .filter_map(|((x, y), distance, direction)| {
                        // Calculate a new score for each branch
                        let score = score + map[y][x];

                        if min_score
                            .map(|min_score| score <= min_score)
                            .unwrap_or(true)
                            && tile_min_scores
                                .get(&((x, y), distance, direction))
                                .map(|&min_score| score < min_score)
                                .unwrap_or(true)
                        {
                            // Mark this tile as visited
                            tile_min_scores.insert(((x, y), distance, direction), score);

                            Some(((x, y), score, distance, direction))
                        } else {
                            None
                        }
                    }),
            );
        }
    }

    min_score
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).expect("valid digit"))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let (goal_x, goal_y) = (map[0].len() - 1, map.len() - 1);

    let mut pointers = VecDeque::from_iter([
        ((3, 0), map[0][1..=3].iter().sum(), 4, Direction::East),
        (
            (0, 3),
            map[1..=3].iter().map(|row| row[0]).sum(),
            4,
            Direction::South,
        ),
    ]);

    let mut tile_min_scores = HashMap::<((usize, usize), u32, Direction), u32>::from_iter(
        pointers
            .iter()
            .cloned()
            .map(|((x, y), score, distance, direction)| (((x, y), distance, direction), score)),
    );

    // Determine a worst-case scenario score
    let mut min_score = None;

    while let Some(((x, y), score, distance, direction)) = pointers.pop_front() {
        // Check if end is reached
        if x == goal_x
            && y == goal_y
            && distance >= 4
            && min_score.map(|min_score| score < min_score).unwrap_or(true)
        {
            min_score = Some(score);
        } else {
            pointers.extend(
                // Rotate direction and continue in both directions
                direction
                    .flip()
                    .into_iter()
                    // Only turn after minimum of 4 steps
                    .filter(|_| distance >= 4)
                    .map(|direction| {
                        let (dx, dy) = direction.into();

                        ((dx, dy), 1, direction)
                    })
                    .chain(if distance < 10 {
                        // Continue in same direction
                        let (dx, dy) = direction.into();

                        Some(((dx, dy), distance + 1, direction))
                    } else {
                        None
                    })
                    .filter_map(|((dx, dy), distance, direction)| {
                        // Make sure that doesn't move below zero
                        let (x, y) = (x.checked_add_signed(dx)?, y.checked_add_signed(dy)?);

                        // Make sure doesn't go past bounds of map
                        if x < map[0].len() && y < map.len() {
                            Some(((x, y), distance, direction))
                        } else {
                            None
                        }
                    })
                    .filter_map(|((x, y), distance, direction)| {
                        // Calculate a new score for each branch
                        let score = score + map[y][x];

                        if min_score
                            .map(|min_score| score <= min_score)
                            .unwrap_or(true)
                            && tile_min_scores
                                .get(&((x, y), distance, direction))
                                .map(|&min_score| score < min_score)
                                .unwrap_or(true)
                        {
                            // Mark this tile as visited
                            tile_min_scores.insert(((x, y), distance, direction), score);

                            Some(((x, y), score, distance, direction))
                        } else {
                            None
                        }
                    }),
            );
        }
    }

    min_score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(102));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(94));
    }
}
