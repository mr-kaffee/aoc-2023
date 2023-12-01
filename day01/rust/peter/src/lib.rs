use std::fs::read_to_string;

// tag::prelude[]
pub const IDENTIFIER: &str = "2023/01";

pub type InputType = str;
pub type SolT = usize;
// end::prelude[]

// tag::parse[]
pub fn parse_input() -> String {
    read_to_string("../../../inputs/input01").unwrap()
}
// end::parse[]

// tag::star_1[]
pub fn map_1(line: &str) -> Option<SolT> {
    let b = line.as_bytes()[0];
    match b {
        b'0'..=b'9' => Some((b - b'0') as _),
        _ => None,
    }
}

fn score<F: Fn(&str) -> Option<SolT>>(l: &str, f: &F) -> SolT {
    (0..l.len()).map(|k| &l[k..]).find_map(f).unwrap() * 10
        + (0..l.len()).rev().map(|k| &l[k..]).find_map(f).unwrap()
}

pub fn star<F: Fn(&str) -> Option<SolT>>(data: &str, map: &F) -> SolT {
    data.lines().map(|line| score(line, map)).sum()
}
// end::star_1[]

// tag::star_2[]
const DIGITS: &[&str] = &[
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

pub fn map_2(line: &str) -> Option<SolT> {
    map_1(line).or_else(|| (0..DIGITS.len()).find(|&digit| line.starts_with(DIGITS[digit])))
}
// end::star_2[]

// tag::tests[]
#[cfg(test)]
mod tests {
    use super::*;

    const CONTENT_1: &str = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
"#;

    #[test]
    pub fn test_star_1() {
        assert_eq!(142, star(CONTENT_1, &map_1));
    }

    const CONTENT_2: &str = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
"#;

    #[test]
    pub fn test_star_2() {
        assert_eq!(281, star(CONTENT_2, &map_2));
    }
}
// end::tests[]
