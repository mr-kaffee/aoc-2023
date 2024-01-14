use input::*;
use std::fs::read_to_string;
use std::iter::successors;

// tag::prelude[]
pub const IDENTIFIER: &str = "2023/09";

pub type SolT = i64;
pub type InputT = PuzzleData;

pub fn read_input() -> String {
    read_to_string("../../../inputs/input09").unwrap()
}
// end::prelude[]

// tag::input[]
pub mod input {
    use crate::SolT;

    #[derive(Debug, PartialEq, Eq)]
    pub struct PuzzleData(pub Vec<Vec<SolT>>);

    impl<T> From<T> for PuzzleData
    where
        T: AsRef<str>,
    {
        fn from(s: T) -> Self {
            Self(
                s.as_ref()
                    .lines()
                    .map(|line| {
                        line.split_ascii_whitespace()
                            .map(str::parse)
                            .collect::<Result<_, _>>()
                    })
                    .collect::<Result<_, _>>()
                    .unwrap(),
            )
        }
    }
}
// end::input[]

// tag::star_1[]
pub fn extrapolate_back(mut values: Vec<SolT>) -> SolT {
    successors(Some((0, values.len())), |&(result, len)| {
        if (0..len).all(|k| values[k] == 0) {
            None
        } else {
            let tail = values[len - 1];
            for k in 0..len - 1 {
                values[k] = values[k + 1] - values[k];
            }
            Some((result + tail, len - 1))
        }
    })
    .last()
    .map(|(result, _)| result)
    .unwrap_or(0)
}

pub fn star_1(PuzzleData(values): &PuzzleData) -> SolT {
    values.iter().cloned().map(extrapolate_back).sum()
}
// end::star_1[]

// tag::star_2[]
#[cfg(all(not(feature = "reverse"), not(feature = "no-sign")))]
pub fn extrapolate_front(mut values: Vec<SolT>) -> SolT {
    successors(Some((0, 1, values.len())), |&(result, sign, len)| {
        if len == 0 || (0..len).all(|k| values[k] == 0) {
            None
        } else {
            let head = values[0];
            for k in 0..len - 1 {
                values[k] = values[k + 1] - values[k];
            }
            Some((result + sign * head, -sign, len - 1))
        }
    })
    .last()
    .map(|(result, _, _)| result)
    .unwrap_or(0)
}

#[cfg(all(not(feature = "reverse"), feature = "no-sign"))]
/// variant that inverses the sign in the update steps to get back to
/// a direct sum; this is almost reverting the list of values
pub fn extrapolate_front(mut values: Vec<SolT>) -> SolT {
    successors(Some((0, values.len())), |&(result, len)| {
        if len == 0 || (0..len).all(|k| values[k] == 0) {
            None
        } else {
            let head = values[0];
            for k in 0..len - 1 {
                values[k] -= values[k + 1];
            }
            Some((result + head, len - 1))
        }
    })
    .last()
    .map(|(result, _)| result)
    .unwrap_or(0)
}

#[cfg(not(feature = "reverse"))]
pub fn star_2(PuzzleData(values): &PuzzleData) -> SolT {
    values.iter().cloned().map(extrapolate_front).sum()
}

#[cfg(feature = "reverse")]
/// variant that reverts the list of values and extrapolates at the back
pub fn star_2(PuzzleData(values): &PuzzleData) -> SolT {
    values
        .iter()
        .map(|values| values.iter().rev().copied().collect())
        .map(extrapolate_back)
        .sum()
}

// end::star_2[]

// tag::tests[]
#[cfg(test)]
mod tests {
    use super::*;

    const CONTENT: &str = r#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
"#;

    #[test]
    pub fn test_from() {
        let data = PuzzleData::from(CONTENT);
        println!("{data:?}");
        assert_eq!(
            PuzzleData(vec![
                vec![0, 3, 6, 9, 12, 15],
                vec![1, 3, 6, 10, 15, 21],
                vec![10, 13, 16, 21, 30, 45]
            ]),
            data
        );
    }

    #[test]
    pub fn test_star_1() {
        assert_eq!(114, star_1(&CONTENT.into()));
    }

    #[cfg(not(feature = "reverse"))]
    #[test]
    pub fn test_extrapolate_front() {
        let front = extrapolate_front(vec![10, 13, 16, 21, 30, 45]);
        assert_eq!(5, front);
    }

    #[test]
    pub fn test_star_2() {
        assert_eq!(2, star_2(&CONTENT.into()));
    }
}
// end::tests[]
