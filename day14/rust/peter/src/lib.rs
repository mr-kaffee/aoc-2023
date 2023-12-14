use input::*;
use std::fs::read_to_string;

// tag::prelude[]
pub const IDENTIFIER: &str = "2023/14";

pub type InputT<'a> = PuzzleData<'a>;

pub fn read_input() -> String {
    read_to_string("../../../inputs/input14").unwrap()
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
pub fn star_1(&PuzzleData(data, w, h): &PuzzleData) -> usize {
    (0..w)
        .map(|col| {
            (0..h).fold((0, 0), |(load, free), row| {
                match data[col + row * (w + 1)] {
                    b'#' => (load, row + 1),
                    b'O' => (load + h - free, free + 1),
                    _ => (load, free),
                }
            })
        })
        .fold(0, |sum, (load, _)| sum + load)
}
// end::star_1[]

// tag::star_2[]
pub fn tilt<F: Fn(usize, usize) -> usize>(data: &mut [u8], d1: usize, d2: usize, idx: F) -> usize {
    (0..d1)
        .map(|x| {
            (0..d2).fold((0, 0), |(load, free), y| match data[idx(x, y)] {
                b'#' => (load, y + 1),
                b'O' => {
                    data.swap(idx(x, free), idx(x, y));
                    (load + d2 - free, free + 1)
                }
                _ => (load, free),
            })
        })
        .fold(0, |sum, (load, _)| sum + load)
}

pub fn cycle(data: &mut [u8], w: usize, h: usize) -> [usize; 4] {
    [
        tilt(data, w, h, |col, row| col + row * (w + 1)),
        tilt(data, h, w, |row, col| col + row * (w + 1)),
        tilt(data, w, h, |col, row_inv| col + (h - row_inv - 1) * (w + 1)),
        tilt(data, h, w, |row, col_inv| (w - col_inv - 1) + row * (w + 1)),
    ]
}

pub fn star_2(&PuzzleData(data, w, h): &PuzzleData) -> usize {
    let mut data = data.to_owned();

    // cycle until repetition is found
    let r = (0..)
        .scan(Vec::new(), |list, n_1| {
            let loads = cycle(&mut data, w, h);
            let v = list
                .iter()
                .position(|prev| prev == &loads)
                .map(|n_0| ((1_000_000_000) - (n_1 + 1)) % (n_1 - n_0));
            list.push(loads);
            Some(v)
        })
        .flatten()
        .next()
        .unwrap();

    // execute residual cycles
    for _ in 0..r {
        cycle(&mut data, w, h);
    }

    // determine load on north (without tilting to north)
    (0..w)
        .map(|col| {
            (0..h)
                .filter(|&row| data[col + row * (w + 1)] == b'O')
                .map(|row| h - row)
                .sum::<usize>()
        })
        .sum()
}
// end::star_2[]

// tag::tests[]
#[cfg(test)]
mod tests {
    use super::*;

    const CONTENT: &str = r#"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
"#;

    const CONTENT_1: &str = r#".....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#....
"#;

    const CONTENT_2: &str = r#".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#..OO###..
#.OOO#...O
"#;

    const CONTENT_3: &str = r#".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#...O###.O
#.OOO#...O
"#;

    #[test]
    pub fn test_from() {
        let PuzzleData(_, w, h) = PuzzleData::from(CONTENT);
        assert_eq!(10, w);
        assert_eq!(10, h);
    }

    #[test]
    pub fn test_cycle() {
        let PuzzleData(data, w, h) = PuzzleData::from(CONTENT);
        let mut data = data.to_owned();
        cycle(&mut data, w, h);
        assert_eq!(String::from_utf8_lossy(&data), CONTENT_1);
        cycle(&mut data, w, h);
        assert_eq!(String::from_utf8_lossy(&data), CONTENT_2);
        cycle(&mut data, w, h);
        assert_eq!(String::from_utf8_lossy(&data), CONTENT_3);
    }

    #[test]
    pub fn test_star_1() {
        assert_eq!(136, star_1(&CONTENT.into()));
    }

    #[test]
    pub fn test_tilt_north() {
        let PuzzleData(data, w, h) = PuzzleData::from(CONTENT);
        let mut data = data.to_owned();
        assert_eq!(136, tilt(&mut data, w, h, |col, row| col + row * (w + 1)));
    }

    #[test]
    pub fn test_star_2() {
        assert_eq!(64, star_2(&CONTENT.into()));
    }
}
// end::tests[]
