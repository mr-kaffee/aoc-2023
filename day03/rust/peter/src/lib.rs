use input::PuzzleData;
use std::fs::read;
use std::{
    collections::HashMap,
    iter::{once, successors},
};

// tag::prelude[]
pub const IDENTIFIER: &str = "2023/03";

pub type InputType = PuzzleData;
pub type SolT = usize;
// end::prelude[]

// tag::input[]
pub mod input {
    #[derive(Debug)]
    pub struct PuzzleData {
        pub grid: Vec<u8>,
        pub w: usize,
    }

    impl From<Vec<u8>> for PuzzleData {
        fn from(grid: Vec<u8>) -> Self {
            let w = grid.iter().position(|&b| b == b'\n').unwrap();
            assert!(grid.len() % (w + 1) == 0, "Inconsistent lines!");
            Self { grid, w }
        }
    }
}

pub fn parse_input() -> PuzzleData {
    read("../../../inputs/input03").unwrap().into()
}
// end::input[]

// tag::star_1[]
pub fn star_1(PuzzleData { grid, w }: &PuzzleData) -> SolT {
    numbers(&grid)
        .filter(|(_, pos, len)| {
            run_around(*pos, *len, *w, grid.len() / (w + 1))
                .map(|pos| grid[pos])
                .any(|b: u8| b != b'.' && !(b'0'..=b'9').contains(&b))
        })
        .map(|(value, _, _)| value)
        .sum()
}
// end::star_1[]

// tag::star_2[]
pub fn star_2(PuzzleData { grid, w }: &PuzzleData) -> SolT {
    numbers(&grid)
        .fold(HashMap::new(), |map, (value, pos, len)| {
            run_around(pos, len, *w, grid.len() / (w + 1))
                .filter(|&pos| grid[pos] == b'*')
                .fold(map, |mut map, pos| {
                    map.entry(pos).or_insert(Vec::new()).push(value);
                    map
                })
        })
        .values()
        .filter(|values| values.len() == 2)
        .map(|values| values[0] * values[1])
        .sum()
}
// end::star_2[]

// tag::run_around[]
pub fn run_around(pos: usize, len: usize, w: usize, h: usize) -> impl Iterator<Item = usize> {
    let (col, row) = (pos % (w + 1), pos / (w + 1));

    (col.saturating_sub(1)..w.min(col + len + 1))
        .filter(move |_| row > 0)
        .map(move |col| col + (w + 1) * (row - 1))
        .chain(
            once(row)
                .filter(move |_| col + len < w)
                .map(move |row| col + len + (w + 1) * row),
        )
        .chain(
            (col.saturating_sub(1)..w.min(col + len + 1))
                .rev()
                .filter(move |_| row < h - 1)
                .map(move |col| col + (w + 1) * (row + 1)),
        )
        .chain(
            once(row)
                .filter(move |_| col > 0)
                .map(move |row| col - 1 + (w + 1) * row),
        )
}
// end::run_around[]

// tag::numbers[]
pub fn numbers(grid: &[u8]) -> impl Iterator<Item = (SolT, usize, usize)> + '_ {
    successors(next_number(grid, 0), |(_, pos, len)| {
        next_number(grid, pos + len)
    })
}

pub fn next_number(grid: &[u8], offset: usize) -> Option<(SolT, usize, usize)> {
    grid[offset..]
        .iter()
        .position(|b| (b'0'..=b'9').contains(b))
        .map(|pos| {
            grid[offset + pos..]
                .iter()
                .take_while(|b| (b'0'..=b'9').contains(b))
                .fold((0, offset + pos, 0), |(val, pos, len), &b| {
                    (10 * val + (b - b'0') as SolT, pos, len + 1)
                })
        })
}
// end::numbers[]

// tag::tests[]
#[cfg(test)]
mod tests {
    use super::*;

    const CONTENT: &str = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
"#;

    #[test]
    pub fn test_from() {
        let data = PuzzleData::from(CONTENT.as_bytes().to_vec());
        assert_eq!(10, data.w);
        assert_eq!(110, data.grid.len())
    }

    #[test]
    pub fn test_next_number() {
        let PuzzleData { grid, w: _w } = PuzzleData::from(CONTENT.as_bytes().to_vec());
        assert_eq!(Some((467, 0, 3)), next_number(&grid, 0));
        assert_eq!(Some((35, 24, 2)), next_number(&grid, 10));
    }

    #[test]
    pub fn test_star_1() {
        assert_eq!(4_361, star_1(&CONTENT.as_bytes().to_vec().into()));
    }

    #[test]
    pub fn test_star_2() {
        assert_eq!(467_835, star_2(&CONTENT.as_bytes().to_vec().into()));
    }
}
// end::tests[]
