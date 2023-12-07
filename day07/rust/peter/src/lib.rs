use input::*;
use std::fs::read_to_string;

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
        let map = cards.iter().fold([0; 15], |mut map, &card| {
            map[card as usize] += 1;
            map
        });

        let jokers = map[0];
        let (a, b) = map[2..]
            .iter()
            .fold((0, 0), |(a, b), &v| match (v > a, v > b) {
                (true, _) => (v, a),
                (_, true) => (a, v),
                _ => (a, b),
            });

        match (a + jokers, a + b + jokers) {
            (5, _) => Self::FiveOfAKind,
            (4, _) => Self::FourOfAKind,
            (_, 5) => Self::FullHouse,
            (3, _) => Self::ThreeOfAKind,
            (_, 4) => Self::TwoPair,
            (2, _) => Self::OnePair,
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
        .map(|(pos, (_, bid))| (pos + 1) * bid)
        .sum()
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
                "Hand got worse with jokers: {:?}",
                cards
            )
        }
    }
}
// end::tests[]
