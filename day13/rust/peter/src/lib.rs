use input::*;
use std::fs::read_to_string;

// tag::prelude[]
pub const IDENTIFIER: &str = "2023/13";

pub type SolT = usize;
pub type InputT<'a> = PuzzleData<'a>;

pub fn read_input() -> String {
    read_to_string("../../../inputs/input13").unwrap()
}
// end::prelude[]

// tag::input[]
pub mod input {
    use crate::SolT;

    #[derive(Debug)]
    pub struct PuzzleData<'a>(pub Vec<(&'a [u8], SolT)>);

    impl<'a, T> From<&'a T> for PuzzleData<'a>
    where
        T: AsRef<str> + 'a + ?Sized,
    {
        fn from(s: &'a T) -> Self {
            Self(
                s.as_ref()
                    .split("\n\n")
                    .map(|pattern| {
                        (
                            pattern.as_bytes(),
                            pattern
                                .bytes()
                                .position(|b| b == b'\n')
                                .unwrap_or(pattern.len()),
                        )
                    })
                    .collect(),
            )
        }
    }
}
// end::input[]

// tag::star_1[]
pub enum ToIdx {
    ColRow(SolT),
    RowCol(SolT),
}

impl ToIdx {
    pub fn idx(&self, x: SolT, y: SolT) -> SolT {
        match self {
            Self::ColRow(w) => x + y * (w + 1),
            Self::RowCol(w) => y + x * (w + 1),
        }
    }
}

pub fn find_line(pattern: &[u8], d1: SolT, d2: SolT, idx: ToIdx) -> Option<SolT> {
    (0..d1 - 1).find(move |&line| {
        (0..d2).all(|y| {
            (0..=line)
                .rev()
                .zip(line + 1..d1)
                .all(|(x_a, x_b)| pattern[idx.idx(x_a, y)] == pattern[idx.idx(x_b, y)])
        })
    })
}

pub fn star<F>(data: &[(&[u8], SolT)], f: F) -> SolT
where
    F: Fn(&[u8], SolT, SolT, ToIdx) -> Option<SolT>,
{
    data.iter()
        .filter_map(|&(pattern, w)| {
            let h = (pattern.len() + 1) / (w + 1);
            f(pattern, w, h, ToIdx::ColRow(w))
                .map(|line| line + 1)
                .or_else(|| f(pattern, h, w, ToIdx::RowCol(w)).map(|line| (line + 1) * 100))
        })
        .sum()
}

pub fn star_1(PuzzleData(data): &PuzzleData) -> SolT {
    star(data, find_line)
}
// end::star_1[]

// tag::star_2[]
pub fn find_line_with_smudge(pattern: &[u8], d1: SolT, d2: SolT, idx: ToIdx) -> Option<SolT> {
    (0..d1 - 1).find(move |&line| {
        let mut sum = 0;
        for v in (0..d2).map(|y| {
            let mut sum = 0;
            for v in (0..=line)
                .rev()
                .zip(line + 1..d1)
                .map(|(x_a, x_b)| pattern[idx.idx(x_a, y)] == pattern[idx.idx(x_b, y)])
            {
                sum += if v { 0 } else { 1 };
                if sum > 1 {
                    break;
                }
            }
            sum
        }) {
            sum += v;
            if sum > 1 {
                break;
            }
        }
        sum == 1
    })
}

pub fn star_2(PuzzleData(data): &PuzzleData) -> SolT {
    star(data, find_line_with_smudge)
}
// end::star_2[]

// tag::tests[]
#[cfg(test)]
mod tests {
    use super::*;

    const CONTENT: &str = r#"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
"#;

    #[test]
    pub fn test_from() {
        let PuzzleData(data) = PuzzleData::from(CONTENT);
        assert_eq!(2, data.len());
        assert_eq!(9, data[0].1);
        assert_eq!(9, data[1].1);
        println!("{data:?}");
    }

    #[test]
    pub fn test_find_line_vertical() {
        let PuzzleData(data) = PuzzleData::from(CONTENT);
        assert_eq!(
            vec![Some(4), None],
            data.iter()
                .map(|&(pattern, w)| find_line(
                    pattern,
                    w,
                    (pattern.len() + 1) / (w + 1),
                    ToIdx::ColRow(w)
                ))
                .collect::<Vec<_>>()
        )
    }

    #[test]
    pub fn test_find_line_horizontal() {
        let PuzzleData(data) = PuzzleData::from(CONTENT);
        assert_eq!(
            vec![None, Some(3)],
            data.iter()
                .map(|&(pattern, w)| find_line(
                    pattern,
                    (pattern.len() + 1) / (w + 1),
                    w,
                    ToIdx::RowCol(w)
                ))
                .collect::<Vec<_>>()
        );
    }

    #[test]
    pub fn test_star_1() {
        assert_eq!(405, star_1(&CONTENT.into()));
    }

    #[test]
    pub fn test_star_2() {
        assert_eq!(400, star_2(&CONTENT.into()));
    }
}
// end::tests[]
