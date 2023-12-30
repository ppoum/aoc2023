pub fn part1(lines: Vec<String>) -> u32 {
    let (mut hands, bets) = parse_hands(&lines, get_card_value_p1);

    let mut computed_hands: Vec<(HandType, u32)> = Vec::with_capacity(hands.len());
    for (i, hand) in hands.iter_mut().enumerate() {
        let unique_cnt = {
            let mut tmp = hand.clone();
            tmp.sort();
            tmp.dedup();
            tmp.len() as u8
        };

        let hand = construct_hand(hand, unique_cnt);
        computed_hands.push((hand, bets[i]));
    }
    // Sort by hand type
    computed_hands.sort_by(|a, b| a.0.cmp(&b.0));

    computed_hands
        .iter()
        .map(|t| t.1)
        .enumerate()
        .fold(0, |acc, e| acc + e.1 * (e.0 as u32 + 1))
}

pub fn part2(lines: Vec<String>) -> u32 {
    let (mut hands, bets) = parse_hands(&lines, get_card_value_p2);

    // Try to obtain the highest hand type possible
    let mut computed_hands: Vec<(HandType, u32)> = Vec::with_capacity(hands.len());
    for (i, hand) in hands.iter_mut().enumerate() {
        let hand = optimize_hand(hand);
        computed_hands.push((hand, bets[i]));
    }
    // Sort by hand type
    computed_hands.sort_by(|a, b| a.0.cmp(&b.0));

    computed_hands
        .iter()
        .map(|t| t.1)
        .enumerate()
        .fold(0, |acc, e| acc + e.1 * (e.0 as u32 + 1))
}

#[derive(Debug, PartialEq, Eq)]
enum HandType {
    FiveOfAKind(Vec<u8>),
    FourOfAKind(Vec<u8>),
    FullHouse(Vec<u8>),
    ThreeOfAKind(Vec<u8>),
    TwoPair(Vec<u8>),
    OnePair(Vec<u8>),
    HighCard(Vec<u8>),
}

impl PartialOrd for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HandType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Compare if types of hands are different
        // Compare card values if types are the same
        match self {
            HandType::FiveOfAKind(a) => match other {
                HandType::FiveOfAKind(b) => a.cmp(b),
                _ => std::cmp::Ordering::Greater,
            },
            HandType::FourOfAKind(a) => match other {
                HandType::FiveOfAKind(_) => std::cmp::Ordering::Less,
                HandType::FourOfAKind(b) => a.cmp(b),
                _ => std::cmp::Ordering::Greater,
            },
            HandType::FullHouse(a) => match other {
                HandType::FiveOfAKind(_) | HandType::FourOfAKind(_) => std::cmp::Ordering::Less,
                HandType::FullHouse(b) => a.cmp(b),
                _ => std::cmp::Ordering::Greater,
            },
            HandType::ThreeOfAKind(a) => match other {
                HandType::FiveOfAKind(_) | HandType::FourOfAKind(_) | HandType::FullHouse(_) => {
                    std::cmp::Ordering::Less
                }
                HandType::ThreeOfAKind(b) => a.cmp(b),
                _ => std::cmp::Ordering::Greater,
            },
            HandType::TwoPair(a) => match other {
                HandType::HighCard(_) | HandType::OnePair(_) => std::cmp::Ordering::Greater,
                HandType::TwoPair(b) => a.cmp(b),
                _ => std::cmp::Ordering::Less,
            },
            HandType::OnePair(a) => match other {
                HandType::HighCard(_) => std::cmp::Ordering::Greater,
                HandType::OnePair(b) => a.cmp(b),
                _ => std::cmp::Ordering::Less,
            },
            HandType::HighCard(a) => match other {
                HandType::HighCard(b) => a.cmp(b),
                _ => std::cmp::Ordering::Less,
            },
        }
    }
}

fn parse_hands(lines: &[String], value_fun: fn(char) -> u8) -> (Vec<Vec<u8>>, Vec<u32>) {
    lines
        .iter()
        .map(|s| {
            let mut parts = s.split_whitespace();
            (
                parts
                    .next()
                    .unwrap()
                    .chars()
                    .map(value_fun)
                    .collect::<Vec<u8>>(),
                parts.next().unwrap().parse::<u32>().unwrap(),
            )
        })
        .unzip()
}

fn get_card_value_p1(card: char) -> u8 {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        _ => card.to_digit(10).unwrap() as u8,
    }
}

fn construct_hand(cards: &[u8], card_cnt: u8) -> HandType {
    let cards = cards.to_vec();
    let sorted = {
        let mut tmp = cards.clone();
        tmp.sort();
        tmp
    };
    match card_cnt {
        5 => {
            // 5 diff cards, high card only
            HandType::HighCard(cards)
        }
        4 => HandType::OnePair(cards),
        3 => {
            // 2 pairs + 1 rand or 3 of a kind + 2 rand
            let triple = sorted
                .windows(3)
                .filter(|w| w[0] == w[1] && w[1] == w[2])
                .map(|w| w[0])
                .next();

            if triple.is_some() {
                // 3kind + 2 rand
                HandType::ThreeOfAKind(cards)
            } else {
                // 2 pair
                HandType::TwoPair(cards)
            }
        }
        2 => {
            // Full house or 4kind
            let quad = sorted
                .windows(4)
                .find(|w| w[0] == w[1] && w[1] == w[2] && w[2] == w[3])
                .map(|w| w[0]);

            if quad.is_some() {
                // 4kind
                HandType::FourOfAKind(cards)
            } else {
                // full house
                HandType::FullHouse(cards)
            }
        }
        1 => HandType::FiveOfAKind(cards),
        _ => unreachable!(),
    }
}

// ----- Part 2 code -----

fn get_card_value_p2(card: char) -> u8 {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 1,
        'T' => 10,
        _ => card.to_digit(10).unwrap() as u8,
    }
}

fn optimize_hand(cards: &[u8]) -> HandType {
    let wildcard_cnt = cards.iter().filter(|&&n| n == 1).count() as u8;
    if wildcard_cnt == 0 {
        // No optimization possible, return normal hand (as in p1)
        let card_cnt = {
            let mut t = cards.to_vec();
            t.sort();
            t.dedup();
            t.len() as u8
        };
        return construct_hand(cards, card_cnt);
    } else if wildcard_cnt == 5 {
        // All cards are wildcards, return 5 of a kind
        return HandType::FiveOfAKind(cards.to_vec());
    }

    // Count cards in hand (1-14)
    let mut counts: Vec<u8> = vec![0; 15];
    for c in cards.iter().filter(|c| **c != 1) {
        counts[*c as usize] += 1;
    }
    // Count non-joker card counts
    let mut counts = counts
        .iter()
        .filter(|&&n| n != 0)
        .cloned()
        .collect::<Vec<_>>();
    counts.sort();
    counts.reverse();

    // Check which hands are possible
    let cards = cards.to_vec();
    let max_single_cnt = counts[0] + wildcard_cnt;
    match max_single_cnt {
        5 => HandType::FiveOfAKind(cards),
        4 => HandType::FourOfAKind(cards),
        3 => {
            // Full house or 3 of a kind (depends on count of 2nd most common card)
            match counts[1] {
                2 => HandType::FullHouse(cards),
                _ => HandType::ThreeOfAKind(cards),
            }
        }
        2 => {
            // 2 pair or 1 pair
            match counts[1] {
                2 => HandType::TwoPair(cards),
                _ => HandType::OnePair(cards),
            }
        }
        _ => HandType::HighCard(cards),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let data = [
            "32T3K 765",
            "T55J5 684",
            "KK677 28",
            "KTJJT 220",
            "QQQJA 483",
        ]
        .map(String::from)
        .to_vec();

        assert_eq!(part1(data), 6440);
    }

    #[test]
    fn test_part2() {
        let data = [
            "32T3K 765",
            "T55J5 684",
            "KK677 28",
            "KTJJT 220",
            "QQQJA 483",
        ]
        .map(String::from)
        .to_vec();

        assert_eq!(part2(data), 5905);
    }
}

