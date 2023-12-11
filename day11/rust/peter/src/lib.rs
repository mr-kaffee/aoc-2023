use input::*;
use mr_kaffee_utils::grids::Grid;
use std::fs::read_to_string;

// tag::prelude[]
pub const IDENTIFIER: &str = "2023/11";

pub type SolT = usize;
pub type InputT = PuzzleData;

pub fn read_input() -> String {
    read_to_string("../../../inputs/input11").unwrap()
}
// end::prelude[]

// tag::input[]
pub mod input {
    use mr_kaffee_utils::grids::{Grid, MakeGrid};

    #[derive(Debug)]
    pub struct PuzzleData(pub Grid);

    impl<T> From<T> for PuzzleData
    where
        T: AsRef<[u8]>,
    {
        fn from(s: T) -> Self {
            Self(s.make_grid(None))
        }
    }
}
// end::input[]

// tag::star_1[]
pub fn sum_shortest_path(grid: &Grid, expansion: SolT) -> SolT {
    let empty_cols = (0..grid.width())
        .filter(|&col| (0..grid.height()).all(|row| grid[(col, row)] == b'.'))
        .collect::<Vec<_>>();
    let empty_rows = (0..grid.height())
        .filter(|&row| (0..grid.width()).all(|col| grid[(col, row)] == b'.'))
        .collect::<Vec<_>>();
    let galaxies = (0..grid.len())
        .filter(|&idx| grid[idx] == b'#')
        .map(|idx| grid.to_col_row(idx))
        .map(|(col, row)| {
            (
                col + empty_cols.iter().take_while(|&&e_col| e_col < col).count() * (expansion - 1),
                row + empty_rows.iter().take_while(|&&e_row| e_row < row).count() * (expansion - 1),
            )
        })
        .collect::<Vec<_>>();

    galaxies
        .iter()
        .enumerate()
        .flat_map(|(pos, &a)| galaxies[pos + 1..].iter().map(move |&b| (a, b)))
        .map(|((col_a, row_a), (col_b, row_b))| {
            col_b.max(col_a) - col_b.min(col_a) + row_b.max(row_a) - row_b.min(row_a)
        })
        .sum()
}

pub fn star_1(PuzzleData(grid): &PuzzleData) -> SolT {
    sum_shortest_path(grid, 2)
}
// end::star_1[]

// tag::star_2[]
pub fn star_2(PuzzleData(grid): &PuzzleData) -> SolT {
    sum_shortest_path(grid, 1_000_000)
}
// end::star_2[]

// tag::tests[]
#[cfg(test)]
mod tests {
    use super::*;

    const CONTENT: &str = r#"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
"#;

    #[test]
    pub fn test_from() {
        let data = PuzzleData::from(CONTENT);
        println!("{data:?}");
        assert_eq!(10, data.0.width());
        assert_eq!(10, data.0.height());
    }

    #[test]
    pub fn test_star_1() {
        assert_eq!(374, star_1(&CONTENT.into()));
    }

    #[test]
    pub fn test_star_2() {
        let PuzzleData(grid) = CONTENT.into();
        assert_eq!(1_030, sum_shortest_path(&grid, 10));
        assert_eq!(8_410, sum_shortest_path(&grid, 100));
    }
}
// end::tests[]
