use input::*;
use mr_kaffee_utils::grids::Grid;
use std::fs::read_to_string;
use std::iter::successors;

// tag::prelude[]
pub const IDENTIFIER: &str = "2023/10";

pub type SolT = usize;
pub type InputT = PuzzleData;

pub fn read_input() -> String {
    read_to_string("../../../inputs/input10").unwrap()
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
            Self(s.make_grid(Some(b'.')))
        }
    }
}
// end::input[]

// tag::star_1[]
fn deduce_pipe(grid: &Grid, (col, row): (usize, usize)) -> u8 {
    const TO_EAST: [u8; 3] = [b'-', b'J', b'7'];
    const TO_NORTH: [u8; 3] = [b'|', b'F', b'7'];
    const TO_WEST: [u8; 3] = [b'-', b'F', b'L'];
    const TO_SOUTH: [u8; 3] = [b'|', b'L', b'J'];

    match (
        TO_EAST.contains(&grid[(col + 1, row)]),
        TO_NORTH.contains(&grid[(col, row - 1)]),
        TO_WEST.contains(&grid[(col - 1, row)]),
        TO_SOUTH.contains(&grid[(col, row + 1)]),
    ) {
        (true, true, false, false) => b'L',
        (true, false, true, false) => b'-',
        (true, false, false, true) => b'F',
        (false, true, true, false) => b'J',
        (false, true, false, true) => b'|',
        (false, false, true, true) => b'7',
        _ => panic!(),
    }
}

pub fn find_loop(grid: &Grid) -> (SolT, Vec<bool>, u8) {
    let start = grid.data().iter().position(|&b| b == b'S').unwrap();
    let start_pipe = deduce_pipe(grid, grid.to_col_row(start));

    let (len, pipe_loop) = successors(Some((0, start)), |&(prev_idx, cur_idx)| {
        let cur = match grid[cur_idx] {
            b'S' => start_pipe,
            b => b,
        };

        let (idx_a, idx_b) = match cur {
            b'-' => (cur_idx - 1, cur_idx + 1),
            b'|' => (cur_idx - grid.width(), cur_idx + grid.width()),
            b'J' => (cur_idx - 1, cur_idx - grid.width()),
            b'7' => (cur_idx - 1, cur_idx + grid.width()),
            b'F' => (cur_idx + 1, cur_idx + grid.width()),
            b'L' => (cur_idx + 1, cur_idx - grid.width()),
            _ => panic!(),
        };

        if prev_idx != idx_a && start != idx_a {
            Some((cur_idx, idx_a))
        } else if prev_idx != idx_b && start != idx_b {
            Some((cur_idx, idx_b))
        } else {
            None
        }
    })
    .fold(
        (0, vec![false; grid.len()]),
        |(len, mut pipe_loop), (_, idx)| {
            pipe_loop[idx] = true;
            (len + 1, pipe_loop)
        },
    );

    (len >> 1, pipe_loop, start_pipe)
}

pub fn star_1(PuzzleData(grid): &PuzzleData) -> SolT {
    let (mx, _, _) = find_loop(grid);
    mx
}
// end::star_1[]

// tag::star_2[]
#[cfg(feature = "point-by-point")]
pub fn star_2(PuzzleData(grid): &PuzzleData) -> SolT {
    let (_, pipe_loop, start_pipe) = find_loop(grid);

    (0..grid.len())
        .map(|pos| grid.to_col_row(pos))
        .filter(|&(col, row)| !pipe_loop[col + grid.width() * row])
        .filter(|&(col, row)| {
            // count crossings of pipe
            let (cnt, _) = (col + 1..grid.width())
                // skip elements not part of pipe
                .filter(|&col| pipe_loop[col + grid.width() * row])
                // map to elements (substituting 'S')
                .map(|col| match grid[(col, row)] {
                    b'S' => start_pipe,
                    b => b,
                })
                // skip purely tangent elements
                .filter(|&b| b != b'-')
                // count crossings
                .fold((0, b'.'), |(cnt, prev), cur| match (prev, cur) {
                    // "|", "L7", and "FJ" are crossings
                    (_, b'|') | (b'L', b'7') | (b'F', b'J') => (cnt + 1, cur),
                    _ => (cnt, cur), // anything else ("LJ", "F7", ...) is not a crossing
                });
            // odd crossing count is inside
            cnt & 1 == 1
        })
        .count()
}

#[cfg(not(feature = "point-by-point"))]
pub fn star_2(PuzzleData(grid): &PuzzleData) -> SolT {
    let (_, pipe_loop, start_pipe) = find_loop(grid);

    (0..grid.height())
        .map(|row| {
            let (cnt, _, _) = (0..grid.width())
                .map(
                    |col| match (pipe_loop[col + grid.width() * row], grid[(col, row)]) {
                        (true, b'S') => start_pipe,
                        (true, b) => b,
                        _ => b'.',
                    },
                )
                .filter(|&b| b != b'-')
                .fold((0, b'.', false), |(cnt, prev, inside), cur| {
                    match (inside, prev, cur) {
                        (true, _, b'.') => (cnt + 1, cur, true),
                        (_, _, b'|') | (_, b'F', b'J') | (_, b'L', b'7') => (cnt, cur, !inside),
                        _ => (cnt, cur, inside),
                    }
                });
            cnt
        })
        .sum()
}
// end::star_2[]

// tag::tests[]
#[cfg(test)]
mod tests {
    use super::*;

    const CONTENT: &str = r#"7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ
"#;
    const EXP_STAR_1: SolT = 8;

    #[test]
    pub fn test_from() {
        let data = PuzzleData::from(CONTENT);
        assert_eq!(7, data.0.width());
        assert_eq!(7, data.0.height());
    }

    #[test]
    pub fn test_star_1() {
        assert_eq!(EXP_STAR_1, star_1(&CONTENT.into()));
    }

    const CONTENT_2: &str = r#"FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
"#;
    const EXP_2_STAR_2: SolT = 10;

    const CONTENT_3: &str = r#"...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........
"#;
    const EXP_3_STAR_2: SolT = 4;

    const CONTENT_4: &str = r#".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...
"#;
    const EXP_4_STAR_2: SolT = 8;

    #[test]
    pub fn test_star_2() {
        assert_eq!(EXP_2_STAR_2, star_2(&CONTENT_2.into()));
        assert_eq!(EXP_3_STAR_2, star_2(&CONTENT_3.into()));
        assert_eq!(EXP_4_STAR_2, star_2(&CONTENT_4.into()));
    }
}
// end::tests[]
