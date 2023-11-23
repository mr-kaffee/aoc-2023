//! Module to handle 5 x 6 'pixels' AoC letters
//!
//! From time to time, AoC puzzles involve pixel displays with letters. Typically '#' represents a lit
//! pixel and '.' represents a dark pixel.
//!
//! This module helps to parse those letters.
//!
//! The alphabet is still quite incomplete. It contains all the different letters that I have seen in my
//! solutions to AoC 2015 to 2022 so far.
use std::any::Any;

pub const LIT: char = '#';
pub const DARK: char = '.';

pub const A: &[u8] = &[];
pub const B: &[u8] = "###..#..#.###..#..#.#..#.###..".as_bytes();
pub const C: &[u8] = ".##..#..#.#....#....#..#..##..".as_bytes();
pub const D: &[u8] = &[];
pub const E: &[u8] = "####.#....###..#....#....####.".as_bytes();
pub const F: &[u8] = "####.#....###..#....#....#....".as_bytes();
pub const G: &[u8] = ".##..#..#.#....#.##.#..#..###.".as_bytes();
pub const H: &[u8] = "#..#.#..#.####.#..#.#..#.#..#.".as_bytes();
pub const I: &[u8] = &[];
pub const J: &[u8] = "..##....#....#....#.#..#..##..".as_bytes();
pub const K: &[u8] = "#..#.#.#..##...#.#..#.#..#..#.".as_bytes();
pub const L: &[u8] = "#....#....#....#....#....####.".as_bytes();
pub const M: &[u8] = &[];
pub const N: &[u8] = &[];
pub const O: &[u8] = &[];
pub const P: &[u8] = "###..#..#.#..#.###..#....#....".as_bytes();
pub const Q: &[u8] = &[];
pub const R: &[u8] = "###..#..#.#..#.###..#.#..#..#.".as_bytes();
pub const S: &[u8] = &[];
pub const T: &[u8] = &[];
pub const U: &[u8] = &[];
pub const V: &[u8] = &[];
pub const W: &[u8] = &[];
pub const X: &[u8] = &[];
pub const Y: &[u8] = "#...##...#.#.#...#....#....#..".as_bytes();
pub const Z: &[u8] = "####....#...#...#...#....####.".as_bytes();

pub const ALPHABET: [&[u8]; 26] = [
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z,
];

pub const WIDTH: usize = 5;
pub const HEIGHT: usize = 6;

pub trait Letters {
    /// Decode a pixel display into letters
    ///
    /// The `nl` parameter specifies how many extra pixels are added at the
    /// end of each line. This is typically `0`, or `1` if `\n` 'pixels' are included
    ///
    /// Each letter is supposed to be 5 pixels wide and 6 pixel high
    fn decode(&self, nl: usize) -> Result<String, String>;

    /// Create a printable string representing the display using the given
    /// chars for lit and dark pixels
    fn printable(&self, nl: usize, lit: char, off: char) -> String;

    /// Create a printable string representing the display using
    /// unicode 0x2588 (full block) for lit pixels and '.' for dark pixels
    fn printable_default(&self, nl: usize) -> String {
        self.printable(nl, '\u{2588}', '.')
    }
}

impl<T> Letters for T
where
    T: AsRef<[u8]>,
{
    fn decode(&self, nl: usize) -> Result<String, String> {
        let s = self.as_ref();
        let (n, w) = dimensions(s, nl, WIDTH, HEIGHT, 0);

        (0..n)
            .map(|k| {
                ALPHABET
                    .iter()
                    .enumerate()
                    .find(|(_, letter)| is_match(s, k, w, letter, WIDTH, HEIGHT, 0))
                    .map(|(idx, _)| (b'A' + idx as u8) as char)
                    .ok_or_else(|| {
                        format!(
                            "Could not parse {}th letter of\n{}",
                            k + 1,
                            s.printable_default(nl)
                        )
                    })
            })
            .collect()
    }

    fn printable(&self, nl: usize, lit: char, dark: char) -> String {
        let s = self.as_ref();
        let (n, w) = dimensions(s, nl, WIDTH, HEIGHT, 0);

        let mut display = String::with_capacity(n * (w + nl));

        for row in 0..HEIGHT {
            for col in 0..n * WIDTH {
                display.push(match s[col + w * row] as char {
                    LIT => lit,
                    DARK => dark,
                    v => v,
                });
            }
            display.push('\n');
        }

        display
    }
}

impl Letters for [char] {
    fn decode(&self, nl: usize) -> Result<String, String> {
        self.iter().map(|&c| c as u8).collect::<Vec<_>>().decode(nl)
    }

    fn printable(&self, nl: usize, lit: char, off: char) -> String {
        self.iter()
            .map(|&c| c as u8)
            .collect::<Vec<_>>()
            .printable(nl, lit, off)
    }
}

pub mod big {
    use super::{dimensions, is_match, Letters};

    pub const A: &[u8] = &[];
    pub const B: &[u8] = &[];
    pub const C: &[u8] = ".####.#....##.....#.....#.....#.....#.....#.....#....#.####.".as_bytes();
    pub const D: &[u8] = &[];
    pub const E: &[u8] = "#######.....#.....#.....#####.#.....#.....#.....#.....######".as_bytes();
    pub const F: &[u8] = &[];
    pub const G: &[u8] = ".####.#....##.....#.....#.....#..####....##....##...##.###.#".as_bytes();
    pub const H: &[u8] = "#....##....##....##....########....##....##....##....##....#".as_bytes();
    pub const I: &[u8] = "###.#..#..#..#..#..#.###".as_bytes();
    pub const J: &[u8] = &[];
    pub const K: &[u8] = &[];
    pub const L: &[u8] = "#.....#.....#.....#.....#.....#.....#.....#.....#.....######".as_bytes();
    pub const M: &[u8] = &[];
    pub const N: &[u8] = &[];
    pub const O: &[u8] = &[];
    pub const P: &[u8] = &[];
    pub const Q: &[u8] = &[];
    pub const R: &[u8] = &[];
    pub const S: &[u8] = &[];
    pub const T: &[u8] = &[];
    pub const U: &[u8] = &[];
    pub const V: &[u8] = &[];
    pub const W: &[u8] = &[];
    pub const X: &[u8] = &[];
    pub const Y: &[u8] = &[];
    pub const Z: &[u8] = "######.....#.....#....#....#....#....#....#.....#.....######".as_bytes();

    pub const ALPHABET: [&[u8]; 26] = [
        A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z,
    ];

    pub const WIDTH: usize = 6;
    pub const HEIGHT: usize = 10;
    pub const SPACE: usize = 2;

    pub trait BigLetters {
        /// Decode a pixel display into letters
        ///
        /// The `nl` parameter specifies how many extra pixels are added at the
        /// end of each line. This is typically `0`, or `1` if `\n` 'pixels' are included
        ///
        /// Each letter is supposed to be 5 pixels wide and 6 pixel high
        fn decode_big(&self, nl: usize) -> Result<String, String>;
    }

    impl<T> BigLetters for T
    where
        T: AsRef<[u8]>,
    {
        fn decode_big(&self, nl: usize) -> Result<String, String> {
            let s = self.as_ref();
            let (n, w) = dimensions(s, nl, WIDTH, HEIGHT, SPACE);

            (0..n)
                .map(|k| {
                    ALPHABET
                        .iter()
                        .enumerate()
                        .find(|(_, letter)| is_match(s, k, w, letter, WIDTH, HEIGHT, SPACE))
                        .map(|(idx, _)| (b'A' + idx as u8) as char)
                        .ok_or_else(|| {
                            format!(
                                "Could not parse {}th letter of\n{}",
                                k + 1,
                                s.printable_default(nl)
                            )
                        })
                })
                .collect()
        }
    }

    impl BigLetters for [char] {
        fn decode_big(&self, nl: usize) -> Result<String, String> {
            self.iter()
                .map(|&c| c as u8)
                .collect::<Vec<_>>()
                .decode_big(nl)
        }
    }
}

fn is_match(
    s: &[u8],
    k: usize,
    w: usize,
    letter: &[u8],
    width: usize,
    height: usize,
    space: usize,
) -> bool {
    if letter.len() < width * height {
        // letter not implemented
        return false;
    }

    for row in 0..height {
        for col in 0..width {
            if s[(width + space) * k + col + w * row] != letter[col + width * row] {
                return false;
            }
        }
    }

    true
}

fn dimensions<T: Any>(
    s: &[T],
    nl: usize,
    width: usize,
    height: usize,
    space: usize,
) -> (usize, usize) {
    let n = (s.len() + nl - height * nl + space * height) / ((width + space) * height);
    let w = n * width + (n - 1) * space + nl;
    assert!(
        s.len() >= height * w - nl && s.len() <= height * w,
        "Expected {} <= len = {} <= {}",
        height * w - nl,
        s.len(),
        height * w
    );
    (n, w)
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "#..#.####.#..#.#..#.####.####...##.####.
#..#....#.#.#..#..#.#....#.......#....#.
####...#..##...####.###..###.....#...#..
#..#..#...#.#..#..#.#....#.......#..#...
#..#.#....#.#..#..#.#....#....#..#.#....
#..#.####.#..#.#..#.#....####..##..####.";

    #[test]
    pub fn test_is_match() {
        assert!(is_match(
            DATA.as_bytes(),
            0,
            8 * WIDTH + 1,
            H,
            WIDTH,
            HEIGHT,
            0
        ));
        assert!(!is_match(
            DATA.as_bytes(),
            0,
            8 * WIDTH + 1,
            J,
            WIDTH,
            HEIGHT,
            0
        ));
    }

    #[test]
    pub fn test_decode() {
        let text = DATA.decode(1);
        match text {
            Ok(text) => assert_eq!("HZKHFEJZ", text),
            Err(err) => {
                println!("{err}");
                panic!()
            }
        }
    }
}
