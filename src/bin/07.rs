advent_of_code::solution!(7);

#[repr(u32)]
#[derive(Debug, Clone, Copy)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl From<&[u32; 5]> for HandType {
    fn from(hand: &[u32; 5]) -> Self {
        // Count all non-joker cards
        let counts =
            hand.iter()
                .filter(|&&c| c != JOKER_VALUE)
                .fold([0u32; 15], |mut counts, &card| {
                    counts[card as usize] += 1;
                    counts
                });

        let (max_count, next_count) = counts.into_iter().fold((0, 0), |(max, max_2), count| {
            if count >= max {
                (count, max)
            } else if count >= max_2 {
                (max, count)
            } else {
                (max, max_2)
            }
        });

        let joker_count = hand.iter().filter(|&&c| c == JOKER_VALUE).count() as u32;

        match (max_count + joker_count, next_count) {
            (5, _) => HandType::FiveOfAKind,
            (4, _) => HandType::FourOfAKind,
            (3, 2) => HandType::FullHouse,
            (3, _) => HandType::ThreeOfAKind,
            (2, 3) => HandType::FullHouse,
            (2, 2) => HandType::TwoPair,
            (2, _) => HandType::OnePair,
            (1, _) => HandType::HighCard,
            _ => unreachable!(),
        }
    }
}

const JACK_VALUE: u32 = 11;
const JOKER_VALUE: u32 = 1;

fn parse_hands(input: &str, j_value: u32) -> impl Iterator<Item = ([u32; 5], u32)> + '_ {
    input
        .lines()
        .flat_map(|line| line.split_once(" "))
        .flat_map(move |(hand, bid)| {
            Some::<([u32; 5], _)>((
                hand.chars()
                    .flat_map(|card| match card {
                        '2'..='9' => card.to_digit(10),
                        'T' => Some(10),
                        'J' => Some(j_value),
                        'Q' => Some(12),
                        'K' => Some(13),
                        'A' => Some(14),
                        _ => None,
                    })
                    .collect::<Vec<_>>()
                    .try_into()
                    .ok()?,
                bid.parse::<u32>().ok()?,
            ))
        })
}

fn run_game(hands: impl Iterator<Item = ([u32; 5], u32)>) -> u32 {
    let mut hands = hands
        .map(|(hand, bid)| (HandType::from(&hand), hand, bid))
        .collect::<Vec<_>>();

    hands.sort_unstable_by_key(|(ty, hand, _)| (*ty as u32, hand.clone()));

    hands
        .into_iter()
        .enumerate()
        .map(|(rank, (_, _, bid))| (rank as u32 + 1) * bid)
        .sum()
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(run_game(parse_hands(input, JACK_VALUE)))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(run_game(parse_hands(input, JOKER_VALUE)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
