use std::collections::HashSet;
use std::fs::read_to_string;

// tag::prelude[]
pub const IDENTIFIER: &str = "2023/04";

pub type InputType = String;
pub type SolT = usize;
// end::prelude[]

pub fn parse_input() -> String {
    read_to_string("../../../inputs/input04").unwrap()
}

// tag::star_1[]
pub fn count_winners(data: &str) -> impl Iterator<Item = SolT> + '_ {
    data.lines()
        .map(|line| line.split_once(':').unwrap().1.split_once(" | ").unwrap())
        .map(|(w, a)| {
            let values = w.split_ascii_whitespace().collect::<HashSet<_>>();
            a.split_ascii_whitespace()
                .filter(|value| values.contains(value))
                .count()
        })
}

pub fn star_1(data: &str) -> SolT {
    count_winners(data)
        .map(|c| match c {
            0 => 0,
            n => 1 << (n - 1),
        })
        .sum()
}
// end::star_1[]

// tag::star_2[]
pub fn star_2(data: &str) -> SolT {
    count_winners(data)
        .enumerate()
        .fold(vec![], |mut counts, (pos, wins)| {
            counts.resize(counts.len().max(pos + wins + 1), 1);
            (pos + 1..=pos + wins).fold(counts, |mut counts, pos_upd| {
                counts[pos_upd] += counts[pos];
                counts
            })
        })
        .iter()
        .sum()
}
// end::star_2[]

// tag::tests[]
#[cfg(test)]
mod tests {
    use super::*;

    const CONTENT: &str = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
"#;

    #[test]
    pub fn test_count_winners() {
        assert_eq!(
            vec![4, 2, 2, 1, 0, 0],
            count_winners(CONTENT).collect::<Vec<_>>()
        );
    }

    #[test]
    pub fn test_star_1() {
        assert_eq!(13, star_1(CONTENT));
    }

    #[test]
    pub fn test_star_2() {
        assert_eq!(30, star_2(CONTENT));
    }
}
// end::tests[]
