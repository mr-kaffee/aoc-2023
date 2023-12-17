use input::*;
use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    fs::read_to_string,
    iter::successors,
};

// tag::prelude[]
pub const IDENTIFIER: &str = "2023/17";

pub type SolT = usize;
pub type InputT<'a> = PuzzleData<'a>;

pub fn read_input() -> String {
    read_to_string("../../../inputs/input17").unwrap()
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
            let w = data.iter().position(|b| b == &b'\n').unwrap_or(data.len());
            let h = (data.len() + 1) / (w + 1);
            Self(data, w, h)
        }
    }
}
// end::input[]

// tag::star_1[]
#[cfg(feature = "initial-star-1")]
pub fn star_1(&PuzzleData(grid, w, h): &PuzzleData) -> SolT {
    type NodeT = ((usize, usize), (u8, u8));

    let start = (0, 0);
    let target = (w - 1, h - 1);
    let mut seen = HashSet::<NodeT>::new();
    let mut queue = BinaryHeap::<(SolT, NodeT)>::from([(!0, (start, (0, 0)))]);
    while let Some((loss, ((c0, r0), (h0, s0)))) = queue.pop() {
        if (c0, r0) == target {
            return !loss;
        }

        let dhs: &[u8] = if s0 == 3 { &[1, 3] } else { &[0, 1, 3] };
        for (h1, s1) in dhs
            .iter()
            .map(|&dh| ((h0 + dh) & 3, if dh == 0 { s0 + 1 } else { 1 }))
        {
            let (dc, dr) = D[h1 as usize];
            let (c1, r1) = (c0.wrapping_add_signed(dc), r0.wrapping_add_signed(dr));
            if c1 < w && r1 < h && seen.insert(((c1, r1), (h1, s1))) {
                queue.push((
                    loss - (grid[c1 + r1 * (w + 1)] - b'0') as SolT,
                    ((c1, r1), (h1, s1)),
                ));
            }
        }
    }

    panic!("No solution!");
}
// end::star_1[]

// tag::solution[]
const D: [(isize, isize); 4] = [(1, 0), (0, -1), (-1, 0), (0, 1)];

pub fn loss_bounds(grid: &[u8], w: usize, h: usize) -> Vec<SolT> {
    let target = (w - 1, h - 1);
    let target_cost = !((grid[(w + 1) * h - 2] - b'0') as SolT);
    let mut bounds = vec![0; w * h];
    bounds[w * h - 1] = target_cost;
    // Queue is a max heap! Need to inverse cost (bitwise complement and subtract).
    let mut queue = BinaryHeap::from([(target_cost, target)]);
    while let Some((cost_0, (c0, r0))) = queue.pop() {
        for (c1, r1) in D
            .map(|(dc, dr)| (c0.wrapping_add_signed(dc), r0.wrapping_add_signed(dr)))
            .into_iter()
            .filter(|&(c1, r1)| c1 < w && r1 < h)
        {
            // settle value; this is safe because the cost adder to reach a node
            // is the same no matter from which neighbor we reach the node
            let r = &mut bounds[c1 + r1 * w];
            if *r == 0 {
                *r = cost_0 - (grid[c1 + r1 * (w + 1)] - b'0') as SolT;
                queue.push((*r, (c1, r1)))
            }
        }
    }
    // undo reversion and subtract self cost
    // (self cost was added in the first place to be able to settle early)
    bounds
        .into_iter()
        .enumerate()
        .map(|(pos, b)| match grid[pos + pos / w] {
            b'.' => !b,
            v => !b - (v - b'0') as SolT,
        })
        .collect()
}

pub fn minimize_loss(grid: &[u8], w: usize, h: usize, s_max: u8, s_min: u8) -> SolT {
    type NodeT = ((usize, usize), (u8, u8));

    // A* heuristic
    let bounds = loss_bounds(grid, w, h);

    // start nodes
    let starts: &[NodeT] = &[((0, 0), (0, s_max)), ((0, 0), (3, s_max))];

    // target predicate
    let is_target = |(pos, _): &NodeT| pos == &(w - 1, h - 1);

    // Queue is a max heap! Need to inverse cost (bitwise complement and subtract).
    let mut queue = BinaryHeap::new();
    let mut costs = HashMap::new();
    let mut settled = HashSet::new();

    // initialize
    for &((col, row), hs) in starts {
        let cost = !0 - bounds[col + row * w];
        queue.push((cost, ((col, row), hs)));
        costs.insert(((col, row), hs), cost);
    }

    while let Some((cost_0, node_0)) = queue.pop() {
        if is_target(&node_0) {
            return !cost_0;
        }

        if !settled.insert(node_0) {
            continue;
        }

        let ((c0, r0), (h0, s0)) = node_0;
        debug_assert!(s0 >= s_min, "s0 >= s_min expected");
        let dhs: &[u8] = if s0 < s_max { &[0, 1, 3] } else { &[1, 3] };
        for (weight, ((c1, r1), (h1, s1))) in dhs
            .iter()
            .map(|&dh| ((h0 + dh) & 3, if dh == 0 { s0 } else { 0 }))
            .filter_map(|(h1, s1)| {
                // move one step or enough to complete s_min, whatever is more
                let to_go = 1.max(s_min - s1.min(s_min));
                let (dc, dr) = D[h1 as usize];
                successors(Some((0, c0, r0)), |(weight, c, r)| {
                    // return Some - summing weights - while in bounds
                    let (c, r) = (c.wrapping_add_signed(dc), r.wrapping_add_signed(dr));
                    (c < w && r < h)
                        .then(|| (weight + (grid[c + r * (w + 1)] - b'0') as SolT, c, r))
                })
                .nth(to_go as _)
                .map(|(weight, c1, r1)| (weight, ((c1, r1), (h1, s1 + to_go))))
            })
            .filter(|(_, node_1)| !settled.contains(node_1))
        {
            let cost_1 = cost_0 - weight + bounds[c0 + r0 * w] - bounds[c1 + r1 * w];
            let node_1 = ((c1, r1), (h1, s1));

            let cost = costs.entry(node_1).or_insert(0);
            if cost_1 > *cost {
                *cost = cost_1;
                queue.push((cost_1, node_1));
            }
        }
    }

    panic!("No solution!");
}

#[cfg(not(feature = "initial-star-1"))]
pub fn star_1(&PuzzleData(grid, w, h): &PuzzleData) -> SolT {
    minimize_loss(grid, w, h, 3, 0)
}

pub fn star_2(&PuzzleData(grid, w, h): &PuzzleData) -> SolT {
    minimize_loss(grid, w, h, 10, 4)
}
// end::solution[]

// tag::tests[]
#[cfg(test)]
mod tests {
    use super::*;

    const CONTENT: &str = r#"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533
"#;

    const CONTENT_2: &str = r#"111111111111
999999999991
999999999991
999999999991
999999999991
"#;

    #[test]
    pub fn test_from() {
        let PuzzleData(grid, w, h) = PuzzleData::from(CONTENT);
        assert_eq!(13, w);
        assert_eq!(13, h);
        assert_eq!(13 * 14, grid.len());
    }

    #[test]
    pub fn test_star_1() {
        assert_eq!(102, star_1(&CONTENT.into()));
    }

    #[test]
    pub fn test_star_2() {
        assert_eq!(94, star_2(&CONTENT.into()));
        assert_eq!(71, star_2(&CONTENT_2.into()));
    }

    #[test]
    pub fn test_loss_bounds() {
        let PuzzleData(grid, w, h) = CONTENT_2.into();
        let bounds = loss_bounds(grid, w, h);
        for row in 0..h {
            for col in 0..w {
                print!("{:>3}", bounds[col + row * w]);
            }
            println!();
        }

        let exp = [
            [15, 14, 13, 12, 11, 10, 09, 08, 07, 06, 05, 04],
            [16, 15, 14, 13, 12, 11, 10, 09, 08, 07, 04, 03],
            [25, 24, 23, 22, 21, 20, 19, 18, 17, 12, 03, 02],
            [34, 33, 32, 31, 30, 29, 28, 27, 20, 11, 02, 01],
            [43, 42, 41, 40, 39, 38, 37, 28, 19, 10, 01, 00],
        ]
        .concat();

        assert_eq!(exp, bounds);
    }
}
// end::tests[]
