use input::*;
use std::fs::read_to_string;

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

// tag::star_1[]
impl PuzzleData<'_> {
    pub fn start(&self) -> (usize, usize) {
        (
            self.data
                .iter()
                .take(self.w)
                .position(|&b| b == b'.')
                .unwrap(),
            0,
        )
    }

    pub fn target(&self) -> (usize, usize) {
        (
            self.data
                .iter()
                .skip((self.h - 1) * (self.w + 1))
                .position(|&b| b == b'.')
                .unwrap(),
            self.h - 1,
        )
    }

    fn is_branch_point(&self, (col, row): (usize, usize)) -> bool {
        self.data[col + row * (self.w + 1)] != b'#'
            && [(1, 0), (0, -1), (-1, 0), (0, 1)]
                .into_iter()
                .map(|(dc, dr)| (col.wrapping_add_signed(dc), row.wrapping_add_signed(dr)))
                .filter(|&(col, row)| {
                    col < self.w && row < self.h && self.data[col + row * (self.w + 1)] != b'#'
                })
                .count()
                > 2
    }

    pub fn branch_points(&self) -> Vec<(usize, usize)> {
        (0..self.w * self.h)
            .map(|pos| (pos % self.w, pos / self.w))
            .filter(|&coord| self.is_branch_point(coord))
            .collect::<Vec<_>>()
    }

    const D: [(isize, isize, u8); 4] = [(1, 0, b'>'), (0, -1, b'^'), (-1, 0, b'<'), (0, 1, b'v')];

    pub fn adj_iter(
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
}

pub fn star(grid: &PuzzleData, ignore_slopes: bool) -> usize {
    // nodes are branch points and start / target
    let mut nodes = grid.branch_points();
    nodes.push(grid.start());
    nodes.push(grid.target());

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
                grid.adj_iter(cur, ignore_slopes)
                    .filter(|&adj| Some(adj) != prev)
                    .map(|adj| (steps + 1, adj, Some(cur))),
            );
        }
    }

    // seen information is stored in bits of u64
    assert!(nodes.len() <= 64);

    // find maximum
    let start = nodes.len() - 2;
    let target = nodes.len() - 1;
    let mut queue = Vec::from([(0, start, 1u64 << start)]);
    let mut max = 0;
    while let Some((cost, idx, seen)) = queue.pop() {
        if idx == target && cost > max {
            max = cost;
            continue;
        }

        for &(adj, weight) in &adjacents[idx] {
            if seen & (1 << adj) == 0 {
                queue.push((cost + weight, adj, seen | 1 << adj));
            }
        }
    }
    max
}

pub fn star_1(grid: &PuzzleData) -> usize {
    star(grid, false)
}
// end::star_1[]

// tag::star_2[]
pub fn star_2(grid: &PuzzleData) -> usize {
    star(grid, true)
}
// end::star_2[]

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
