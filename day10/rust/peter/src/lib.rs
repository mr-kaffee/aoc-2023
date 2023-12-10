use input::*;
use mr_kaffee_utils::grids::Grid;
use std::collections::{HashSet, VecDeque};
use std::fs::read_to_string;

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

    impl<T> From<&T> for PuzzleData
    where
        T: AsRef<[u8]> + ?Sized,
    {
        fn from(s: &T) -> Self {
            Self(s.make_grid(Some(b'.')))
        }
    }
}
// end::input[]

// tag::star_1[]
const TO_EAST: [u8; 3] = [b'-', b'J', b'7'];
const TO_NORTH: [u8; 3] = [b'|', b'F', b'7'];
const TO_WEST: [u8; 3] = [b'-', b'F', b'L'];
const TO_SOUTH: [u8; 3] = [b'|', b'L', b'J'];

pub fn find_loop(grid: &Grid, start: (usize, usize)) -> (HashSet<(usize, usize)>, SolT) {
    let mut pipe_loop = HashSet::from([start]);
    let mut queue = VecDeque::from([(start, 0)]);
    let mut mx = 0;
    while let Some(((col, row), dist)) = queue.pop_front() {
        mx = mx.max(dist);
        let cur = grid[(col, row)];
        if TO_EAST.contains(&grid[(col + 1, row)])
            && (cur == b'S' || TO_WEST.contains(&cur))
            && pipe_loop.insert((col + 1, row))
        {
            queue.push_back(((col + 1, row), dist + 1));
        }
        if TO_NORTH.contains(&grid[(col, row - 1)])
            && (cur == b'S' || TO_SOUTH.contains(&cur))
            && pipe_loop.insert((col, row - 1))
        {
            queue.push_back(((col, row - 1), dist + 1));
        }
        if TO_WEST.contains(&grid[(col - 1, row)])
            && (cur == b'S' || TO_EAST.contains(&cur))
            && pipe_loop.insert((col - 1, row))
        {
            queue.push_back(((col - 1, row), dist + 1));
        }
        if TO_SOUTH.contains(&grid[(col, row + 1)])
            && (cur == b'S' || TO_NORTH.contains(&cur))
            && pipe_loop.insert((col, row + 1))
        {
            queue.push_back(((col, row + 1), dist + 1));
        }
    }
    (pipe_loop, mx)
}

pub fn star_1(PuzzleData(grid): &PuzzleData) -> SolT {
    let (_, mx) = find_loop(
        grid,
        grid.to_2d(grid.data().iter().position(|&b| b == b'S').unwrap()),
    );
    mx
}
// end::star_1[]

// tag::star_2[]
pub fn star_2(PuzzleData(grid): &PuzzleData) -> SolT {
    let (col, row) = grid.to_2d(grid.data().iter().position(|&b| b == b'S').unwrap());
    let (pipe_loop, _) = find_loop(grid, (col, row));

    // determine type of pipe below 'S' (required for count of crossings)
    let start_pipe = match (
        TO_EAST.contains(&grid[(col + 1, row)]),
        TO_NORTH.contains(&grid[(col, row - 1)]),
        TO_WEST.contains(&grid[(col - 1, row)]),
        TO_SOUTH.contains(&grid[(col, row + 1)]),
    ) {
        (true, true, _, _) => b'L',
        (true, _, true, _) => b'-',
        (true, _, _, true) => b'F',
        (_, true, true, _) => b'J',
        (_, true, _, true) => b'|',
        (_, _, true, true) => b'7',
        _ => panic!(),
    };

    (0..grid.len())
        .map(|pos| grid.to_2d(pos))
        .filter(|pos| !pipe_loop.contains(pos))
        .filter(|&(col, row)| {
            // count crossings of pipe
            let (cnt, _) = (col + 1..grid.width())
                // skip elements not part of pipe
                .filter(|&col| pipe_loop.contains(&(col, row)))
                // map to elements (substituting 'S')
                .map(|col| match grid[(col, row)] {
                    b'S' => start_pipe,
                    b => b,
                })
                // skip purely tangent elements
                .filter(|&b| b != b'-')
                // count crossings
                .fold((0, b'.'), |(cnt, prev), cur| {
                    (
                        match (prev, cur) {
                            (_, b'|') => cnt + 1,    // a "|" is always a crossing
                            (b'L', b'7') => cnt + 1, // a "L7" is a crossing
                            (b'F', b'J') => cnt + 1, // a "FJ" is a crossing
                            _ => cnt, // anything else ("LJ", "F7", ...) is not a crossing
                        },
                        cur,
                    )
                });
            // odd crossing count is inside
            cnt & 1 == 1
        })
        .count()
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
