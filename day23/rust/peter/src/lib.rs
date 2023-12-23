use input::*;
use std::fs::read_to_string;
use std::collections::{hash_map::Entry, HashMap};

// tag::prelude[]
pub const IDENTIFIER: &str = "2023/23";

pub type InputT<'a> = PuzzleData<'a>;

pub fn read_input() -> String {
    read_to_string("../../../inputs/input23").unwrap()
}
// end::prelude[]

// tag::input[]
pub mod input {
    #[derive(Debug)]
    pub struct PuzzleData<'a> {
        pub data: &'a [u8],
        pub w: usize,
        pub h: usize,
    }

    impl<'a, T> From<&'a T> for PuzzleData<'a>
    where
        T: AsRef<[u8]> + 'a + ?Sized,
    {
        fn from(s: &'a T) -> Self {
            let data = s.as_ref();
            let w = data.iter().position(|&b| b == b'\n').unwrap_or(data.len());
            let h = (data.len() + 1) / (w + 1);
            Self { data, w, h }
        }
    }
}
// end::input[]

// tag::solution[]
impl PuzzleData<'_> {
    const D: [(isize, isize, u8); 4] = [(1, 0, b'>'), (0, -1, b'^'), (-1, 0, b'<'), (0, 1, b'v')];

    fn is_branch_point(&self, (col, row): (usize, usize)) -> bool {
        Self::D
            .iter()
            .map(|(dc, dr, _)| (col.wrapping_add_signed(*dc), row.wrapping_add_signed(*dr)))
            .filter(|&(col, row)| {
                col < self.w && row < self.h && self.data[col + row * (self.w + 1)] != b'#'
            })
            .count()
            > 2
    }

    pub fn branch_points(&self) -> Vec<(usize, usize)> {
        (0..self.w * self.h)
            .map(|pos| (pos % self.w, pos / self.w))
            .filter(|&(col, row)| {
                self.data[col + row * (self.w + 1)] != b'#'
                    && (row == 0 || row == self.h - 1 || self.is_branch_point((col, row)))
            })
            .collect::<Vec<_>>()
    }

    fn adj_iter(
        &self,
        (col, row): (usize, usize),
        ignore_slopes: bool,
    ) -> impl Iterator<Item = (usize, usize)> + '_ {
        Self::D
            .iter()
            .map(move |&(dc, dr, ok)| {
                (col.wrapping_add_signed(dc), row.wrapping_add_signed(dr), ok)
            })
            .filter(move |&(col, row, ok)| {
                col < self.w
                    && row < self.h
                    && (ignore_slopes && self.data[col + row * (self.w + 1)] != b'#'
                        || [b'.', ok].contains(&self.data[col + row * (self.w + 1)]))
            })
            .map(|(col, row, _)| (col, row))
    }

    /// nodes are branch points and start (first) / target (last)
    pub fn make_graph(
        &self,
        ignore_slopes: bool,
    ) -> (Vec<(usize, usize)>, Vec<Vec<(usize, usize)>>) {
        let nodes = self.branch_points();

        // calculate length of (unique) paths between all nodes
        let mut adjacents = vec![Vec::new(); nodes.len()];
        for (k0, &start) in nodes.iter().enumerate() {
            let mut queue = Vec::from([(0, start, None)]);
            while let Some((steps, cur, prev)) = queue.pop() {
                if let Some(k1) = (cur != start)
                    .then_some(())
                    .and_then(|_| nodes.iter().position(|&coord| coord == cur))
                {
                    adjacents[k0].push((k1, steps));
                    continue;
                }

                queue.extend(
                    self.adj_iter(cur, ignore_slopes)
                        .filter(|&adj| Some(adj) != prev)
                        .map(|adj| (steps + 1, adj, Some(cur))),
                );
            }
        }
        (nodes, adjacents)
    }
}

pub fn reachable(adj_masks: &[u64], idx: usize, seen: u64) -> u64 {
    let mut queue = 1u64 << idx;
    let mut reached = seen | queue;
    while queue != 0 {
        let cur = queue.trailing_zeros();
        queue &= !(1 << cur);

        let mask = adj_masks[cur as usize];
        queue |= mask & !reached;
        reached |= mask;
    }
    reached & !seen
}

pub fn star(grid: &PuzzleData, ignore_slopes: bool) -> usize {
    let (nodes, mut adjacents) = grid.make_graph(ignore_slopes);

    // seen information is stored in bits of u64
    assert!(nodes.len() <= 64);

    let target = nodes.len() - 1;
    if ignore_slopes {
        // Idea taken from https://www.reddit.com/user/MattieShoes/
        // At the last crossing before the target, we must go for the target
        let (last_before_target, cost) = adjacents[target][0];
        adjacents[last_before_target] = vec![(target, cost)];
    }

    let adj_masks: Vec<u64> = adjacents
        .iter()
        .map(|adjacents| adjacents.iter().fold(0, |mask, (adj, _)| mask | (1 << adj)))
        .collect();

    let mut bests = HashMap::new();

    // find maximum
    let mut queue = Vec::from([(0, 0, 1u64)]);
    let mut max = 0;
    while let Some((cost, idx, seen)) = queue.pop() {
        if idx == target && cost > max {
            max = cost;
            continue;
        }

        // Efficient BFS to get reachable nodes
        let reachable = reachable(&adj_masks, idx, seen);
        if reachable & (1 << target) == 0 {
            // Target is not reachable
            continue;
        }

        // Idea taken from https://www.reddit.com/user/boombulerDev/
        // If I have been here with the same set of reachable nodes, only expand node
        // if it has a higher cost
        match bests.entry((idx, reachable)) {
            Entry::Occupied(o) if cost <= *o.get() => continue,
            Entry::Occupied(mut o) => *o.get_mut() = cost,
            Entry::Vacant(v) => _ = v.insert(cost),
        }

        queue.extend(
            adjacents[idx]
                .iter()
                .filter(|(adj, _)| seen & (1 << adj) == 0)
                .map(|&(adj, weight)| (cost + weight, adj, seen | 1 << adj)),
        );
    }
    max
}

pub fn star_1(grid: &PuzzleData) -> usize {
    star(grid, false)
}

pub fn star_2(grid: &PuzzleData) -> usize {
    star(grid, true)
}
// end::solution[]

// tag::tests[]
#[cfg(test)]
mod tests {
    use super::*;

    const CONTENT: &str = r#"#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#
"#;

    #[test]
    pub fn test_from() {
        let data = PuzzleData::from(CONTENT);
        println!("{data:?}");
    }

    #[test]
    pub fn test_is_interesting() {
        let data = PuzzleData::from(CONTENT);
        assert_eq!(true, data.is_branch_point((11, 3)));
        assert_eq!(false, data.is_branch_point((4, 1)));
    }

    #[test]
    pub fn test_star_1() {
        assert_eq!(94, star_1(&CONTENT.into()));
    }

    #[test]
    pub fn test_star_2() {
        assert_eq!(154, star_2(&CONTENT.into()));
    }
}
// end::tests[]
