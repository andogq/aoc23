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

pub fn part_one(input: &str) -> Option<u32> {
    let mut hands = input
        .lines()
        .flat_map(|line| line.split_once(" "))
        .flat_map(|(hand, bid)| {
            Some::<([u32; 5], _)>((
                hand.chars()
                    .flat_map(|card| match card {
                        '2'..='9' => card.to_digit(10),
                        'T' => Some(10),
                        'J' => Some(11),
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
        .map(|(hand, bid)| {
            (
                {
                    let mut sorted_hand = hand.clone();
                    sorted_hand.sort_unstable();
                    let mut sorted_hand = sorted_hand.into_iter().peekable();

                    let mut current_guess = HandType::HighCard;

                    while let Some(card) = sorted_hand.next() {
                        let mut same = 1;
                        while sorted_hand.peek().map(|&c| c == card).unwrap_or_default() {
                            sorted_hand.next();
                            same += 1;
                        }

                        current_guess = match same {
                            5 => HandType::FiveOfAKind,
                            4 => HandType::FourOfAKind,
                            3 if matches!(current_guess, HandType::OnePair) => HandType::FullHouse,
                            3 => HandType::ThreeOfAKind,
                            2 if matches!(current_guess, HandType::OnePair) => HandType::TwoPair,
                            2 if matches!(current_guess, HandType::ThreeOfAKind) => {
                                HandType::FullHouse
                            }
                            2 => HandType::OnePair,
                            _ => current_guess,
                        };
                    }

                    current_guess
                },
                hand,
                bid,
            )
        })
        .collect::<Vec<_>>();

    hands.sort_unstable_by_key(|(ty, hand, _)| (*ty as u32, hand.clone()));

    Some(
        hands
            .into_iter()
            .enumerate()
            .map(|(rank, (_, _, bid))| (rank as u32 + 1) * bid)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut hands = input
        .lines()
        .flat_map(|line| line.split_once(" "))
        .flat_map(|(hand, bid)| {
            Some::<([u32; 5], _)>((
                hand.chars()
                    .flat_map(|card| match card {
                        'J' => Some(1),
                        '2'..='9' => card.to_digit(10),
                        'T' => Some(10),
                        'Q' => Some(11),
                        'K' => Some(12),
                        'A' => Some(13),
                        _ => None,
                    })
                    .collect::<Vec<_>>()
                    .try_into()
                    .ok()?,
                bid.parse::<u32>().ok()?,
            ))
        })
        .flat_map(|(hand, bid)| {
            Some((
                {
                    let mut sorted_hand = hand
                        .iter()
                        .filter(|&&c| c != 1)
                        .cloned()
                        .collect::<Vec<_>>();
                    sorted_hand.sort_unstable();

                    let joker_count = hand.len() - sorted_hand.len();

                    // Find the most popular card in the hand
                    let mut card_counts = Vec::new();

                    let mut hand_iter = sorted_hand.iter().peekable();
                    while let Some(card) = hand_iter.next() {
                        let mut count = 1;
                        while hand_iter.peek().map(|&c| c == card).unwrap_or_default() {
                            hand_iter.next();
                            count += 1;
                        }

                        card_counts.push(count);
                    }

                    card_counts.sort_unstable();
                    card_counts.reverse();
                    let mut iter = card_counts.into_iter();

                    let max_count = iter.next().unwrap_or_default() + joker_count;
                    let next_count = iter.next();

                    match (max_count, next_count) {
                        (5, None) => HandType::FiveOfAKind,
                        (4, _) => HandType::FourOfAKind,
                        (3, Some(2)) => HandType::FullHouse,
                        (3, _) => HandType::ThreeOfAKind,
                        (2, Some(3)) => HandType::FullHouse,
                        (2, Some(2)) => HandType::TwoPair,
                        (2, _) => HandType::OnePair,
                        (1, _) => HandType::HighCard,
                        _ => unreachable!(),
                    }
                },
                hand,
                bid,
            ))
        })
        .collect::<Vec<_>>();

    hands.sort_unstable_by_key(|(ty, hand, _)| (*ty as u32, hand.clone()));

    Some(
        hands
            .into_iter()
            .enumerate()
            .map(|(rank, (_, _, bid))| (rank as u32 + 1) * bid)
            .sum(),
    )
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
