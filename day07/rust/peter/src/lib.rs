use input::*;
use std::{collections::HashMap, fs::read_to_string};

// tag::prelude[]
pub const IDENTIFIER: &str = "2023/07";

pub type SolT = usize;
// end::prelude[]

// tag::input[]
mod input {
    use crate::SolT;

    #[derive(Debug, PartialEq, Eq)]
    pub struct PuzzleData(pub Vec<([u8; 5], SolT)>);

    impl<T: AsRef<str>> From<T> for PuzzleData {
        fn from(value: T) -> Self {
            Self(
                value
                    .as_ref()
                    .lines()
                    .filter_map(|line| line.split_once(' '))
                    .map(|(cards, bid)| {
                        (
                            cards.bytes().take(5).enumerate().fold(
                                [0; 5],
                                |mut cards, (pos, card)| {
                                    cards[pos] = card;
                                    cards
                                },
                            ),
                            bid.parse().unwrap(),
                        )
                    })
                    .collect(),
            )
        }
    }
}
// end::input[]

pub fn parse_input() -> PuzzleData {
    read_to_string("../../../inputs/input07").unwrap().into()
}

// tag::star_1[]
pub fn map_cards(cards: &[u8; 5], joker: bool) -> [u8; 5] {
    cards.map(|b| match b {
        b'A' => 14,
        b'K' => 13,
        b'Q' => 12,
        b'J' if joker => 0,
        b'J' => 11,
        b'T' => 10,
        b => b - b'0',
    })
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl From<&[u8; 5]> for HandType {
    fn from(cards: &[u8; 5]) -> Self {
        let mut map = cards.iter().fold(HashMap::new(), |mut map, &card| {
            *map.entry(card).or_insert(0) += 1;
            map
        });
        let jokers = map.remove(&0).unwrap_or(0);
        let mut counts = map.into_values().collect::<Vec<_>>();
        counts.sort_unstable();
        match (counts.pop().unwrap_or(0), counts.pop().unwrap_or(0)) {
            (a, _) if a + jokers == 5 => Self::FiveOfAKind,
            (a, _) if a + jokers == 4 => Self::FourOfAKind,
            (a, b) if a + jokers >= 3 && b + jokers >= 2 && a + b + jokers == 5 => Self::FullHouse,
            (a, _) if a + jokers == 3 => Self::ThreeOfAKind,
            (a, b) if a + jokers >= 2 && b + jokers >= 2 && a + b + jokers == 4 => Self::TwoPair,
            (a, _) if a + jokers == 2 => Self::OnePair,
            _ => Self::HighCard,
        }
    }
}

pub fn star(PuzzleData(input): &PuzzleData, joker: bool) -> SolT {
    let mut hands = input
        .iter()
        .map(|(cards, bid)| (map_cards(cards, joker), *bid))
        .map(|(cards, bid)| ((HandType::from(&cards), cards), bid))
        .collect::<Vec<_>>();
    hands.sort_unstable();
    hands
        .iter()
        .enumerate()
        .fold(0, |score, (pos, (_, bid))| score + (pos + 1) * bid)
}

pub fn star_1(input: &PuzzleData) -> SolT {
    star(input, false)
}
// end::star_1[]

// tag::star_2[]
pub fn star_2(input: &PuzzleData) -> SolT {
    star(input, true)
}
// end::star_2[]

// tag::tests[]
#[cfg(test)]
mod tests {
    use super::*;

    const CONTENT: &str = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
"#;

    #[test]
    pub fn test_input_from() {
        let data = PuzzleData::from(CONTENT);
        assert_eq!(
            PuzzleData(vec![
                ([b'3', b'2', b'T', b'3', b'K'], 765),
                ([b'T', b'5', b'5', b'J', b'5'], 684),
                ([b'K', b'K', b'6', b'7', b'7'], 28),
                ([b'K', b'T', b'J', b'J', b'T'], 220),
                ([b'Q', b'Q', b'Q', b'J', b'A'], 483)
            ]),
            data
        );
        println!("{data:?}");
    }

    #[test]
    pub fn test_star_1() {
        assert_eq!(6440, star_1(&CONTENT.into()));
    }

    #[test]
    pub fn test_star_2() {
        assert_eq!(5905, star_2(&CONTENT.into()));
    }

    #[test]
    pub fn test_joker() {
        let PuzzleData(data) = parse_input();

        for (cards, _) in data.iter() {
            assert!(
                HandType::from(&map_cards(cards, true)) >= HandType::from(&map_cards(cards, false)),
                "Hand got worse with jokers: {:?}", cards
            )
        }
    }
}
// end::tests[]
