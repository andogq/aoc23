advent_of_code::solution!(11);

fn solve(input: &str, expansion_factor: u64) -> u64 {
    let universe = input
        .lines()
        .map(|line| line.chars().map(|c| c == '#').collect::<Vec<_>>())
        .collect::<Vec<_>>();

    // Expand universe
    let row_factors = universe
        .iter()
        .enumerate()
        .map(|(_, row)| {
            if row.iter().all(|c| !c) {
                // Is empty
                expansion_factor
            } else {
                1
            }
        })
        .collect::<Vec<_>>();
    let col_factors = (0..universe[0].len())
        .map(|i| {
            if universe.iter().map(|row| row[i]).all(|c| !c) {
                expansion_factor
            } else {
                1
            }
        })
        .collect::<Vec<_>>();

    let galaxies = universe
        .iter()
        .enumerate()
        .map(|(y, row)| ((0..y).map(|y| row_factors[y as usize]).sum::<u64>(), row))
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, c)| ((0..x).map(|x| col_factors[x as usize]).sum::<u64>(), c))
                .filter(|(_, c)| **c)
                .map(move |(x, _)| (x, y))
        })
        .collect::<Vec<_>>();

    let mut total = 0;

    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            // Calculate the distance between the two pairs
            let ga = galaxies[i];
            let gb = galaxies[j];

            total += ga.0.abs_diff(gb.0) + ga.1.abs_diff(gb.1);
        }
    }

    total
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(solve(input, 2))
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(solve(input, 100))
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
