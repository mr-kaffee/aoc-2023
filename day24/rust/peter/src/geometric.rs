use self::vec_3d::DotProd;
use crate::{geometric::vec_3d::CrossProd, input::PuzzleData};
use mr_kaffee_utils::euclid::gcd;

type Coord = i64;
type CoordE = i128;
type PosAndVelocity<T> = (Vec3D<T>, Vec3D<T>);

pub fn solve(PuzzleData(data): &PuzzleData) -> Coord {
    let data = data
        .iter()
        .map(|[x, y, z]| ([x.p, y.p, z.p].into(), [x.v, y.v, z.v].into()))
        .collect::<Vec<_>>();
    let (xr, _) = find_rock(&data);
    xr.dot_prod([1, 1, 1].into())
}

pub fn find_rock(data: &[PosAndVelocity<Coord>]) -> PosAndVelocity<Coord> {
    assert!(data.len() >= 3);

    let (xa, va) = up_cast(data[0]);
    let (xb, vb) = up_cast(data[1]);
    let (xc, vc) = up_cast(data[2]);

    let (xb1_a, vb_a) = (xb - xa, vb - va);
    let (xc1_a, vc_a) = (xc - xa, vc - va);
    let xb2_a = xb1_a + vb_a;
    let xc2_a = xc1_a + vc_a;

    let nb_a = (xb1_a.cross_prod(xb2_a)).shrink();
    debug_assert_eq!(0, nb_a.dot_prod(xb1_a));
    debug_assert_eq!(0, nb_a.dot_prod(xb2_a));

    let nc_a = (xc1_a.cross_prod(xc2_a)).shrink();
    debug_assert_eq!(0, nc_a.dot_prod(xc1_a));
    debug_assert_eq!(0, nc_a.dot_prod(xc2_a));

    let dir_vr_a = (nb_a.cross_prod(nc_a)).shrink();
    debug_assert_eq!(0, dir_vr_a.dot_prod(nb_a));
    debug_assert_eq!(0, dir_vr_a.dot_prod(nc_a));

    // xb1_a + t_b vb_a = T_b vr_a
    let (t_b, t_r_b, den_rb) = intersect(&(xb1_a, vb_a), &([0, 0, 0].into(), dir_vr_a));
    let xb_a_hit = xb1_a + vb_a * t_b;
    assert_eq!(1, den_rb);
    debug_assert_eq!(xb_a_hit, dir_vr_a * t_r_b);

    // xc1_a + t_c vc_a = t_r_c vr_a
    let (t_c, t_r_c, den_rc) = intersect(&(xc1_a, vc_a), &([0, 0, 0].into(), dir_vr_a));
    let xc_a_hit = xc1_a + vc_a * t_c;
    assert_eq!(1, den_rc);
    debug_assert_eq!(xc_a_hit, dir_vr_a * t_r_c);

    // determine actual rock velocity and position
    // xc_a_hit = xr_a + vr_a * t_c = dir_vr_a * t_r_c
    // xc_b_hit = xr_a + vr_a * t_b = dir_vr_a * t_r_b
    debug_assert_eq!(0, (t_r_c - t_r_b) % (t_c - t_b));
    let vr_a = dir_vr_a * ((t_r_c - t_r_b) / (t_c - t_b));
    debug_assert_eq!(0, (t_c * t_r_b - t_b * t_r_c) % (t_c - t_b));
    let xr_a = dir_vr_a * ((t_c * t_r_b - t_b * t_r_c) / (t_c - t_b));
    debug_assert_eq!(xr_a + vr_a * t_b, xb_a_hit);
    debug_assert_eq!(xr_a + vr_a * t_c, xc_a_hit);

    down_cast((xr_a + xa, vr_a + va))
}

fn up_cast((p, v): PosAndVelocity<Coord>) -> PosAndVelocity<CoordE> {
    (p.0.map(|p| p as _).into(), v.0.map(|v| v as _).into())
}

fn down_cast((p, v): PosAndVelocity<CoordE>) -> PosAndVelocity<Coord> {
    (p.0.map(|p| p as _).into(), v.0.map(|v| v as _).into())
}

fn intersect(
    (xa, va): &PosAndVelocity<CoordE>,
    (xb, vb): &PosAndVelocity<CoordE>,
) -> (CoordE, CoordE, CoordE) {
    let (xa, va) = (xa.0, va.0);
    let (xb, vb) = (xb.0, vb.0);

    // xa + t_a va = xb + t_b vb
    let (num_a, num_b, den) = (0..3)
        .map(|k| {
            let l = (k + 1) % 3;
            (
                vb[l] * (xa[k] - xb[k]) - vb[k] * (xa[l] - xb[l]),
                va[l] * (xa[k] - xb[k]) - va[k] * (xa[l] - xb[l]),
                vb[k] * va[l] - va[k] * vb[l],
            )
        })
        .find(|(_, _, den)| den != &0)
        .unwrap();
    let g = gcd(gcd(num_a, num_b), den).abs();
    (
        num_a / g * den.signum(),
        num_b / g * den.signum(),
        den.abs() / g,
    )
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test_find_rock() {
        // r(t) = [0,0,0] + t * [1,1,1]
        // a(t) = [1,1,1] + (t - 1) * [1,-1,0] = [0,2,1] + t * [1,-1,0]
        // b(t) = [2,2,2] + (t - 2) * [0,1,-1] = [2,0,4] + t * [0,1,-1]
        // c(t) = [3,3,3] + (t - 3) * [-1,0,1] = [6,3,0] + t * [-1,0,1]

        let r = ([0, 0, 0].into(), [1, 1, 1].into());
        let a = ([0, 2, 1].into(), [1, -1, 0].into());
        let b = ([2, 0, 4].into(), [0, 1, -1].into());
        let c = ([6, 3, 0].into(), [-1, 0, 1].into());

        assert_eq!(r, find_rock(&[a, b, c]));
    }

    const CONTENT: &str = r#"19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3
"#;

    #[test]
    pub fn test_solve() {
        // [24, 13, 10], [-3, 1, 2]
        assert_eq!(47, solve(&CONTENT.into()));
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Vec3D<T>(pub [T; 3]);

mod vec_3d {
    use super::Vec3D;
    use mr_kaffee_utils::euclid::{gcd, Zero};
    use std::{
        iter::Sum,
        ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, Sub, SubAssign},
    };

    impl<S, T> From<S> for Vec3D<T>
    where
        [T; 3]: From<S>,
    {
        fn from(value: S) -> Self {
            Self(value.into())
        }
    }

    pub trait Abs {
        fn abs(self) -> Self;
    }

    macro_rules! impl_abs_signed {
        ($($t:ty),+) => {$(
            impl Abs for $t {
                fn abs(self) -> Self {
                    <$t>::abs(self)
                }
            }
        )*};
    }

    impl_abs_signed!(isize, i8, i16, i32, i64, i128);

    macro_rules! impl_abs_unsigned {
        ($($t:ty),+) => {$(
            impl Abs for $t {
                fn abs(self) -> Self {
                    self
                }
            }
        )*};
    }

    impl_abs_unsigned!(usize, u8, u16, u32, u64, u128);

    impl<T: Eq + Copy + Rem<Output = T> + Zero + DivAssign + Abs> Vec3D<T> {
        pub fn shrink(mut self) -> Self {
            let g = gcd(gcd(self.0[0], self.0[1]), self.0[2]).abs();
            self /= g;
            self
        }
    }

    pub trait DotProd {
        type Output;

        fn dot_prod(self, rhs: Self) -> Self::Output;
    }

    impl<T: Mul<Output = T> + Sum> DotProd for Vec3D<T> {
        type Output = T;

        fn dot_prod(self, rhs: Self) -> Self::Output {
            self.0.into_iter().zip(rhs.0).map(|(a, b)| a * b).sum()
        }
    }

    pub trait CrossProd {
        fn cross_prod(self, rhs: Self) -> Self;
    }

    impl<T: Mul<Output = T> + Sub<Output = T> + Copy> CrossProd for Vec3D<T> {
        fn cross_prod(self, rhs: Self) -> Self {
            [
                self.0[1] * rhs.0[2] - self.0[2] * rhs.0[1],
                self.0[2] * rhs.0[0] - self.0[0] * rhs.0[2],
                self.0[0] * rhs.0[1] - self.0[1] * rhs.0[0],
            ]
            .into()
        }
    }

    impl<T: AddAssign> AddAssign for Vec3D<T> {
        fn add_assign(&mut self, rhs: Self) {
            for (lhs, rhs) in self.0.iter_mut().zip(rhs.0) {
                *lhs += rhs;
            }
        }
    }

    impl<T: AddAssign> Add for Vec3D<T> {
        type Output = Self;

        fn add(mut self, rhs: Self) -> Self::Output {
            self += rhs;
            self
        }
    }

    impl<T: SubAssign> SubAssign for Vec3D<T> {
        fn sub_assign(&mut self, rhs: Self) {
            for (lhs, rhs) in self.0.iter_mut().zip(rhs.0) {
                *lhs -= rhs;
            }
        }
    }

    impl<T: SubAssign> Sub for Vec3D<T> {
        type Output = Self;

        fn sub(mut self, rhs: Self) -> Self::Output {
            self -= rhs;
            self
        }
    }

    impl<T: Neg<Output = T>> Neg for Vec3D<T> {
        type Output = Self;

        fn neg(self) -> Self::Output {
            let Self([x, y, z]) = self;
            Self([-x, -y, -z])
        }
    }

    impl<T: MulAssign + Copy> MulAssign<T> for Vec3D<T> {
        fn mul_assign(&mut self, rhs: T) {
            for lhs in self.0.iter_mut() {
                *lhs *= rhs;
            }
        }
    }

    impl<T: MulAssign + Copy> Mul<T> for Vec3D<T> {
        type Output = Self;

        fn mul(mut self, rhs: T) -> Self::Output {
            self *= rhs;
            self
        }
    }

    impl<T: DivAssign + Copy> DivAssign<T> for Vec3D<T> {
        fn div_assign(&mut self, rhs: T) {
            for lhs in self.0.iter_mut() {
                *lhs /= rhs;
            }
        }
    }

    impl<T: DivAssign + Copy> Div<T> for Vec3D<T> {
        type Output = Self;

        fn div(mut self, rhs: T) -> Self::Output {
            self /= rhs;
            self
        }
    }
}
