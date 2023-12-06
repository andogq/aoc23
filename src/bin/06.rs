advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let mut iter = input.lines().map(|line| {
        line.split_whitespace()
            .skip(1)
            .flat_map(|n| n.parse::<u32>())
    });
    let time = iter.next()?;
    let distance = iter.next()?;
    let races = time.zip(distance);

    Some(
        races
            .into_iter()
            .map(|(time, record)| {
                (1..=time)
                    .map(move |hold_time| hold_time * (time - hold_time))
                    .filter(move |&time| time > record)
                    .count() as u32
            })
            .product(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut iter = input
        .lines()
        .flat_map(|line| line.replace(" ", "").split_once(":")?.1.parse::<u64>().ok());
    let time = iter.next()?;
    let record = iter.next()?;

    Some(
        (1..=time)
            .map(move |hold_time| hold_time * (time - hold_time))
            .filter(move |&time| time > record)
            .count() as u32,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
