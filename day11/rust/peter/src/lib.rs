use input::*;
use std::fs::read_to_string;

// tag::prelude[]
pub const IDENTIFIER: &str = "2023/11";

pub type SolT = isize;
pub type InputT = PuzzleData;

pub fn read_input() -> String {
    read_to_string("../../../inputs/input11").unwrap()
}
// end::prelude[]

// tag::input[]
pub mod input {
    #[derive(Debug, PartialEq, Eq, Default)]
    pub struct PuzzleData {
        pub galaxies: Vec<(usize, usize)>,
        pub count_in_cols: Vec<usize>,
        pub count_in_rows: Vec<usize>,
    }

    impl<T> From<T> for PuzzleData
    where
        T: AsRef<str>,
    {
        fn from(s: T) -> Self {
            s.as_ref()
                .lines()
                .enumerate()
                .fold(PuzzleData::default(), |mut data, (row, line)| {
                    data.count_in_rows.push(0);
                    line.bytes().enumerate().filter(|(_, b)| b == &b'#').fold(
                        data,
                        |mut data, (col, _)| {
                            data.galaxies.push((col, row));
                            data.count_in_cols
                                .resize(data.count_in_cols.len().max(col + 1), 0);
                            data.count_in_cols[col] += 1;
                            data.count_in_rows[row] += 1;
                            data
                        },
                    )
                })
        }
    }
}
// end::input[]

// tag::star_1[]
fn calc_offsets(counts: &[usize], expansion: SolT) -> Vec<SolT> {
    counts
        .iter()
        .scan(0, |cum_sum, &cnt| {
            let val = Some(*cum_sum);
            *cum_sum += if cnt > 0 { 1 } else { expansion };
            val
        })
        .collect()
}

pub fn sum_shortest_path(data: &PuzzleData, expansion: SolT) -> SolT {
    let col_offsets = calc_offsets(&data.count_in_cols, expansion);
    let row_offsets = calc_offsets(&data.count_in_rows, expansion);
    data.galaxies
        .iter()
        .map(|&(col_a, row_a)| (col_offsets[col_a], row_offsets[row_a]))
        .enumerate()
        .map(|(pos, (col_a, row_a))| {
            data.galaxies[pos + 1..]
                .iter()
                .map(|&(col_a, row_a)| (col_offsets[col_a], row_offsets[row_a]))
                .map(|(col_b, row_b)| {
                    col_b.max(col_a) - col_b.min(col_a) + row_b.max(row_a) - row_b.min(row_a)
                })
                .sum::<SolT>()
        })
        .sum()
}

pub fn star_1(data: &PuzzleData) -> SolT {
    sum_shortest_path(data, 2)
}
// end::star_1[]

// tag::star_2[]
pub fn star_2(data: &PuzzleData) -> SolT {
    sum_shortest_path(data, 1_000_000)
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
        assert_eq!(
            PuzzleData {
                galaxies: vec![
                    (3, 0),
                    (7, 1),
                    (0, 2),
                    (6, 4),
                    (1, 5),
                    (9, 6),
                    (7, 8),
                    (0, 9),
                    (4, 9)
                ],
                count_in_cols: vec![2, 1, 0, 1, 1, 0, 1, 2, 0, 1],
                count_in_rows: vec![1, 1, 1, 0, 1, 1, 1, 0, 1, 2],
            },
            data
        );
    }

    #[test]
    pub fn test_star_1() {
        assert_eq!(374, star_1(&CONTENT.into()));
    }

    #[test]
    pub fn test_star_2() {
        assert_eq!(1_030, sum_shortest_path(&CONTENT.into(), 10));
        assert_eq!(8_410, sum_shortest_path(&CONTENT.into(), 100));
    }
}
// end::tests[]
