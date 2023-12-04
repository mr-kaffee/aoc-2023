use input::PuzzleData;
use mr_kaffee_utils::grids::Grid;
use std::fs::read_to_string;
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
    use mr_kaffee_utils::grids::{Grid, MakeGrid};

    pub struct PuzzleData(pub Grid);

    impl<T: AsRef<[u8]>> From<T> for PuzzleData {
        fn from(value: T) -> Self {
            PuzzleData(value.make_grid(Some(b'.')))
        }
    }
}

pub fn parse_input() -> PuzzleData {
    read_to_string("../../../inputs/input03").unwrap().into()
}
// end::input[]

// tag::star_1[]
pub fn star_1(PuzzleData(grid): &PuzzleData) -> SolT {
    numbers(grid)
        .filter(|(_, pos, len)| {
            run_around(grid.to_2d(*pos), *len)
                .map(|pos| grid[pos])
                .any(|b: u8| b != b'.' && !(b'0'..=b'9').contains(&b))
        })
        .map(|(value, _, _)| value)
        .sum()
}
// end::star_1[]

// tag::star_2[]
pub fn star_2(PuzzleData(grid): &PuzzleData) -> SolT {
    numbers(grid)
        .fold(HashMap::new(), |map, (value, pos, len)| {
            run_around(grid.to_2d(pos), len)
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
pub fn run_around((col, row): (usize, usize), len: usize) -> impl Iterator<Item = (usize, usize)> {
    (col - 1..col + len + 1)
        .map(move |col| (col, row - 1))
        .chain(once((col + len, row)))
        .chain(
            (col - 1..col + len + 1)
                .rev()
                .map(move |col| (col, row + 1)),
        )
        .chain(once((col - 1, row)))
}
// end::run_around[]

// tag::numbers[]
pub fn numbers(grid: &Grid) -> impl Iterator<Item = (SolT, usize, usize)> + '_ {
    successors(next_number(grid, 0), |(_, pos, len)| {
        next_number(grid, pos + len)
    })
}

pub fn next_number(grid: &Grid, offset: usize) -> Option<(SolT, usize, usize)> {
    grid.data()[offset..]
        .iter()
        .position(|b| (b'0'..=b'9').contains(b))
        .map(|pos| {
            grid.data()[offset + pos..]
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

    const WIDTH_0: usize = 10;
    const HEIGHT_0: usize = 10;
    const WIDTH_EXT: usize = WIDTH_0 + 2;
    const HEIGHT_EXT: usize = HEIGHT_0 + 2;

    #[test]
    pub fn test_from() {
        let PuzzleData(grid) = PuzzleData::from(CONTENT);
        assert_eq!(WIDTH_EXT, grid.width());
        assert_eq!(HEIGHT_EXT, grid.height());
    }

    #[test]
    pub fn test_next_number() {
        let PuzzleData(grid) = PuzzleData::from(CONTENT);
        println!("{:?}", grid);
        assert_eq!(Some((467, WIDTH_EXT + 1, 3)), next_number(&grid, 0));
        assert_eq!(
            Some((35, 3 * WIDTH_EXT + 3, 2)),
            next_number(&grid, 2 * WIDTH_EXT)
        );
    }

    #[test]
    pub fn test_star_1() {
        assert_eq!(4_361, star_1(&CONTENT.into()));
    }

    #[test]
    pub fn test_star_2() {
        assert_eq!(467_835, star_2(&CONTENT.into()));
    }
}
// end::tests[]
