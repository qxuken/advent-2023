use once_cell::sync::Lazy;
use std::collections::HashMap;

const CARDS: [char; 13] = [
    'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
];

static CARDS_POWER: Lazy<HashMap<char, u8>> = Lazy::new(|| {
    let mut map = HashMap::with_capacity(CARDS.len());
    for (i, card) in CARDS.iter().enumerate() {
        map.insert(*card, i as u8);
    }
    map
});

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfKind,
    FullHouse,
    FourOfKind,
    FiveOfKind,
}

impl From<&[u8; 5]> for HandType {
    fn from(value: &[u8; 5]) -> Self {
        let mut map = value
            .iter()
            .fold(HashMap::<&u8, usize>::new(), |mut map, card| {
                map.entry(card).and_modify(|v| *v += 1).or_insert(1);
                map
            });
        if let Some((count, value)) = map.get(&0).copied().zip(
            map.iter_mut()
                .filter(|(&k, _v)| k != &0)
                .max_by_key(|(_k, &mut v)| v)
                .map(|(_k, v)| v),
        ) {
            *value += count;
            map.remove(&0);
        }
        let mut pairs = map.values().copied().collect::<Vec<usize>>();
        pairs.sort();
        match &pairs[..] {
            [5] => Self::FiveOfKind,
            [1, 4] => Self::FourOfKind,
            [2, 3] => Self::FullHouse,
            [1, 1, 3] => Self::ThreeOfKind,
            [1, 2, 2] => Self::TwoPair,
            [1, 1, 1, 2] => Self::OnePair,
            [1, 1, 1, 1, 1] => Self::HighCard,
            _ => panic!("Not expected combo"),
        }
    }
}

#[derive(Debug)]
struct Hand {
    hand: [u8; 5],
    hand_type: HandType,
    bet: usize,
}

impl From<(String, usize)> for Hand {
    fn from((hand, bet): (String, usize)) -> Self {
        let hand = hand
            .chars()
            .filter_map(|ch: char| CARDS_POWER.get(&ch))
            .copied()
            .collect::<Vec<u8>>();
        let hand: [u8; 5] = hand.try_into().expect("Hand of 5 cards");

        let hand_type = HandType::from(&hand);
        Hand {
            hand,
            hand_type,
            bet,
        }
    }
}

pub fn total_winnings(input: &str) -> usize {
    let mut hands = input
        .trim()
        .split('\n')
        .filter_map(|line| {
            let mut split = line.trim().split_ascii_whitespace();
            let hand = split.next().map(|s| s.to_owned());
            let bet = split.next().and_then(|n| n.parse().ok());
            hand.zip(bet)
        })
        .map(|t| t.into())
        .collect::<Vec<Hand>>();
    hands.sort_by_key(|h| (h.hand_type, h.hand));

    hands
        .iter()
        .map(|h| h.bet)
        .enumerate()
        .fold(0, |sum, (pow, bet)| sum + (bet * (pow + 1)))
}

fn main() {
    let input = include_str!("../input/day7_camel_poker.txt");

    let start = std::time::Instant::now();
    println!("{}", total_winnings(input));
    let duration = start.elapsed();

    println!("Time elapsed is: {:?}", duration);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total_winnings() {
        let input = r#"
            32T3K 765
            T55J5 684
            KK677 28
            KTJJT 220
            QQQJA 483
        "#;
        assert_eq!(total_winnings(input), 5905);
    }

    #[test]
    fn test_hand_type_parsing() {
        assert_eq!(HandType::from(&[1, 2, 3, 4, 5]), HandType::HighCard);
        assert_eq!(HandType::from(&[1, 1, 3, 4, 5]), HandType::OnePair);
        assert_eq!(HandType::from(&[1, 1, 3, 3, 5]), HandType::TwoPair);
        assert_eq!(HandType::from(&[1, 1, 1, 4, 5]), HandType::ThreeOfKind);
        assert_eq!(HandType::from(&[1, 1, 1, 4, 4]), HandType::FullHouse);
        assert_eq!(HandType::from(&[1, 1, 1, 1, 4]), HandType::FourOfKind);
        assert_eq!(HandType::from(&[1, 1, 1, 1, 1]), HandType::FiveOfKind);
    }

    #[test]
    fn test_hand_type_parsing_with_joker() {
        assert_eq!(HandType::from(&[0, 1, 2, 3, 4]), HandType::OnePair);
        assert_eq!(HandType::from(&[0, 1, 2, 2, 3]), HandType::ThreeOfKind);
        assert_eq!(HandType::from(&[0, 1, 1, 1, 5]), HandType::FourOfKind);
    }
}
