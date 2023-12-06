use std::{fs::read_to_string, iter::successors};

// tag::prelude[]
pub const IDENTIFIER: &str = "2023/06";

pub type SolT = i64;
// end::prelude[]

pub fn parse_input() -> String {
    read_to_string("../../../inputs/input06").unwrap()
}

// tag::star_1[]
pub fn play_naive((time, dist): (SolT, SolT)) -> SolT {
    (0..=time).filter(|&b| (time - b) * b > dist).count() as _
}

pub fn star_1(s: &str) -> SolT {
    // time holding button: b
    // total time: T
    // distance: (T - b) * b
    let mut lines = s.lines().map(|line| {
        line.split_ascii_whitespace()
            .skip(1)
            .map(|value| value.parse::<SolT>().unwrap())
    });
    lines
        .next()
        .unwrap()
        .zip(lines.next().unwrap())
        .map(play_naive)
        .product()
}
// end::star_1[]

// tag::star_2[]
pub fn bisect<F: Fn(SolT) -> bool>(bs: (SolT, SolT), test: F) -> (SolT, SolT) {
    successors(Some(bs), |&(b_l, b_r)| {
        let b = (b_l + b_r) / 2;
        if test(b) {
            Some((b_l, b))
        } else {
            Some((b, b_r))
        }
    })
    .skip_while(|(b_l, b_r)| b_r - b_l > 1)
    .next()
    .unwrap()
}

pub fn play_smart((time, dist): (SolT, SolT)) -> SolT {
    // distance: (T - b) * b
    // optimum: T - 2 b = 0
    let b_opt = time / 2;
    let (b_l, _) = bisect((0, b_opt), |b| (time - b) * b > dist);
    let (_, b_r) = bisect((b_opt, time), |b| (time - b) * b <= dist);
    b_r - b_l - 1
}

pub fn star_2(s: &str) -> SolT {
    let mut values = s.lines().map(|line| {
        line.bytes()
            .filter(|b| (b'0'..=b'9').contains(b))
            .fold(0, |val, b| 10 * val + (b - b'0') as SolT)
    });
    play_smart((values.next().unwrap(), values.next().unwrap()))
}
// end::star_2[]

// tag::tests[]
#[cfg(test)]
mod tests {
    use super::*;

    const CONTENT: &str = r#"Time:      7  15   30
Distance:  9  40  200
"#;

    #[test]
    pub fn test_star_1() {
        assert_eq!(288, star_1(&CONTENT));
    }

    #[test]
    pub fn test_star_2() {
        assert_eq!(71503, star_2(&CONTENT));
    }
}
// end::tests[]