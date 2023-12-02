advent_of_code::solution!(2);

#[derive(Clone, Copy)]
#[repr(u8)]
enum Color {
    Red = 0,
    Green,
    Blue,
}

impl Color {
    fn valid_amount(&self, amount: u32) -> bool {
        match self {
            Self::Red if amount <= 12 => true,
            Self::Green if amount <= 13 => true,
            Self::Blue if amount <= 14 => true,
            _ => false,
        }
    }
}

impl TryFrom<&str> for Color {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "red" => Ok(Self::Red),
            "green" => Ok(Self::Green),
            "blue" => Ok(Self::Blue),
            _ => Err(()),
        }
    }
}

struct Pull {
    pub color: Color,
    pub amount: u32,
}

impl TryFrom<&str> for Pull {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value
            .split_once(" ")
            .and_then(|(amount, color)| {
                Some(Self {
                    amount: amount.parse::<u32>().ok()?,
                    color: Color::try_from(color).ok()?,
                })
            })
            .ok_or(())
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .flat_map(|line| line.split_once(": "))
            .flat_map(|line| {
                line.1
                    .split("; ")
                    .all(|pull| {
                        pull.split(", ")
                            .flat_map(Pull::try_from)
                            .all(|pull| pull.color.valid_amount(pull.amount))
                    })
                    // Parse and return game ID
                    .then_some(line.0.split_once(" ")?.1.parse::<u32>().ok()?)
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .flat_map(|line| line.split_once(": "))
            .map(|line| {
                line.1
                    .split("; ")
                    .flat_map(|pull| pull.split(", ").flat_map(Pull::try_from))
                    .fold([0, 0, 0], |mut counts, pull| {
                        counts[pull.color as usize] = counts[pull.color as usize].max(pull.amount);
                        counts
                    })
                    .into_iter()
                    .product::<u32>()
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
