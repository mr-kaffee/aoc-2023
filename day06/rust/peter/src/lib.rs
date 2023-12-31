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
    // distance: (time - b) * b
    (0..=time).filter(|&b| (time - b) * b > dist).count() as _
}

pub fn star_1(s: &str) -> SolT {
    #[cfg(feature = "play_math")]
    const PLAY: fn((SolT, SolT)) -> SolT = play_math;
    #[cfg(all(not(feature = "play_math"), not(feature = "play_1_smart")))]
    const PLAY: fn((SolT, SolT)) -> SolT = play_naive;
    #[cfg(all(not(feature = "play_math"), feature = "play_1_smart"))]
    const PLAY: fn((SolT, SolT)) -> SolT = play_smart;

    let mut lines = s.lines().map(|line| {
        line.split_ascii_whitespace()
            .skip(1)
            .map(|value| value.parse::<SolT>().unwrap())
    });
    lines
        .next()
        .unwrap()
        .zip(lines.next().unwrap())
        .map(PLAY)
        .product()
}
// end::star_1[]

// tag::star_2[]
fn bisect<F: Fn(SolT) -> bool>(bs: (SolT, SolT), test: F) -> (SolT, SolT) {
    successors(Some(bs), |&(b_l, b_r)| {
        let b = (b_l + b_r) >> 1;
        if test(b) {
            Some((b_l, b))
        } else {
            Some((b, b_r))
        }
    })
    .find(|(b_l, b_r)| b_r - b_l <= 1)
    .unwrap()
}

pub fn play_smart((time, dist): (SolT, SolT)) -> SolT {
    // distance: (time - b) * b
    // optimum: time - 2 b = 0 => (time - time >> 1) * (time >> 1)
    let b_opt = time >> 1;
    let (b_l, _) = bisect((0, b_opt), |b| (time - b) * b > dist);
    let (_, b_r) = bisect((b_opt, time), |b| (time - b) * b <= dist);
    b_r - b_l - 1
}

pub fn play_math((time, dist): (SolT, SolT)) -> SolT {
    // distance: (time - b) * b > dist
    // b^2 - time * b + dist = 0
    // b = (time +/- sqrt(time^2 - 4 dist)) / 2

    let sqrt_d = ((time * time - 4 * dist) as f64).sqrt();
    let b1 = (((time as f64) - sqrt_d) / 2.0).floor() as SolT;
    let b2 = (((time as f64) + sqrt_d) / 2.0).ceil() as SolT;

    b2 - b1 - 1
}

pub fn star_2(s: &str) -> SolT {
    #[cfg(feature = "play_math")]
    const PLAY: fn((SolT, SolT)) -> SolT = play_math;
    #[cfg(all(not(feature = "play_math"), feature = "play_2_naive"))]
    const PLAY: fn((SolT, SolT)) -> SolT = play_naive;
    #[cfg(all(not(feature = "play_math"), not(feature = "play_2_naive")))]
    const PLAY: fn((SolT, SolT)) -> SolT = play_smart;

    let mut values = s.lines().map(|line| {
        line.bytes()
            .filter(u8::is_ascii_digit)
            .fold(0, |val, b| 10 * val + (b - b'0') as SolT)
    });
    PLAY((values.next().unwrap(), values.next().unwrap()))
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
        assert_eq!(71_503, star_2(&CONTENT));
    }
}
// end::tests[]
