use input::PuzzleData;
use std::fs::read_to_string;

// tag::prelude[]
pub const IDENTIFIER: &str = "2023/02";

pub type InputType = PuzzleData;
pub type SolT = usize;
// end::prelude[]

// tag::input[]
pub mod input {
    use crate::SolT;

    #[derive(Debug)]
    pub struct PuzzleData(pub Vec<Vec<[SolT; 3]>>);

    fn parse_draw(line: &str) -> [SolT; 3] {
        line.split(", ").fold([0; 3], |[r, g, b], color| {
            match color
                .split_once(' ')
                .map(|(n, c)| (n.parse::<SolT>().unwrap(), c))
            {
                Some((n, "red")) => [r + n, g, b],
                Some((n, "green")) => [r, g + n, b],
                Some((n, "blue")) => [r, g, b + n],
                _ => panic!("Unexpected draw."),
            }
        })
    }

    fn parse_game(line: &str) -> Vec<[SolT; 3]> {
        line.split_once(": ")
            .unwrap()
            .1
            .split("; ")
            .map(parse_draw)
            .collect()
    }

    impl<T: AsRef<str>> From<T> for PuzzleData {
        fn from(s: T) -> Self {
            PuzzleData(s.as_ref().lines().map(parse_game).collect())
        }
    }
}
// end::input[]

// tag::parse[]
pub fn parse_input() -> InputType {
    read_to_string("../../../inputs/input02").unwrap().into()
}
// end::parse[]

// tag::star_1[]
pub fn star_1(PuzzleData(games): &PuzzleData) -> SolT {
    games
        .iter()
        .enumerate()
        .filter(|(_, game)| {
            game.iter()
                .all(|[r, g, b]| *r <= 12 && *g <= 13 && *b <= 14)
        })
        .map(|(pos, _)| pos + 1)
        .sum()
}
// end::star_1[]

// tag::star_2[]
pub fn star_2(PuzzleData(games): &PuzzleData) -> SolT {
    games
        .iter()
        .map(|game| {
            game.iter()
                .fold([0; 3], |[r_max, g_max, b_max], [r, g, b]| {
                    [r_max.max(*r), g_max.max(*g), b_max.max(*b)]
                })
        })
        .map(|[r, g, b]| r * g * b)
        .sum()
}
// end::star_2[]

// tag::tests[]
#[cfg(test)]
mod tests {
    use super::*;

    const CONTENT: &str = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
"#;

    #[test]
    pub fn test_from() {
        let expected: Vec<Vec<[SolT; 3]>> = vec![
            vec![[4, 0, 3], [1, 2, 6], [0, 2, 0]],
            vec![[0, 2, 1], [1, 3, 4], [0, 1, 1]],
            vec![[20, 8, 6], [4, 13, 5], [1, 5, 0]],
            vec![[3, 1, 6], [6, 3, 0], [14, 3, 15]],
            vec![[6, 3, 1], [1, 2, 2]],
        ];
        let PuzzleData(games) = PuzzleData::from(CONTENT);
        assert_eq!(expected, games);
    }

    #[test]
    pub fn test_star_1() {
        assert_eq!(8, star_1(&CONTENT.into()));
    }

    #[test]
    pub fn test_star_2() {
        assert_eq!(2286, star_2(&CONTENT.into()));
    }
}
// end::tests[]
