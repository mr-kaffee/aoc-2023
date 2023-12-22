use input::*;
use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
    fs::read_to_string,
    ops::RangeInclusive,
};

// tag::prelude[]
pub const IDENTIFIER: &str = "2023/22";

pub type InputT = PuzzleData;

pub fn read_input() -> String {
    read_to_string("../../../inputs/input22").unwrap()
}
// end::prelude[]

// tag::input[]
pub mod input {
    use crate::{Brick, Point};

    #[derive(Debug)]
    pub struct PuzzleData(pub Vec<Brick>);

    impl From<&str> for Point {
        fn from(value: &str) -> Self {
            let mut values = value.split(',').map(|v| v.parse().unwrap());
            Self {
                x: values.next().unwrap(),
                y: values.next().unwrap(),
                z: values.next().unwrap(),
            }
        }
    }

    impl From<&str> for Brick {
        fn from(value: &str) -> Self {
            value
                .split_once('~')
                .map(|(a, b)| Self(a.into(), b.into()))
                .unwrap()
        }
    }

    impl<T> From<&T> for PuzzleData
    where
        T: AsRef<str> + ?Sized,
    {
        fn from(s: &T) -> Self {
            Self(s.as_ref().lines().map(Brick::from).collect())
        }
    }
}
// end::input[]

// tag::solution[]
pub type Coord = usize;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Point {
    pub x: Coord,
    pub y: Coord,
    pub z: Coord,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Brick(pub Point, pub Point);

pub struct SandStack {
    bricks: Vec<Brick>,
    belows: Vec<Vec<(usize, Coord)>>,
}

trait Intersect {
    fn intersect(&self, other: &Self) -> Self;
}

impl Intersect for RangeInclusive<Coord> {
    fn intersect(&self, other: &Self) -> Self {
        *self.start().max(other.start())..=*self.end().min(other.end())
    }
}

impl From<(Coord, Coord, Coord)> for Point {
    fn from((x, y, z): (Coord, Coord, Coord)) -> Self {
        Self { x, y, z }
    }
}

impl Brick {
    pub fn compare(&self, other: &Self) -> Option<(Ordering, Coord)> {
        if !self.x_range().intersect(&other.x_range()).is_empty()
            && !self.y_range().intersect(&other.y_range()).is_empty()
        {
            Some((
                self.top_z().cmp(&other.top_z()),
                self.top_z().min(other.top_z()),
            ))
        } else {
            None
        }
    }

    pub fn x_range(&self) -> RangeInclusive<Coord> {
        self.0.x.min(self.1.x)..=self.0.x.max(self.1.x)
    }

    pub fn y_range(&self) -> RangeInclusive<Coord> {
        self.0.y.min(self.1.y)..=self.0.y.max(self.1.y)
    }

    pub fn top_z(&self) -> Coord {
        self.0.z.max(self.1.z)
    }

    pub fn bottom_z(&self) -> Coord {
        self.0.z.min(self.1.z)
    }
}

impl From<&PuzzleData> for SandStack {
    fn from(PuzzleData(bricks): &PuzzleData) -> Self {
        let mut belows = vec![Vec::new(); bricks.len()];
        for (k1, b1) in bricks.iter().enumerate() {
            for (k2, b2) in bricks.iter().enumerate().skip(k1 + 1) {
                match b1.compare(b2) {
                    Some((Ordering::Greater, b)) => belows[k1].push((k2, b)),
                    Some((_, b)) => belows[k2].push((k1, b)),
                    None => (),
                }
            }
        }
        let bricks = bricks.to_owned();
        Self { bricks, belows }
    }
}

impl SandStack {
    fn settle_rec(&mut self, settled: &mut [bool], offsets: &mut [usize], k: usize) {
        if settled[k] || self.bricks[k].bottom_z() == 1 {
            return;
        }

        let z = (0..self.belows[k].len())
            .map(|p| {
                let (k2, _) = self.belows[k][p];
                self.settle_rec(settled, offsets, k2);
                let (_, ref mut z) = &mut self.belows[k][p];
                *z -= offsets[k2];
                *z
            })
            .max()
            .unwrap_or(0);

        let me = &mut self.bricks[k];
        assert!(me.bottom_z() > z);

        offsets[k] = me.bottom_z() - z - 1;
        me.0.z -= offsets[k];
        me.1.z -= offsets[k];

        settled[k] = true;
    }

    pub fn settle(&mut self) {
        let mut settled = vec![false; self.bricks.len()];
        let mut offsets = vec![0; self.bricks.len()];
        for k in 0..self.bricks.len() {
            self.settle_rec(&mut settled, &mut offsets, k);
        }
    }

    pub fn count_disintegrateable(&self) -> usize {
        let mut unique_supporters = vec![false; self.bricks.len()];
        for (k, me) in self.bricks.iter().enumerate() {
            let supporters = self.belows[k]
                .iter()
                .filter(|(_, z)| z + 1 == me.bottom_z())
                .map(|(p, _)| *p)
                .take(2)
                .collect::<Vec<_>>();
            if supporters.len() == 1 {
                unique_supporters[supporters[0]] = true;
            }
        }
        self.bricks.len() - unique_supporters.into_iter().filter(|&s| s).count()
    }

    pub fn sum_count_falling(&self) -> usize {
        // figure out what is supported above and what is supporting below
        let mut supported = vec![Vec::new(); self.bricks.len()];
        let mut supporting = vec![Vec::new(); self.bricks.len()];
        for (k_above, (z_above, belows)) in (self.bricks.iter().map(Brick::bottom_z))
            .zip(self.belows.iter())
            .enumerate()
        {
            for &(k_below, _) in belows.iter().filter(|(_, z_below)| z_below + 1 == z_above) {
                supported[k_below].push(k_above);
                supporting[k_above].push(k_below);
            }
        }

        // use binary heap (max heap) and sort by max z (inverted) to make
        // sure we process bricks layer by layer
        let mut queue = BinaryHeap::new();
        let mut falling = HashSet::new();
        let mut sum_count = 0;
        for k_root in 0..self.bricks.len() {
            falling.insert(k_root);
            queue.push((!self.bricks[k_root].top_z(), k_root));
            while let Some((_, k_below)) = queue.pop() {
                for &k_above in supported[k_below].iter() {
                    if supporting[k_above].iter().all(|k| falling.contains(k))
                        && falling.insert(k_above)
                    {
                        queue.push((!self.bricks[k_above].top_z(), k_above));
                    }
                }
            }
            sum_count += falling.len() - 1; // do not count self
            falling.clear();
        }

        sum_count
    }
}

pub fn star_1_and_2(data: &PuzzleData) -> (usize, usize) {
    let mut universe = SandStack::from(data);
    universe.settle();
    (
        universe.count_disintegrateable(),
        universe.sum_count_falling(),
    )
        .into()
}
// end::solution[]

// tag::tests[]
#[cfg(test)]
mod tests {
    use super::*;

    const CONTENT: &str = r#"1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9
"#;

    #[test]
    pub fn test_from() {
        let data = PuzzleData::from(CONTENT);

        let exp = vec![
            Brick((1, 0, 1).into(), (1, 2, 1).into()),
            Brick((0, 0, 2).into(), (2, 0, 2).into()),
            Brick((0, 2, 3).into(), (2, 2, 3).into()),
            Brick((0, 0, 4).into(), (0, 2, 4).into()),
            Brick((2, 0, 5).into(), (2, 2, 5).into()),
            Brick((0, 1, 6).into(), (2, 1, 6).into()),
            Brick((1, 1, 8).into(), (1, 1, 9).into()),
        ];

        assert_eq!(exp, data.0);

        println!("{data:?}");
    }

    #[test]
    pub fn test_star_1_and_2() {
        assert_eq!((5, 7), star_1_and_2(&CONTENT.into()).into());
    }
}
// end::tests[]
