use input::*;
#[cfg(feature = "plot")]
use std::collections::HashMap;
use std::fs::read_to_string;

// tag::prelude[]
pub const IDENTIFIER: &str = "2023/17";

pub type SolT = usize;
pub type SolST = isize;
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

const D: [(SolST, SolST); 4] = [(1, 0), (0, -1), (-1, 0), (0, 1)];

// tag::loss_bounds[]
#[cfg(not(feature = "no-heuristic"))]
fn loss_bounds(grid: &[u8], w: usize, h: usize) -> Vec<SolT> {
    use std::collections::BinaryHeap;

    let target = (w - 1, h - 1);
    let mut bounds = vec![SolT::MAX; w * h];
    bounds[w * h - 1] = 0;
    // queue is a max heap! Need to inverse cost.
    let mut queue =
        BinaryHeap::from([(SolT::MAX - (grid[(w + 1) * h - 2] - b'0') as SolT, target)]);
    while let Some((cost_0, (c0, r0))) = queue.pop() {
        for (c1, r1) in D
            .map(|(dc, dr)| (c0.wrapping_add_signed(dc), r0.wrapping_add_signed(dr)))
            .into_iter()
            .filter(|&(c1, r1)| c1 < w && r1 < h)
        {
            // settle value; this is safe because the cost adder to reach a node
            // is the same no matter from which neighbor we reach the node
            let r = &mut bounds[c1 + r1 * w];
            if *r == SolT::MAX {
                *r = SolT::MAX - cost_0;
                queue.push((cost_0 - (grid[c1 + r1 * (w + 1)] - b'0') as SolT, (c1, r1)))
            }
        }
    }
    bounds
}

#[cfg(not(feature = "no-heuristic"))]
fn loss_bounds_heuristic(grid: &[u8], w: usize, h: usize) -> impl Fn((usize, usize)) -> SolT {
    let bounds = loss_bounds(grid, w, h);
    move |(col, row)| bounds[col + row * w]
}

#[cfg(feature = "no-heuristic")]
fn loss_bounds_heuristic(_: &[u8], _: usize, _: usize) -> impl Fn((usize, usize)) -> SolT {
    |_| 0
}
// end::loss_bounds[]

#[cfg(feature = "settle-early")]
type NodeT = ((usize, usize), (u8, u8));

#[cfg(all(feature = "plot", feature = "settle-early"))]
pub fn to_string(
    w: usize,
    h: usize,
    ((c, r), (hd, s)): NodeT,
    parents: &HashMap<NodeT, NodeT>,
) -> String {
    use std::iter::successors;

    let mut data = vec!['.'; (w + 1) * h];
    (0..h).for_each(|r| data[w + r * (w + 1)] = '\n');

    parents.keys().for_each(|((c, r), _)| {
        data[c + r * (w + 1)] = '+';
    });

    successors(Some((((c, r), (hd, s)), ((c, r), (hd, s)))), |(_, n1)| {
        parents.get(n1).map(|n2| (*n1, *n2))
    })
    .flat_map(|(((c1, r1), _), ((c2, r2), _))| {
        (c1.min(c2)..=c1.max(c2)).flat_map(move |c| (r1.min(r2)..=r1.max(r2)).map(move |r| (c, r)))
    })
    .for_each(|(c, r)| data[c + r * (w + 1)] = '#');

    data.iter().collect()
}

// tag::settle-early[]
#[cfg(feature = "settle-early")]
pub fn optimize(grid: &[u8], w: usize, h: usize, s_max: u8, s_min: u8) -> SolT {
    use std::collections::{BinaryHeap, HashSet};
    use std::iter::successors;

    let heuristic = loss_bounds_heuristic(grid, w, h);

    let start_cost = heuristic((0, 0));
    let starts: [(usize, NodeT); 2] = [
        (start_cost, ((0, 0), (0, s_max))),
        (start_cost, ((0, 0), (3, s_max))),
    ];

    let target_pos = (w - 1, h - 1);

    let mut queue = BinaryHeap::new();
    let mut seen = HashSet::new();
    for (cost, node) in starts.into_iter() {
        queue.push((SolT::MAX - cost, node));
        seen.insert(node);
    }

    #[cfg(feature = "plot")]
    let mut parents = HashMap::new();

    while let Some((cost_0, ((c0, r0), (hd0, s0)))) = queue.pop() {
        if (c0, r0) == target_pos {
            #[cfg(feature = "plot")]
            println!("{}", to_string(w, h, ((c0, r0), (hd0, s0)), &parents));
            return SolT::MAX - cost_0;
        }

        let dhs: &[u8] = if s0 < s_max { &[0, 1, 3] } else { &[1, 3] };
        for (cost_1, node_1) in dhs
            .iter()
            .map(move |&dh| ((hd0 + dh) & 3, if dh == 0 { s0 } else { 0 }))
            .filter_map(|(hd1, s1)| {
                // move one step or enough to complete s_min, whatever is more
                let to_go = 1.max(s_min - s1.min(s_min));
                let (dc, dr) = D[hd1 as usize];
                successors(Some((0, c0, r0)), |(weight, c, r)| {
                    // return Some - summing weights - while in bounds
                    let (c, r) = (c.wrapping_add_signed(dc), r.wrapping_add_signed(dr));
                    (c < w && r < h)
                        .then(|| (weight + (grid[c + r * (w + 1)] - b'0') as SolT, c, r))
                })
                .nth(to_go as _)
                .map(|(weight, c1, r1)| {
                    (
                        cost_0 - weight + heuristic((c0, r0)) - heuristic((c1, r1)),
                        ((c1, r1), (hd1, s1 + to_go)),
                    )
                })
            })
        {
            if seen.insert(node_1) {
                queue.push((cost_1, node_1));
                #[cfg(feature = "plot")]
                parents.insert(node_1, ((c0, r0), (hd0, s0)));
            }
        }
    }

    panic!("No solution found.")
}
// end::settle-early[]

#[cfg(not(feature = "settle-early"))]
type NodeT = ((usize, usize), u8);

#[cfg(all(feature = "plot", not(feature = "settle-early")))]
pub fn to_string(
    w: usize,
    h: usize,
    ((c, r), hd): NodeT,
    settled: &[u8],
    parents: &HashMap<NodeT, NodeT>,
) -> String {
    use std::iter::successors;

    let mut data = vec!['.'; (w + 1) * h];
    (0..h).for_each(|r| data[w + r * (w + 1)] = '\n');

    parents.keys().for_each(|((c, r), _)| {
        data[c + r * (w + 1)] = '-';
    });

    (0..settled.len())
        .filter(|&p| settled[p] > 0)
        .for_each(|p| data[p % w + (p / w) * (w + 1)] = '+');

    successors(Some((((c, r), hd), ((c, r), hd))), |(_, n1)| {
        parents.get(n1).map(|n2| (*n1, *n2))
    })
    .flat_map(|(((c1, r1), _), ((c2, r2), _))| {
        (c1.min(c2)..=c1.max(c2)).flat_map(move |c| (r1.min(r2)..=r1.max(r2)).map(move |r| (c, r)))
    })
    .for_each(|(c, r)| data[c + r * (w + 1)] = '#');

    data.iter().collect()
}

// tag::greedy-expand[]
#[cfg(not(feature = "settle-early"))]
pub fn optimize(grid: &[u8], w: usize, h: usize, s_max: u8, s_min: u8) -> SolT {
    use std::collections::BinaryHeap;
    use std::iter::successors;

    let heuristic = &loss_bounds_heuristic(grid, w, h);

    let start_cost = SolT::MAX - heuristic((0, 0));
    let starts: [(usize, NodeT); 2] = [(start_cost, ((0, 0), 0)), (start_cost, ((0, 0), 3))];

    let target_pos = (w - 1, h - 1);

    let mut queue = BinaryHeap::new();
    let mut settled = vec![0u8; w * h];
    let mut costs = vec![0; w * h * 4];
    for (cost, ((c, r), hd)) in starts.into_iter() {
        queue.push((cost, ((c, r), hd)));
        costs[c + r * w + (hd as usize) * w * h] = cost;
    }

    #[cfg(feature = "plot")]
    let mut parents = HashMap::new();

    while let Some((cost_0, ((c0, r0), hd0))) = queue.pop() {
        if (c0, r0) == target_pos {
            #[cfg(feature = "plot")]
            println!("{}", to_string(w, h, ((c0, r0), hd0), &settled, &parents));
            return SolT::MAX - cost_0;
        }

        if settled[c0 + r0 * w] & (1 << hd0) == 0 {
            settled[c0 + r0 * w] |= 1 << hd0;
        } else {
            continue;
        }

        for (cost_1, ((c1, r1), hd1)) in [1, 3]
            .iter()
            .map(move |&dh| (hd0 + dh) & 3)
            .flat_map(|hd1| {
                let (dc, dr) = D[hd1 as usize];
                successors(Some((0, c0, r0)), move |(weight, c, r)| {
                    let (c, r) = (c.wrapping_add_signed(dc), r.wrapping_add_signed(dr));
                    (c < w && r < h)
                        .then(|| (weight + (grid[c + r * (w + 1)] - b'0') as SolT, c, r))
                })
                .take(s_max as usize + 1)
                .skip(1.max(s_min as _))
                .map(move |(weight, c1, r1)| {
                    (
                        cost_0 - weight + heuristic((c0, r0)) - heuristic((c1, r1)),
                        ((c1, r1), hd1),
                    )
                })
            })
            .filter(|(_, ((r1, c1), hd1))| settled[r1 + c1 * w] & (1 << hd1) == 0)
        {
            let cost_1_prev = &mut costs[c1 + r1 * w + (hd1 as usize) * w * h];
            if cost_1 > *cost_1_prev {
                *cost_1_prev = cost_1;
                queue.push((cost_1, ((c1, r1), hd1)));
                #[cfg(feature = "plot")]
                parents.insert(((c1, r1), hd1), ((c0, r0), hd0));
            }
        }
    }

    panic!("No solution found.")
}
// end::greedy-expand[]

// tag::solution[]
pub fn star_1(&PuzzleData(grid, w, h): &PuzzleData) -> SolT {
    optimize(grid, w, h, 3, 0)
}

pub fn star_2(&PuzzleData(grid, w, h): &PuzzleData) -> SolT {
    optimize(grid, w, h, 10, 4)
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

    #[cfg(not(feature = "no-heuristic"))]
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
