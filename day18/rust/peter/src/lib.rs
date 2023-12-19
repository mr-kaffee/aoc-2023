use std::fs::read_to_string;

// tag::prelude[]
pub const IDENTIFIER: &str = "2023/18";

pub type SolT = isize;
pub type InputT<'a> = &'a str;

pub fn read_input() -> String {
    read_to_string("../../../inputs/input18").unwrap()
}
// end::prelude[]

// tag::star_1[]
pub fn shoelace<F>(data: F) -> SolT
where
    F: Iterator<Item = (u8, SolT)>,
{
    let (_, s, p) = data.fold(((0, 0), 0, 0), |((x0, y0), s, p), (d, len)| {
        let (x1, y1) = match d {
            0 => (x0 + len, y0),
            1 => (x0, y0 + len),
            2 => (x0 - len, y0),
            3 => (x0, y0 - len),
            _ => unreachable!(),
        };
        (
            (x1, y1),
            s + (y0 + y1) * (x0 - x1),
            p + x0.max(x1) - x0.min(x1) + y0.max(y1) - y0.min(y1),
        )
    });
    s / 2 + p / 2 + 1
}

pub fn star_1(data: &&str) -> SolT {
    shoelace(data.lines().map(|line| {
        let mut parts = line.split_ascii_whitespace();
        (
            match parts.next().unwrap() {
                "R" => 0,
                "D" => 1,
                "L" => 2,
                "U" => 3,
                d => panic!("Illegal direction: {}", d),
            },
            parts.next().unwrap().parse().unwrap(),
        )
    }))
}
// end::star_1[]

// tag::star_2[]
pub fn star_2(data: &&str) -> SolT {
    shoelace(data.lines().map(|line| {
        line.split_ascii_whitespace()
            .nth(2)
            .and_then(|code| SolT::from_str_radix(&code[2..code.len() - 1], 16).ok())
            .map(|code| ((code & 0xf) as _, code >> 4))
            .unwrap()
    }))
}
// end::star_2[]

// tag::tests[]
#[cfg(test)]
mod tests {
    use super::*;

    const CONTENT: &str = r#"R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)
"#;

    #[test]
    pub fn test_star_1() {
        assert_eq!(62, star_1(&CONTENT));
    }

    #[test]
    pub fn test_star_2() {
        assert_eq!(952_408_144_115, star_2(&CONTENT));
    }
}
// end::tests[]
