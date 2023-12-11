use std::collections::HashSet;

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u32> {
    let mut universe = input
        .lines()
        .map(|line| line.chars().map(|c| c == '#').collect::<Vec<_>>())
        .collect::<Vec<_>>();

    // Expand universe
    let mut row_i = 0;
    while row_i < universe.len() {
        let row = &universe[row_i];

        if row.iter().all(|c| !c) {
            // Insert an empty row since it only contains space
            universe.insert(row_i, row.clone());
            row_i += 1;
        }

        row_i += 1;
    }

    let mut col_i = 0;
    while col_i < universe[0].len() {
        let mut col = universe.iter().map(|row| row[col_i]);

        if col.all(|c| !c) {
            // Insert a new item in each row
            universe.iter_mut().for_each(|row| row.insert(col_i, false));
            col_i += 1;
        }

        col_i += 1;
    }

    let galaxies = universe
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, c)| **c)
                .map(move |(x, _)| (x as u32, y as u32))
        })
        .collect::<Vec<_>>();

    let mut total = 0;

    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            // Calculate the distance between the two pairs
            let galaxy_a = galaxies[i];
            let galaxy_b = galaxies[j];

            total += galaxy_a.0.abs_diff(galaxy_b.0) + galaxy_a.1.abs_diff(galaxy_b.1);
        }
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u128> {
    let universe = input
        .lines()
        .map(|line| line.chars().map(|c| c == '#').collect::<Vec<_>>())
        .collect::<Vec<_>>();

    // Expand universe
    let empty_rows = universe
        .iter()
        .enumerate()
        .filter(|(_, row)| row.iter().all(|c| !c))
        .map(|(i, _)| i as u32)
        .collect::<HashSet<_>>();
    let empty_cols = (0u32..universe[0].len() as u32)
        .filter(|&i| universe.iter().map(|row| row[i as usize]).all(|c| !c))
        .collect::<HashSet<_>>();

    let galaxies = universe
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, c)| **c)
                .map(move |(x, _)| (x as u32, y as u32))
        })
        .collect::<Vec<_>>();

    let mut total = 0;

    const EXPANSION_FACTOR: u128 = 1_000_000;

    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            // Calculate the distance between the two pairs
            let galaxy_a = galaxies[i];
            let galaxy_b = galaxies[j];

            // Traverse columns
            let min = galaxy_a.0.min(galaxy_b.0);
            let max = galaxy_a.0.max(galaxy_b.0);

            let column_distance = (min..max)
                .map(|col| {
                    if empty_cols.contains(&col) {
                        EXPANSION_FACTOR
                    } else {
                        1
                    }
                })
                .sum::<u128>();

            let min = galaxy_a.1.min(galaxy_b.1);
            let max = galaxy_a.1.max(galaxy_b.1);

            let row_distance = (min..max)
                .map(|row| {
                    if empty_rows.contains(&row) {
                        EXPANSION_FACTOR
                    } else {
                        1
                    }
                })
                .sum::<u128>();

            total += column_distance + row_distance;
        }
    }

    Some(total as u128)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8410));
    }
}
