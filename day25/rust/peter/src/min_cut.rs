use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    hash::Hash,
    ops::AddAssign,
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
}

pub trait Weight: Graph {
    /// Get weight of edge between two vertices at given indices.
    fn weight(&self, s_idx: usize, t_idx: usize) -> Self::Weight;
}

pub trait Adjacents: Graph {
    /// Type of adjacents iterator
    type AdjIt<'a>: Iterator<Item = (usize, Self::Weight)>
    where
        Self::Weight: 'a,
        Self: 'a;

    /// Get iterator over adjacents of node at given index
    fn adjacents(&self, idx: usize) -> Self::AdjIt<'_>;
}

/// Implementation of the Stoer-Wagner algorithm to find a minimum cut
/// of a [`Graph`].
///
/// See <https://blog.thomasjungblut.com/graph/mincut/mincut/>
/// See <https://en.wikipedia.org/wiki/Stoer%E2%80%93Wagner_algorithm>
pub trait MinCut<W, L> {
    /// Perform a minimum cut phase.
    ///
    /// Return the weight of the cut and the indices of the last two
    /// vertices, `s` and `t`
    fn min_cut_phase(&self) -> (W, usize, usize);

    /// Perform minimum cut. Stop early if bound yields true.
    ///
    /// Return the weight of the minimum cut and a list of vertex
    /// labels in one of the partitions.
    ///
    /// There is no guarantee on which of the partitions is returned.
    fn min_cut_with_bound<F>(self, bound: F) -> Option<(W, Vec<L>)>
    where
        F: Fn(&(W, Vec<L>)) -> bool;

    /// Perform minimum cut.
    ///
    /// Equivalent to [`Self::min_cut_with_bound`] with a bound function
    /// that always yields `false`.
    fn min_cut(self) -> Option<(W, Vec<L>)>
    where
        Self: Sized,
    {
        self.min_cut_with_bound(|_| false)
    }
}

pub trait Merge {
    /// Merge `t` into `s`. This will result in deleting `t`.
    fn merge(&mut self, s: usize, t: usize);
}

impl<T, W, L> MinCut<W, L> for T
where
    T: Adjacents<Weight = W, Label = L> + Merge,
    W: Copy + Ord + AddAssign + Default,
    L: Copy + Eq + Hash,
{
    fn min_cut_phase(&self) -> (W, usize, usize) {
        let mut seen = HashSet::from([0]);
        let mut weights = vec![W::default(); self.len()];
        let mut queue = BinaryHeap::new();
        for (idx, w) in self.adjacents(0) {
            weights[idx] = w;
            queue.push((w, idx));
        }

        let (mut w, mut s, mut t) = (W::default(), 0, 0);
        while let Some((cur_w, cur_idx)) = queue.pop() {
            if !seen.insert(cur_idx) {
                continue;
            }

            (w, s, t) = (cur_w, t, cur_idx);
            if seen.len() == self.len() {
                break;
            }

            for (next_idx, next_w) in self
                .adjacents(cur_idx)
                .filter(|(idx, _)| !seen.contains(idx))
            {
                let w = &mut weights[next_idx];
                *w += next_w;
                queue.push((*w, next_idx));
            }
        }
        (w, s, t)
    }

    fn min_cut_with_bound<F>(mut self, bound: F) -> Option<(W, Vec<L>)>
    where
        F: Fn(&(W, Vec<L>)) -> bool,
    {
        let mut merged = self
            .vertex_labels()
            .iter()
            .map(|&l| (l, vec![l]))
            .collect::<HashMap<_, _>>();
        let mut best = None;
        for _ in 0..self.len().saturating_sub(1) {
            let (w, s_idx, t_idx) = self.min_cut_phase();
            let t_labels = merged.remove(self.label(t_idx)).unwrap();

            if best.as_ref().map(|(min_w, _)| &w < min_w).unwrap_or(true) {
                best = Some((w, t_labels.clone()));
            }

            merged
                .get_mut(self.label(s_idx))
                .unwrap()
                .extend(t_labels.into_iter());
            self.merge(s_idx, t_idx);

            if best.as_ref().map(&bound).unwrap_or_default() {
                break;
            }
        }

        best
    }
}

pub mod adjacency_matrix {
    use super::{Adjacents, Graph, Merge, Weight};
    use std::{
        collections::HashMap,
        hash::Hash,
        iter::{Copied, Enumerate, Filter},
        ops::AddAssign,
        slice::Iter,
    };

    #[derive(Debug, Clone)]
    /// `Graph` implementation using an adjacency matrix
    pub struct AdjacencyMatrix<W, L> {
        pub(crate) matrix: Vec<Vec<W>>,
        pub(crate) labels: Vec<L>,
    }

    impl<W, L> FromIterator<((L, L), W)> for AdjacencyMatrix<W, L>
    where
        W: Copy + Default,
        L: Copy + Eq + Hash,
    {
        fn from_iter<T: IntoIterator<Item = ((L, L), W)>>(iter: T) -> Self {
            iter.into()
        }
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
    }

    impl<W, L> Weight for AdjacencyMatrix<W, L>
    where
        W: Copy + Eq + Default,
        L: PartialEq,
    {
        fn weight(&self, s_idx: usize, t_idx: usize) -> Self::Weight {
            self.matrix[s_idx][t_idx]
        }
    }

    type AdjPred<W> = for<'a> fn(&'a (usize, W)) -> bool;

    impl<W, L> Adjacents for AdjacencyMatrix<W, L>
    where
        W: Copy + Eq + Default,
        L: PartialEq,
    {
        type AdjIt<'a> = Filter<Enumerate<Copied<Iter<'a, W>>>, AdjPred<W>> where W: 'a, L: 'a;

        fn adjacents(&self, idx: usize) -> Self::AdjIt<'_> {
            self.matrix[idx]
                .iter()
                .copied()
                .enumerate()
                .filter(|(_, w)| w != &W::default())
        }
    }

    impl<W, L> AdjacencyMatrix<W, L> {
        pub fn remove(&mut self, idx: usize) {
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

pub mod adjacency_list {
    use super::{Adjacents, Graph, Merge, Weight};
    use std::{collections::HashMap, hash::Hash, iter::Copied, ops::AddAssign, slice::Iter};

    #[derive(Debug, Clone)]
    pub struct AdjacencyList<W, L> {
        pub(crate) adjacents: Vec<Vec<(usize, W)>>,
        pub(crate) labels: Vec<L>,
    }

    impl<W, L> FromIterator<((L, L), W)> for AdjacencyList<W, L>
    where
        W: Copy,
        L: Copy + Eq + Hash,
    {
        /// See [`AdjacencyList::from`]
        fn from_iter<T: IntoIterator<Item = ((L, L), W)>>(iter: T) -> Self {
            iter.into()
        }
    }

    impl<W, L, T> From<T> for AdjacencyList<W, L>
    where
        W: Copy,
        L: Copy + Eq + Hash,
        T: IntoIterator<Item = ((L, L), W)>,
    {
        /**
         * Create an adjacency list from an iterator over edges.
         *
         * It is an error if the iterator yields two edge connecting the
         * same vertices or an edge connecting a vertex to itself. This
         * will result in an inconsistent adjacency list.
         *
         * You can verify consistency with [`AdjacencyList::consistent`]
         */
        fn from(value: T) -> Self {
            let it = value.into_iter();
            let mut indices = HashMap::new();
            let mut labels = Vec::with_capacity(it.size_hint().0);
            let mut adjacents = Vec::with_capacity(it.size_hint().0);

            for ((s, t), w) in it {
                let s_idx = *indices.entry(s).or_insert(labels.len());
                adjacents.resize(adjacents.len().max(s_idx + 1), Vec::new());
                labels.resize(labels.len().max(s_idx + 1), s);

                let t_idx = *indices.entry(t).or_insert(labels.len());
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
    }

    impl<W, L> Weight for AdjacencyList<W, L>
    where
        W: Copy + Default,
        L: PartialEq,
    {
        fn weight(&self, s_idx: usize, t_idx: usize) -> Self::Weight {
            self.adjacents[s_idx]
                .iter()
                .find(|(idx, _)| idx == &t_idx)
                .map(|(_, w)| *w)
                .unwrap_or_default()
        }
    }

    impl<W, L> Adjacents for AdjacencyList<W, L>
    where
        W: Copy + Default,
        L: PartialEq,
    {
        type AdjIt<'a> = Copied<Iter<'a, (usize, W)>> where W: 'a, L: 'a;

        fn adjacents(&self, idx: usize) -> Self::AdjIt<'_> {
            self.adjacents[idx].iter().copied()
        }
    }

    impl<W, L> AdjacencyList<W, L> {
        pub fn remove(&mut self, idx: usize) -> Vec<(usize, W)> {
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

        /**
         * Return a consistent adjacency list as `Ok` value or the labels of a
         * duplicate edge or a self-referential edge in an `Err` value. The
         * second label is `None`, if the edge is self-referential.
         *
         * # Examples
         * ```
         * # use mr_kaffee_2023_25::min_cut::adjacency_list::AdjacencyList;
         *
         * let edges = [(("a", "b"), 1), (("b", "c"), 1), (("c", "a"), 2)];
         * let graph = AdjacencyList::from(edges).consistent().unwrap();
         *
         * let edges = [(("a", "b"), 1), (("b", "c"), 1), (("b", "a"), 2)];
         * let duplicate = AdjacencyList::from(edges).consistent().unwrap_err();
         * assert!(("a", Some("b")) == duplicate || ("b", Some("a")) == duplicate);
         *
         * let edges = [(("a", "b"), 1), (("b", "c"), 1), (("a", "a"), 2)];
         * let duplicate = AdjacencyList::from(edges).consistent().unwrap_err();
         * assert!(("a", None) == duplicate);
         * ```
         */
        pub fn consistent(mut self) -> Result<Self, (L, Option<L>)> {
            match self
                .adjacents
                .iter()
                .enumerate()
                .find_map(|(s_idx, adjacents)| {
                    adjacents
                        .iter()
                        .enumerate()
                        .find(|&(k, (t_idx, _))| {
                            adjacents[0..k].iter().any(|(idx, _)| idx == t_idx)
                        })
                        .map(|(_, &(t_idx, _))| (s_idx.min(t_idx), s_idx.max(t_idx)))
                        .map(|(s_idx, t_idx)| {
                            (
                                self.labels.swap_remove(t_idx),
                                if s_idx == t_idx {
                                    None
                                } else {
                                    Some(self.labels.swap_remove(s_idx))
                                },
                            )
                        })
                }) {
                Some(val) => Err(val),
                None => Ok(self),
            }
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
    use super::{adjacency_list::*, adjacency_matrix::*, Graph, Merge, MinCut, Weight};
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
        G: Merge + Weight<Weight = u64, Label = usize>,
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
