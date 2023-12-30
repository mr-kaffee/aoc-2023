use input::PuzzleData;
use std::collections::{HashMap, HashSet, VecDeque};
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

    impl<T: AsRef<str> + ?Sized> From<&T> for PuzzleData {
        fn from(value: &T) -> Self {
            let mut adjacents = Vec::<Vec<usize>>::new();
            let mut indices = HashMap::new();
            for line in value.as_ref().lines() {
                let mut parts = line
                    .split::<&[char]>(&[' ', ':'])
                    .map(str::trim)
                    .filter(|v| v.len() > 0)
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

// tag::star_1[]
pub fn get_shortest_path(
    adjacents: &[Vec<usize>],
    forbidden: &[Vec<(usize, usize)>],
    start: usize,
    target: usize,
) -> Option<HashMap<usize, usize>> {
    let mut seen = HashSet::from([start]);
    let mut queue = VecDeque::from([start]);
    let mut parents = HashMap::new();
    while let Some(idx) = queue.pop_front() {
        if idx == target {
            return Some(parents);
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

    None
}

pub fn get_target_with_three_paths(
    adjacents: &[Vec<usize>],
    start: usize,
) -> (usize, [Vec<(usize, usize)>; 3]) {
    let mut paths = [Vec::new(), Vec::new(), Vec::new()];
    for target in 1..adjacents.len() - 1 {
        // find three disjoint paths
        for k in 0..3 {
            let Some(parents) = get_shortest_path(adjacents, &paths[0..k], start, target) else {
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

        // check that there is no fourth path
        if get_shortest_path(adjacents, &paths, start, target).is_none() {
            return (target, paths);
        }
    }

    panic!("No solution");
}

pub fn count_reachable_without_target(
    adjacents: &[Vec<usize>],
    forbidden: &[(usize, usize)],
    start: usize,
    target: usize,
) -> Option<usize> {
    let mut queue = Vec::from([start]);
    let mut seen = HashSet::from([start]);
    let mut reached = 0;
    while let Some(idx) = queue.pop() {
        reached += 1;
        if idx == target {
            return None;
        }

        for &adj in &adjacents[idx] {
            if !forbidden.contains(&(adj.min(idx), adj.max(idx))) && seen.insert(adj) {
                queue.push(adj);
            }
        }
    }
    Some(reached)
}

pub fn star_1(PuzzleData(adjacents): &PuzzleData) -> usize {
    // get a target in other part of graph with three disjoint paths
    let (target, paths) = get_target_with_three_paths(adjacents, 0);

    // find a triple of links, one from every path, so that removal of those
    // links splits the graph in two
    let reachable = (0..paths.iter().map(Vec::len).product())
        .map(|k| {
            [
                paths[0][k % paths[0].len()],
                paths[1][(k / paths[0].len()) % paths[1].len()],
                paths[2][k / (paths[0].len() * paths[1].len())],
            ]
        })
        .find_map(|forbidden| count_reachable_without_target(adjacents, &forbidden, 0, target))
        .expect("No solution!");

    // return product of sizes of two disconnected parts of graph
    reachable * (adjacents.len() - reachable)
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
