use input::*;
use std::{cmp::Ordering, ops::RangeInclusive};
use std::fs::read_to_string;

// tag::prelude[]
pub const IDENTIFIER: &str = "2023/24";

pub type InputT = PuzzleData;

pub fn read_input() -> String {
    read_to_string("../../../inputs/input24").unwrap()
}
// end::prelude[]

// tag::input[]
pub type Coord = i64;
pub type CoordE = i128;
#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
pub struct PAndV<T> {
    p: T,
    v: T,
}
pub type Hail = [PAndV<Coord>; 3];

pub mod input {
    use crate::Hail;

    #[derive(Debug)]
    pub struct PuzzleData(pub Vec<Hail>);

    impl<T> From<&T> for PuzzleData
    where
        T: AsRef<str> + ?Sized,
    {
        fn from(s: &T) -> Self {
            Self(
                s.as_ref()
                    .lines()
                    .map(|line| {
                        line.split(&[',', '@'])
                            .map(str::trim)
                            .map(str::parse)
                            .map(Result::unwrap)
                            .enumerate()
                            .fold(Hail::default(), |mut h, (p, v)| {
                                *match p {
                                    0..=2 => &mut h[p].p,
                                    3..=5 => &mut h[p - 3].v,
                                    _ => panic!(),
                                } = v;
                                h
                            })
                    })
                    .collect(),
            )
        }
    }
}
// end::input[]

// tag::star_1[]
const X: usize = 0;
const Y: usize = 1;
const Z: usize = 2;

pub fn intersect_2d(
    h1: &Hail,
    h2: &Hail,
    x_range: &RangeInclusive<Coord>,
    y_range: &RangeInclusive<Coord>,
) -> bool {
    let h1 = h1.map(|PAndV { p, v }| PAndV {
        p: p as CoordE,
        v: v as CoordE,
    });
    let h2 = h2.map(|PAndV { p, v }| PAndV {
        p: p as CoordE,
        v: v as CoordE,
    });

    let d = h2[X].v * h1[Y].v - h1[X].v * h2[Y].v;
    if d == 0 {
        // parallel
        h1[X].p == h2[X].p && h1[Y].p == h2[Y].p
    } else {
        let n1 = (h2[X].v * (h2[Y].p - h1[Y].p) - h2[Y].v * (h2[X].p - h1[X].p)) * d.signum();
        let n2 = (h1[X].v * (h2[Y].p - h1[Y].p) - h1[Y].v * (h2[X].p - h1[X].p)) * d.signum();
        let d = d.abs();

        debug_assert_eq!(h1[X].p * d + h1[X].v * n1, h2[X].p * d + h2[X].v * n2);
        debug_assert_eq!(h1[Y].p * d + h1[Y].v * n1, h2[Y].p * d + h2[Y].v * n2);

        if n1 < 0 || n2 < 0 {
            false
        } else {
            (*x_range.start() as CoordE * d..=*x_range.end() as CoordE * d)
                .contains(&(h1[X].p * d + h1[X].v * n1))
                && (*y_range.start() as CoordE * d..=*y_range.end() as CoordE * d)
                    .contains(&(h1[Y].p * d + h1[Y].v * n1))
        }
    }
}

pub fn count_intersections_2d(
    hails: &[Hail],
    x_range: RangeInclusive<Coord>,
    y_range: RangeInclusive<Coord>,
) -> usize {
    hails
        .iter()
        .enumerate()
        .map(|(k, h1)| {
            hails[k + 1..]
                .iter()
                .filter(|&h2| intersect_2d(h1, h2, &x_range, &y_range))
                .count()
        })
        .sum()
}

pub fn star_1(PuzzleData(hails): &PuzzleData) -> usize {
    const RANGE: RangeInclusive<Coord> = 200_000_000_000_000..=400_000_000_000_000;
    count_intersections_2d(hails, RANGE, RANGE)
}
// end::star_1[]

// tag::star_2[]
pub fn solve<const N: usize, const M: usize>(mut mat: [[f64; M]; N]) -> [f64; N] {
    assert!(M == N + 1, "M == N + 1 expected");

    let mut h = 0;
    let mut k = 0;

    // gaussian eliminate
    while h < N && k < M {
        let i_max = (h..N)
            .map(|i| (i, mat[i][k].abs()))
            .max_by(|(_, v1), (_, v2)| v1.partial_cmp(v2).unwrap_or(Ordering::Equal))
            .map(|(row, _)| row)
            .unwrap();
        if mat[i_max][k] == 0.0 {
            k += 1;
        } else {
            (mat[i_max], mat[h]) = (mat[h], mat[i_max]);
            for i in h + 1..N {
                let f = mat[i][k] / mat[h][k];
                mat[i][k] = 0.0;
                for j in k + 1..M {
                    mat[i][j] -= mat[h][j] * f;
                }
            }
            h += 1;
            k += 1;
        }
    }

    // calculate solution
    let mut r = [0.0; N];
    for k in (0..N).rev() {
        r[k] = mat[k][M - 1] / mat[k][k];
        for row in mat[0..k].iter_mut() {
            row[M - 1] -= row[k] * r[k];
        }
    }

    r
}

pub fn star_2(PuzzleData(hails): &PuzzleData) -> Coord {
    // number of dimensions
    const N: usize = 3;

    // Base equations
    //   x[i] + t[i] vx[i] = xr + t[i] vxr
    //   y[i] + t[i] vy[i] = yr + t[i] vyr
    //
    // Eliminate t[i]
    //   (x[i] - xr) / (vxr - vx[i]) = (y[i] - yr) / (vyr - vy[i])
    //   (x[i] - xr) (vyr - vy[i]) - (y[i] - yr) (vxr - vx[i]) = 0
    // This step creates additional solutions vxr = vx[i] and vyr = vy[i] with arbitrary
    // x[i], y[i] and xr = x[i] and yr = y[i] with arbitrary vx[i], vy[i].
    //
    // Expanding and re-arranging terms yields
    //   vyr xr - vxr yr = vy[i] xr - vx[i] yr - y[i] vxr + x[i] vyr + (vx[i] y[i] - vy[i] x[i])
    //   ----LHS_xy-----   ------------------------------RHS_xy[i]------------------------------
    // where all nonlinear terms are on the left (LHS_xy), independent of i.
    //
    // RHS_xy[i] - RHS_xy[j] = 0 finally yields a linear equation in the unknowns
    // xr, yr, vxr, vyr. Using four hailstones, we can construct 6 equations using
    // RHS_xy[i] and RHS_yz[i] for 6 unknowns.
    //
    // What about the additional solutions introduced above by potentially multiplying by 0?
    // - The problem has a unique solution
    // - Any solution satisfies the equations (we do not remove solutions by multiplying by 0)
    // => If the equations have a unique solution, it is the one we are looking for
    let mut mat = [[0.0; 2 * N + 1]; 2 * N];
    for k in 0..N {
        let l = k + 1;
        mat[k] = [
            (hails[k][Y].v - hails[l][Y].v) as _,
            (hails[l][Y].p - hails[k][Y].p) as _,
            (hails[l][X].v - hails[k][X].v) as _,
            (hails[k][X].p - hails[l][X].p) as _,
            0.0,
            0.0,
            ((hails[l][X].v * hails[l][Y].p - hails[l][Y].v * hails[l][X].p)
                - (hails[k][X].v * hails[k][Y].p - hails[k][Y].v * hails[k][X].p)) as _,
        ];
        mat[N + k] = [
            0.0,
            0.0,
            (hails[k][Z].v - hails[l][Z].v) as _,
            (hails[l][Z].p - hails[k][Z].p) as _,
            (hails[l][Y].v - hails[k][Y].v) as _,
            (hails[k][Y].p - hails[l][Y].p) as _,
            ((hails[l][Y].v * hails[l][Z].p - hails[l][Z].v * hails[l][Y].p)
                - (hails[k][Y].v * hails[k][Z].p - hails[k][Z].v * hails[k][Y].p)) as _,
        ];
    }

    // solve the linear equations and transform to hail
    let sol = solve(mat).map(|r| r.round() as Coord);
    let r = [
        PAndV {
            p: sol[0],
            v: sol[1],
        },
        PAndV {
            p: sol[2],
            v: sol[3],
        },
        PAndV {
            p: sol[4],
            v: sol[5],
        },
    ];

    // check solution on all points
    debug_assert_eq!(
        None,
        hails
            .iter()
            .map(|&h| [0, 1, 2].map(|k| (h[k].p - r[k].p, r[k].v - h[k].v)))
            .position(|deltas| {
                // check t is the same for all coordinates with non-zero velocity delta
                let (chk_1, _) = deltas
                    .into_iter()
                    .filter(|(_, v)| *v != 0)
                    .map(|(p, v)| p / v)
                    .fold((true, None), |(ok, cur_t), t| match (ok, cur_t) {
                        (false, _) => (false, None),
                        (_, None) => (t > 0, (t > 0).then_some(t)),
                        (_, Some(cur_t)) => (cur_t == t, (cur_t == t).then_some(cur_t)),
                    });
                // ... and coordinate deltas are multiples of velocity deltas
                let chk_2 = deltas
                    .into_iter()
                    .all(|(p, v)| v != 0 && p % v == 0 || v == 0 && p == 0);
                !(chk_1 && chk_2)
            })
            .map(|idx| hails[idx]),
        "Inconsistent solution"
    );

    r.into_iter().map(|c| c.p).sum()
}
// end::star_2[]

// tag::tests[]
#[cfg(test)]
mod tests {
    use super::*;

    const CONTENT: &str = r#"19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3
"#;

    #[test]
    pub fn test_from() {
        let PuzzleData(hails) = PuzzleData::from(CONTENT);
        println!("{hails:?}");
        assert_eq!(
            vec![
                [
                    PAndV { p: 19, v: -2 },
                    PAndV { p: 13, v: 1 },
                    PAndV { p: 30, v: -2 }
                ],
                [
                    PAndV { p: 18, v: -1 },
                    PAndV { p: 19, v: -1 },
                    PAndV { p: 22, v: -2 }
                ],
                [
                    PAndV { p: 20, v: -2 },
                    PAndV { p: 25, v: -2 },
                    PAndV { p: 34, v: -4 }
                ],
                [
                    PAndV { p: 12, v: -1 },
                    PAndV { p: 31, v: -2 },
                    PAndV { p: 28, v: -1 }
                ],
                [
                    PAndV { p: 20, v: 1 },
                    PAndV { p: 19, v: -5 },
                    PAndV { p: 15, v: -3 }
                ]
            ],
            hails
        );
    }

    #[test]
    pub fn test_star_1() {
        const RANGE: RangeInclusive<Coord> = 7..=27;
        let PuzzleData(hails) = CONTENT.into();
        assert_eq!(2, count_intersections_2d(&hails, RANGE, RANGE));
    }

    #[test]
    pub fn test_star_2() {
        assert_eq!(47, star_2(&CONTENT.into()));
    }
}
// end::tests[]
