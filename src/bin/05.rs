use std::collections::{HashSet, VecDeque};

advent_of_code::solution!(5);

struct RangeMap {
    destination_start: u64,
    source_start: u64,
    length: u64,
}

impl TryFrom<&str> for RangeMap {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut iter = value.split_whitespace().flat_map(|n| n.parse::<u64>());

        Ok(Self {
            destination_start: iter.next().ok_or(())?,
            source_start: iter.next().ok_or(())?,
            length: iter.next().ok_or(())?,
        })
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut iter = input.split("\n\n");

    let seeds = iter
        .next()?
        .split_once(": ")?
        .1
        .split_whitespace()
        .flat_map(|n| n.parse::<u64>());

    let maps = iter
        .map(|map| {
            map.lines()
                .skip(1)
                .flat_map(RangeMap::try_from)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    seeds
        .map(|seed| {
            maps.iter().fold(seed, |seed, map| {
                map.iter()
                    .find_map(|range| {
                        if range.source_start <= seed && seed - range.source_start < range.length {
                            Some(seed - range.source_start + range.destination_start)
                        } else {
                            None
                        }
                    })
                    .unwrap_or(seed)
            })
        })
        .min()
        .map(|n| n as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut iter = input.split("\n\n");

    let seeds = iter
        .next()?
        .split_once(": ")?
        .1
        .split_whitespace()
        .flat_map(|n| n.parse::<u64>())
        .collect::<Vec<_>>();

    let maps = iter
        .map(|map| {
            let mut map = map
                .lines()
                .skip(1)
                .flat_map(RangeMap::try_from)
                .collect::<Vec<_>>();

            map.sort_unstable_by_key(|range| range.source_start);

            map
        })
        .collect::<Vec<_>>();

    let mut chunks = seeds
        .chunks(2)
        .map(|chunk| (chunk[0], chunk[0] + chunk[1] - 1))
        .collect::<VecDeque<_>>();

    maps.into_iter().for_each(|map| {
        // Apply range transformations to chunks
        let mut next_chunks = HashSet::new();

        while let Some((start, end)) = chunks.pop_front() {
            let mut found = false;
            for range in map.iter() {
                let start_valid =
                    start >= range.source_start && start < range.source_start + range.length;
                let end_valid =
                    end >= range.source_start && end < range.source_start + range.length;

                if start_valid && end_valid {
                    // Entire chunk is fine
                    next_chunks.insert((
                        start + range.destination_start - range.source_start,
                        end + range.destination_start - range.source_start,
                    ));
                    found = true;
                } else if start_valid {
                    chunks.push_back((start, range.source_start + range.length - 1));
                    chunks.push_back((range.source_start + range.length, end));
                    found = true;
                } else if end_valid {
                    chunks.push_back((start, range.source_start - 1));
                    chunks.push_back((range.source_start, end));
                    found = true;
                } else if start < range.source_start && end >= range.source_start + range.length {
                    chunks.push_back((start, range.source_start - 1));
                    chunks.push_back((range.source_start, range.source_start + range.length - 1));
                    chunks.push_back((range.source_start + range.length, end));
                    found = true;
                }
            }

            if !found {
                next_chunks.insert((start, end));
            }
        }

        chunks = next_chunks.into_iter().collect();
    });

    // Find the smallest chunk
    chunks
        .into_iter()
        .min_by_key(|(start, _)| *start)
        .map(|(start, _)| start as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
