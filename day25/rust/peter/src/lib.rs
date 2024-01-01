use input::PuzzleData;
use std::fs::read_to_string;

// tag::prelude[]
pub const IDENTIFIER: &str = "2023/25";

pub type InputT = PuzzleData;

pub fn read_input() -> String {
    read_to_string("../../../inputs/input25").unwrap()
}
// end::prelude[]

// tag::input[]
mod input {
    use std::collections::HashMap;

    pub struct PuzzleData(pub Vec<Vec<usize>>);

    impl<T: AsRef<str>> From<T> for PuzzleData {
        fn from(value: T) -> Self {
            let mut adjacents = Vec::<Vec<usize>>::new();
            let mut indices = HashMap::new();
            for line in value.as_ref().lines() {
                let mut parts = line
                    .split::<&[char]>(&[' ', ':'])
                    .map(str::trim)
                    .filter(|v| !v.is_empty())
                    .map(|key| {
                        let idx = indices.len();
                        *indices.entry(key).or_insert(idx)
                    });
                let key = parts.next().unwrap();
                for value in parts {
                    adjacents.resize(adjacents.len().max(key + 1).max(value + 1), Vec::new());
                    adjacents[key].push(value);
                    adjacents[value].push(key);
                }
            }

            Self(adjacents)
        }
    }
}
// end::input[]

#[cfg(feature = "min-cut")]
pub mod min_cut;

#[cfg(feature = "min-cut")]
pub fn star_1(PuzzleData(adjacents): &PuzzleData) -> usize {
    use min_cut::MinCut;

    let n = adjacents.len();
    let labels = (0..n as u16).collect();
    let adjacents = adjacents
        .iter()
        .map(|adjacents| adjacents.iter().map(|&idx| (idx, 1u16)).collect())
        .collect();

    let g = min_cut::adjacency_list::AdjacencyList { adjacents, labels };
    let (w, p) = g.min_cut_with_bound(|(w, _)| w <= &3).unwrap();
    assert_eq!(3, w);

    let l1 = p.len();
    l1 * (n - l1)
}

// tag::star_1[]
#[cfg(not(feature = "min-cut"))]
mod direct_solution {
    use std::collections::{HashMap, HashSet, VecDeque};

    pub fn get_shortest_path(
        adjacents: &[Vec<usize>],
        forbidden: &[Vec<(usize, usize)>],
        start: usize,
        target: usize,
    ) -> (Option<HashMap<usize, usize>>, usize) {
        let mut seen = HashSet::from([start]);
        let mut queue = VecDeque::from([start]);
        let mut parents = HashMap::new();
        let mut reached = 0;
        while let Some(idx) = queue.pop_front() {
            reached += 1;
            if idx == target {
                return (Some(parents), reached);
            }

            for &adj in adjacents[idx].iter().filter(|&&adj| {
                !forbidden
                    .iter()
                    .any(|forbidden| forbidden.contains(&(adj.min(idx), adj.max(idx))))
            }) {
                if seen.insert(adj) {
                    queue.push_back(adj);
                    parents.insert(adj, idx);
                }
            }
        }

        (None, reached)
    }

    pub fn get_partitions_connected_by_three_paths(
        adjacents: &[Vec<usize>],
        start: usize,
    ) -> (usize, usize) {
        let mut paths = [Vec::new(), Vec::new(), Vec::new()];
        for target in 1..adjacents.len() - 1 {
            // find three disjoint paths
            for k in 0..3 {
                let (Some(parents), _) = get_shortest_path(adjacents, &paths[0..k], start, target)
                else {
                    panic!("Less than three paths from {} to {}!", start, target);
                };
                paths[k].clear();
                let mut cur = target;
                while let Some(&parent) = parents.get(&cur) {
                    let link = (cur.min(parent), cur.max(parent));
                    paths[k].push(link);
                    cur = parent;
                }
            }

            // return partition sizes, if there is no fourth path
            if let (None, reached) = get_shortest_path(adjacents, &paths, start, target) {
                return (reached, adjacents.len() - reached);
            }
        }

        panic!("No solution");
    }
}

#[cfg(not(feature = "min-cut"))]
pub fn star_1(PuzzleData(adjacents): &PuzzleData) -> usize {
    // get a size of partitions connected by three paths
    let (p1, p2) = direct_solution::get_partitions_connected_by_three_paths(adjacents, 0);
    p1 * p2
}
// end::star_1[]

// tag::tests[]
#[cfg(test)]
mod tests {
    use super::*;

    const CONTENT: &str = r#"jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr
"#;

    #[test]
    pub fn test_star_1() {
        assert_eq!(54, star_1(&CONTENT.into()));
    }
}
// end::tests[]
