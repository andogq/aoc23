advent_of_code::solution!(13);

pub fn part_one(input: &str) -> Option<u32> {
    let input = input
        .split("\n\n")
        .map(|section| {
            section
                .lines()
                .map(|line| {
                    line.chars()
                        .map(|c| match c {
                            '#' => true,
                            '.' => false,
                            _ => unreachable!("invalid character in input"),
                        })
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut score = 0;

    for section in input {
        let mut found_reflection = false;

        for x in 1..(section[0].len()) {
            let distance = if x <= section[0].len() / 2 {
                x
            } else {
                section[0].len() - x
            };

            // dbg!(x, distance);

            let reflected = (0..distance).into_iter().all(|d| {
                section.iter().all(|line| {
                    dbg!(x, d);
                    let line_a = line[x - d - 1];
                    let line_b = line[x + d];

                    line_a == line_b
                })
            });

            if reflected {
                println!("vertical reflection at x = {x}");
                score += x as u32;
                found_reflection = true;
                break;
            }
        }

        if found_reflection {
            continue;
        }

        for y in 1..(section.len()) {
            let distance = if y < section.len() / 2 {
                y
            } else {
                section.len() - y
            };

            // dbg!(y, distance);

            let reflected = (0..distance)
                .into_iter()
                .all(|d| (0..section[y].len()).all(|x| section[y + d][x] == section[y - d - 1][x]));

            if reflected {
                println!("horizontal reflection at y = {y}");
                score += y as u32 * 100;
                break;
            }
        }
    }

    Some(score)
}

pub fn part_two(input: &str) -> Option<u32> {
    let input = input
        .split("\n\n")
        .map(|section| {
            section
                .lines()
                .map(|line| {
                    line.chars()
                        .map(|c| match c {
                            '#' => true,
                            '.' => false,
                            _ => unreachable!("invalid character in input"),
                        })
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut score = 0;

    for section in input {
        let mut found_reflection = false;

        for x in 1..(section[0].len()) {
            let distance = if x <= section[0].len() / 2 {
                x
            } else {
                section[0].len() - x
            };

            // dbg!(x, distance);

            let reflected = (0..distance)
                .into_iter()
                .map(|d| {
                    section
                        .iter()
                        .filter(|line| {
                            dbg!(x, d);
                            let line_a = line[x - d - 1];
                            let line_b = line[x + d];

                            line_a != line_b
                        })
                        .count()
                })
                .sum::<usize>()
                == 1;

            if reflected {
                println!("vertical reflection at x = {x}");
                score += x as u32;
                found_reflection = true;
                break;
            }
        }

        if found_reflection {
            continue;
        }

        for y in 1..(section.len()) {
            let distance = if y <= section.len() / 2 {
                y
            } else {
                section.len() - y
            };

            // dbg!(y, distance);

            let reflected = (0..distance)
                .into_iter()
                .map(|d| {
                    (0..section[y].len())
                        .filter(|&x| section[y + d][x] != section[y - d - 1][x])
                        .count()
                })
                .sum::<usize>()
                == 1;

            if reflected {
                println!("horizontal reflection at y = {y}");
                score += y as u32 * 100;
                break;
            }
        }
    }

    Some(score)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(400));
    }
}
