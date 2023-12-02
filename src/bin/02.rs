advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .flat_map(|line| {
                let line = line.split_once(": ").unwrap();
                let game_id = line.0.split_once(" ").unwrap().1.parse::<u32>().unwrap();

                let valid = line.1.split("; ").all(|pull| {
                    pull.split(", ").all(|collection| {
                        collection
                            .split_once(" ")
                            .and_then(|(amount, color)| {
                                amount.parse::<u32>().ok().map(|amount| (amount, color))
                            })
                            .map(|(amount, color)| match (color, amount) {
                                ("red", amount) if amount <= 12 => true,
                                ("green", amount) if amount <= 13 => true,
                                ("blue", amount) if amount <= 14 => true,
                                _ => false,
                            })
                            .unwrap()
                    })
                });

                if valid {
                    Some(game_id)
                } else {
                    None
                }
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let line = line.split_once(": ").unwrap();

                let (red, green, blue) = line
                    .1
                    .split("; ")
                    .flat_map(|pull| {
                        pull.split(", ").flat_map(|collection| {
                            collection.split_once(" ").and_then(|(amount, color)| {
                                amount.parse::<u32>().ok().map(|amount| (amount, color))
                            })
                        })
                    })
                    .fold(
                        (0, 0, 0),
                        |(red, green, blue), (amount, color)| match color {
                            "red" => (red.max(amount), green, blue),
                            "green" => (red, green.max(amount), blue),
                            "blue" => (red, green, blue.max(amount)),
                            _ => (red, green, blue),
                        },
                    );

                red * green * blue
            })
            .sum(),
    )
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
        assert_eq!(result, Some(2286));
    }
}
