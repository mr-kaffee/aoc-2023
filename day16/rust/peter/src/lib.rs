use input::*;
use std::fs::read_to_string;

// tag::prelude[]
pub const IDENTIFIER: &str = "2023/16";

pub type InputT<'a> = PuzzleData<'a>;

pub fn read_input() -> String {
    read_to_string("../../../inputs/input16").unwrap()
}
// end::prelude[]

// tag::input[]
pub mod input {
    #[derive(Debug)]
    pub struct PuzzleData<'a>(pub &'a [u8], pub usize, pub usize);

    impl<'a, T> From<&'a T> for PuzzleData<'a>
    where
        T: AsRef<[u8]> + 'a + ?Sized,
    {
        fn from(s: &'a T) -> Self {
            let data = s.as_ref();
            let w = data.iter().position(|&b| b == b'\n').unwrap_or(data.len());
            let h = (data.len() + 1) / (w + 1);
            Self(data, w, h)
        }
    }
}
// end::input[]

// tag::star_1[]
const EAST: u8 = 0;
const NORTH: u8 = 1;
const WEST: u8 = 2;
const SOUTH: u8 = 3;

const EAST_WEST: u8 = 0;
const NORTH_SOUTH: u8 = 1;

pub fn count_energized(
    data: &[u8],
    (w, h): (usize, usize),
    ((col, row), heading): ((usize, usize), u8),
) -> usize {
    let mut queue = Vec::from([((col, row), heading)]);
    let mut seen = vec![0u8; w * h];
    seen[col + row * w] |= 1 << heading;

    while let Some(((col, row), heading)) = queue.pop() {
        let deltas: &[u8] = match (data[col + row * (w + 1)], heading & 1) {
            (b'.', _) | (b'-', EAST_WEST) | (b'|', NORTH_SOUTH) => &[0], // pass-through
            (b'-', _) | (b'|', _) => &[1, 3],                            // split, turn left & right
            (b'/', EAST_WEST) | (b'\\', NORTH_SOUTH) => &[1],            // turn left
            (b'\\', EAST_WEST) | (b'/', NORTH_SOUTH) => &[3],            // turn right
            _ => panic!(),
        };

        for delta in deltas {
            let heading = (heading + delta) & 3;
            let (col, row) = match heading {
                EAST => (col + 1, row),
                NORTH => (col, row.wrapping_sub(1)),
                WEST => (col.wrapping_sub(1), row),
                SOUTH => (col, row + 1),
                _ => panic!(),
            };
            if col < w && row < h && (seen[col + row * w] & (1 << heading)) == 0 {
                seen[col + row * w] |= 1 << heading;
                queue.push(((col, row), heading));
            }
        }
    }

    seen.into_iter().filter(|&v| v > 0).count()
}

pub fn star_1(&PuzzleData(data, w, h): &PuzzleData) -> usize {
    count_energized(data, (w, h), ((0, 0), EAST))
}
// end::star_1[]

// tag::star_2[]
pub fn star_2(&PuzzleData(data, w, h): &PuzzleData) -> usize {
    (0..w)
        .map(|col| ((col, 0), SOUTH))
        .chain((0..h).map(|row| ((w - 1, row), WEST)))
        .chain((0..w).map(|col_inv| ((w - col_inv - 1, h - 1), NORTH)))
        .chain((0..h).map(|row_inv| ((0, h - row_inv - 1), EAST)))
        .map(|start| count_energized(data, (w, h), start))
        .max()
        .unwrap()
}
// end::star_2[]

// tag::tests[]
#[cfg(test)]
mod tests {
    use super::*;

    const CONTENT: &str = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....
"#;

    #[test]
    pub fn test_from() {
        let PuzzleData(data, w, h) = PuzzleData::from(CONTENT);
        assert_eq!(10, w);
        assert_eq!(10, h);
        assert_eq!(110, data.len());
    }

    #[test]
    pub fn test_star_1() {
        assert_eq!(46, star_1(&CONTENT.into()));
    }

    #[test]
    pub fn test_star_2() {
        assert_eq!(51, star_2(&CONTENT.into()));
    }
}
// end::tests[]
