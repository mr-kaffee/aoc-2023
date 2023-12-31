use std::{
    collections::HashMap,
    hash::Hash,
    ops::{Add, AddAssign},
};

pub trait Graph {
    /// The type of edge weights.
    type Weight;

    /// The type of vertex labels.
    type Label;

    /// Get list of vertex labels.
    fn vertex_labels(&self) -> &[Self::Label];

    /// Get label of vertex at given index.
    fn label(&self, idx: usize) -> &Self::Label {
        &self.vertex_labels()[idx]
    }

    /// Get index for vertex with given label.
    fn index_for_label(&self, label: &Self::Label) -> Option<usize>;

    /// Get length of graph = number of vertices.
    fn len(&self) -> usize;

    /// Check whether the graph is trivial
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Get weight of edge between two vertices at given indices.
    fn weight(&self, s_idx: usize, t_idx: usize) -> Self::Weight;
}

/// Implementation of the Stoer-Wagner algorithm to find a minimum cut
/// of a [`Graph`].
///
/// See https://blog.thomasjungblut.com/graph/mincut/mincut/
/// See https://en.wikipedia.org/wiki/Stoer%E2%80%93Wagner_algorithm
pub trait MinCut<W, L> {
    /// Perform a minimum cut phase.
    ///
    /// Return the weight of the cut and the indices of the last two
    /// vertices, `s` and `t`
    fn min_cut_phase(&self) -> (W, usize, usize);

    /// Perform minimum cut.
    ///
    /// Return the weight of the minimum cut and a list of vertex
    /// labels in one of the partitions.
    ///
    /// There is no guarantee on which of the partitions is returned.
    fn min_cut(self) -> Option<(W, Vec<L>)>;
}

pub trait Merge {
    /// Merge `t` into `s'. This will result in deleting `t`.
    fn merge(&mut self, s: usize, t: usize);
}

impl<T, W, L> MinCut<W, L> for T
where
    T: Graph<Weight = W, Label = L> + Merge,
    W: Ord + Add<Output = W> + AddAssign + Default,
    L: Copy + Eq + Hash,
{
    fn min_cut_phase(&self) -> (W, usize, usize) {
        let mut founds = Vec::from([0]);
        let mut cut_weight = W::default();
        let mut candidates = (1..self.len()).collect::<Vec<_>>();
        for _ in 0..self.len() - 1 {
            let (max_next_idx, max_weight) = candidates
                .iter()
                .map(|&next| {
                    founds.iter().fold(W::default(), |weight_sum, &current| {
                        weight_sum + self.weight(next, current)
                    })
                })
                .enumerate()
                .max_by(|(_, lhs), (_, rhs)| lhs.cmp(rhs))
                .unwrap();

            founds.push(candidates.swap_remove(max_next_idx));
            cut_weight = max_weight;
        }

        (
            cut_weight,
            founds[founds.len() - 2],
            founds[founds.len() - 1],
        )
    }

    fn min_cut(mut self) -> Option<(W, Vec<L>)> {
        let mut merged = self
            .vertex_labels()
            .iter()
            .map(|&l| (l, vec![l]))
            .collect::<HashMap<_, _>>();
        let mut best = None;
        let n = self.len();
        for _ in 0..n - 1 {
            let (weight, s, t) = self.min_cut_phase();
            let t_labels = merged.remove(self.label(t)).unwrap();

            if best
                .as_ref()
                .map(|(min_weight, _)| &weight < min_weight)
                .unwrap_or(true)
            {
                best = Some((weight, t_labels.clone()));
            }

            merged
                .get_mut(self.label(s))
                .unwrap()
                .extend(t_labels.into_iter());
            self.merge(s, t);
        }

        best
    }
}

mod adjacency_matrix {
    use super::{Graph, Merge};
    use std::{collections::HashMap, hash::Hash, ops::AddAssign};

    /// `Graph` implementation using an adjacency matrix
    pub struct AdjacencyMatrix<W, L> {
        pub(crate) matrix: Vec<Vec<W>>,
        pub(crate) labels: Vec<L>,
    }

    impl<W, L, T> From<T> for AdjacencyMatrix<W, L>
    where
        W: Copy + Default,
        L: Copy + Eq + Hash,
        T: IntoIterator<Item = ((L, L), W)>,
    {
        /// construct adjacency matrix from iterator over edges represented
        /// by tuples `((s, t), w): ((L, L), W)`
        fn from(value: T) -> Self {
            let mut indices = HashMap::new();
            let mut labels = Vec::new();
            let mut adjacents = Vec::new();

            for ((s, t), w) in value.into_iter() {
                let s_idx = *indices.entry(s).or_insert(labels.len());
                adjacents.resize(adjacents.len().max(s_idx + 1), Vec::new());
                labels.resize(labels.len().max(s_idx + 1), s);

                let t_idx = *indices.entry(t).or_insert(adjacents.len());
                adjacents.resize(adjacents.len().max(t_idx + 1), Vec::new());
                labels.resize(labels.len().max(t_idx + 1), t);

                adjacents[s_idx].push((t_idx, w));
                adjacents[t_idx].push((s_idx, w));
            }

            let mut matrix = vec![vec![W::default(); labels.len()]; labels.len()];
            for (k1, adj) in adjacents.into_iter().enumerate() {
                for (k2, w) in adj {
                    matrix[k1][k2] = w;
                }
            }

            Self { matrix, labels }
        }
    }

    impl<W, L> Graph for AdjacencyMatrix<W, L>
    where
        W: Copy + Eq + Default,
        L: PartialEq,
    {
        type Weight = W;
        type Label = L;

        fn vertex_labels(&self) -> &[Self::Label] {
            &self.labels
        }

        fn index_for_label(&self, label: &Self::Label) -> Option<usize> {
            self.labels.iter().position(|l| l == label)
        }

        fn len(&self) -> usize {
            self.matrix.len()
        }

        fn weight(&self, s_idx: usize, t_idx: usize) -> Self::Weight {
            self.matrix[s_idx][t_idx]
        }
    }

    impl<W, L> AdjacencyMatrix<W, L> {
        fn remove(&mut self, idx: usize) {
            self.matrix.swap_remove(idx);
            for row in self.matrix.iter_mut() {
                row.swap_remove(idx);
            }
            self.labels.swap_remove(idx);
        }
    }

    impl<W, L> Merge for AdjacencyMatrix<W, L>
    where
        W: AddAssign + Copy + Eq + Default,
        L: PartialEq,
    {
        fn merge(&mut self, s: usize, t: usize) {
            for k in (0..self.matrix.len()).filter(|&k| k != s && k != t) {
                let w = self.matrix[t][k];
                self.matrix[s][k] += w;
                let w = self.matrix[k][t];
                self.matrix[k][s] += w;
            }

            self.remove(t);
        }
    }
}

mod adjacency_list {
    use std::{collections::HashMap, hash::Hash, ops::AddAssign};

    use super::{Graph, Merge};

    pub struct AdjacencyList<W, L> {
        pub(crate) adjacents: Vec<Vec<(usize, W)>>,
        pub(crate) labels: Vec<L>,
    }

    impl<W, L, T> From<T> for AdjacencyList<W, L>
    where
        W: Copy,
        L: Copy + Eq + Hash,
        T: IntoIterator<Item = ((L, L), W)>,
    {
        fn from(value: T) -> Self {
            let mut indices = HashMap::new();
            let mut labels = Vec::new();
            let mut adjacents = Vec::new();

            for ((s, t), w) in value.into_iter() {
                let s_idx = *indices.entry(s).or_insert(labels.len());
                adjacents.resize(adjacents.len().max(s_idx + 1), Vec::new());
                labels.resize(labels.len().max(s_idx + 1), s);

                let t_idx = *indices.entry(t).or_insert(adjacents.len());
                adjacents.resize(adjacents.len().max(t_idx + 1), Vec::new());
                labels.resize(labels.len().max(t_idx + 1), t);

                adjacents[s_idx].push((t_idx, w));
                adjacents[t_idx].push((s_idx, w));
            }

            Self { adjacents, labels }
        }
    }

    impl<W, L> Graph for AdjacencyList<W, L>
    where
        W: Copy + Default,
        L: PartialEq,
    {
        type Weight = W;
        type Label = L;

        fn vertex_labels(&self) -> &[Self::Label] {
            &self.labels
        }

        fn index_for_label(&self, label: &Self::Label) -> Option<usize> {
            self.labels.iter().position(|l| l == label)
        }

        fn len(&self) -> usize {
            self.labels.len()
        }

        fn weight(&self, s_idx: usize, t_idx: usize) -> Self::Weight {
            self.adjacents[s_idx]
                .iter()
                .find(|(idx, _)| idx == &t_idx)
                .map(|(_, w)| *w)
                .unwrap_or_default()
        }
    }

    impl<W, L> AdjacencyList<W, L> {
        fn remove(&mut self, idx: usize) -> Vec<(usize, W)> {
            // remove row
            let mut removed = self.adjacents.swap_remove(idx);
            let n = self.adjacents.len();

            // remove edges from other ends
            for (s_idx, _) in removed.iter_mut() {
                if *s_idx == n {
                    // last was swapped in place of idx
                    *s_idx = idx;
                }
                let k = self.adjacents[*s_idx]
                    .iter()
                    .position(|(i, _)| i == &idx)
                    .unwrap();
                self.adjacents[*s_idx].swap_remove(k);
            }

            // update indices for node that was swapped in place of idx
            if idx < n {
                for k in 0..self.adjacents[idx].len() {
                    let s_idx = self.adjacents[idx][k].0;
                    let (t_idx, _) = self.adjacents[s_idx]
                        .iter_mut()
                        .find(|(i, _)| i == &n)
                        .unwrap();
                    *t_idx = idx;
                }
            }

            // remove label
            self.labels.swap_remove(idx);

            removed
        }
    }

    impl<W, L> Merge for AdjacencyList<W, L>
    where
        W: Copy + AddAssign,
    {
        fn merge(&mut self, s: usize, t: usize) {
            let removed = self.remove(t);
            let s = if s == self.adjacents.len() { t } else { s };
            for (t_idx, w) in removed {
                if let Some(k) = self.adjacents[s].iter().position(|(i, _)| i == &t_idx) {
                    // update edge weights if they exist s
                    let (_, w0) = &mut self.adjacents[s][k];
                    *w0 += w;
                    let (_, w0) = self.adjacents[t_idx]
                        .iter_mut()
                        .find(|(i, _)| i == &s)
                        .unwrap();
                    *w0 += w;
                } else if t_idx != s {
                    // insert new edges if they do not exist for s
                    self.adjacents[s].push((t_idx, w));
                    self.adjacents[t_idx].push((s, w));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{adjacency_list::*, adjacency_matrix::*, Graph, Merge, MinCut};
    use std::{collections::HashSet, fmt::Debug, hash::Hash};

    //  1-(2)-2-(3)-3-(4)-4
    //  |   / |     |   / |
    // (3)(2)(2)   (2)(2)(2)
    //  | /   |     | /   |
    //  5-(3)-6-(1)-7-(3)-8
    const EDGES: [((usize, usize), u64); 12] = [
        ((1, 2), 2),
        ((2, 3), 3),
        ((3, 4), 4),
        ((1, 5), 3),
        ((2, 5), 2),
        ((2, 6), 2),
        ((3, 7), 2),
        ((4, 7), 2),
        ((4, 8), 2),
        ((5, 6), 3),
        ((6, 7), 1),
        ((7, 8), 3),
    ];

    pub fn do_test_min_cut_phase<G>(g: G)
    where
        G: MinCut<u64, usize> + Graph<Weight = u64, Label = usize>,
    {
        // (1) -> 5 @ 3
        // (1, 5) -> 2 @ 4
        // (1, 2, 5) -> 6 @ 5
        // (1, 2, 5, 6) -> 3 @ 3
        // (1, 2, 3, 5, 6) -> 4 @ 4
        // (1, 2, 3, 4, 5, 6) -> 7 @ 5
        // (1, 2, 3, 4, 5, 6, 7) -> 8 @ 5

        let s = g.index_for_label(&7).unwrap();
        let t = g.index_for_label(&8).unwrap();
        assert_eq!((5, s, t), g.min_cut_phase());
    }

    #[test]
    pub fn test_min_cut_phase() {
        do_test_min_cut_phase(AdjacencyMatrix::from(EDGES));
        do_test_min_cut_phase(AdjacencyList::from(EDGES));
    }

    pub fn do_test_merge<G>(mut g: G)
    where
        G: Merge + Graph<Weight = u64, Label = usize>,
    {
        //  1-(2)-2-(3)-3-(4)-4             2-(3)-3-(4)-4
        //  |   / |     |   / |           / |     |   / |
        // (3)(2)(2)   (2)(2)(2)  =>    (4)(2)   (2)(2)(2)
        //  | /   |     | /   |         /   |     | /   |
        //  5-(3)-6-(1)-7-(3)-8       5-(3)-6-(1)-7-(3)-8
        let s = g.index_for_label(&5).unwrap();
        let t = g.index_for_label(&1).unwrap();
        g.merge(s, t);
        assert_eq!(7, g.len());
        let weights = [2, 3, 4, 5, 6, 7, 8]
            .into_iter()
            .filter_map(|l| g.index_for_label(&l))
            .map(|k| g.weight(k, 4))
            .collect::<Vec<_>>();
        assert_eq!(vec![4, 0, 0, 0, 3, 0, 0], weights);

        //        2-(3)-3-(4)-4             2-(3)-3-(6)-4
        //      / |     |   / |           / |   /   \   |
        //    (4)(2)   (2)(2)(2)  =>    (4)(2)(1)   (3)(2)
        //    /   |     | /   |         /   | /       \ |
        //  5-(3)-6-(1)-7-(3)-8       5-(3)-6           8
        let s = g.index_for_label(&3).unwrap();
        let t = g.index_for_label(&7).unwrap();
        g.merge(s, t);
        assert_eq!(6, g.len());
        let weights = [2, 3, 4, 5, 6, 8]
            .into_iter()
            .filter_map(|l| g.index_for_label(&l))
            .map(|k| g.weight(k, 2))
            .collect::<Vec<_>>();
        assert_eq!(vec![3, 0, 6, 0, 1, 3], weights);
    }

    #[test]
    pub fn test_merge() {
        do_test_merge(AdjacencyMatrix::from(EDGES));
        do_test_merge(AdjacencyList::from(EDGES));
    }

    /// test minimum cut
    /// make sure minimum cut of g yields given weight an partitions
    fn do_test_min_cut<G, W, L>(g: G, exp_w: W, exp_pq: &HashSet<Vec<L>>)
    where
        G: MinCut<W, L> + Graph<Weight = W, Label = L>,
        W: Eq + Debug,
        L: Copy + Eq + Ord + Hash + Debug,
    {
        let mut ns = g.vertex_labels().iter().copied().collect::<Vec<_>>();
        ns.sort_unstable();

        let (w, mut p) = g.min_cut().unwrap();
        p.sort_unstable();
        let q = ns
            .into_iter()
            .filter(|n| !p.contains(n))
            .collect::<Vec<_>>();

        assert_eq!(exp_w, w);
        assert_eq!(exp_pq, &HashSet::from([p, q]));
    }

    #[test]
    pub fn test_min_cut_1() {
        let exp_pq = HashSet::from([vec![1, 2, 5, 6], vec![3, 4, 7, 8]]);

        // original graph
        do_test_min_cut(AdjacencyMatrix::from(EDGES), 4, &exp_pq);
        do_test_min_cut(AdjacencyList::from(EDGES), 4, &exp_pq);

        // changed representation
        // swap nodes within one partition
        let AdjacencyMatrix {
            mut matrix,
            mut labels,
        } = AdjacencyMatrix::from(EDGES);
        matrix.swap(0, 1);
        labels.swap(0, 1);
        for row in matrix.iter_mut() {
            row.swap(0, 1);
        }
        let g2 = AdjacencyMatrix { matrix, labels };
        do_test_min_cut(g2, 4, &exp_pq);

        // changed representation
        // swap nodes across partitions
        let AdjacencyMatrix {
            mut matrix,
            mut labels,
        } = AdjacencyMatrix::from(EDGES);
        matrix.swap(0, 7);
        labels.swap(0, 7);
        for row in matrix.iter_mut() {
            row.swap(0, 7);
        }
        let g3 = AdjacencyMatrix { matrix, labels };
        do_test_min_cut(g3, 4, &exp_pq);
    }

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

    pub fn adj_iter(data: &str) -> impl Iterator<Item = ((&str, &str), u64)> {
        data.lines().flat_map(|line| {
            let mut parts = line
                .split::<&[char]>(&[' ', ':'])
                .map(str::trim)
                .filter(|v| !v.is_empty());
            let key = parts.next().unwrap();
            parts.map(move |value| ((key, value), 1))
        })
    }

    #[test]
    pub fn test_min_cut_2() {
        let exp_pq = HashSet::from([
            vec!["bvb", "hfx", "jqt", "ntq", "rhn", "xhk"],
            vec![
                "cmg", "frs", "lhk", "lsr", "nvd", "pzl", "qnr", "rsh", "rzs",
            ],
        ]);
        do_test_min_cut(AdjacencyMatrix::from(adj_iter(CONTENT)), 3, &exp_pq);
        do_test_min_cut(AdjacencyList::from(adj_iter(CONTENT)), 3, &exp_pq);
    }
}
