use std::{
    collections::{BinaryHeap, VecDeque},
    fs::read_to_string,
};

// tag::prelude[]
pub const IDENTIFIER: &str = "2023/21";

pub type InputT<'a> = Grid<'a>;

pub fn read_input() -> String {
    read_to_string("../../../inputs/input21").unwrap()
}
// end::prelude[]

// tag::solution[]
pub type Steps = u32;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Heading {
    East = 0,
    North = 1,
    West = 2,
    South = 3,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Boundary {
    offset: usize,
    heading: Heading,
    data: Vec<Steps>,
}

impl Boundary {
    pub fn from_matrix(
        data: &[Steps],
        w: usize,
        h: usize,
        heading: Heading,
        offset: usize,
    ) -> Self {
        let mut data: Vec<_> = match heading {
            Heading::East => (0..h)
                .map(|row_inv| data[w - 1 + (h - 1 - row_inv) * w])
                .collect(),
            Heading::North => (0..w).map(|col_inv| data[w - 1 - col_inv]).collect(),
            Heading::West => (0..h).map(|row| data[row * w]).collect(),
            Heading::South => (0..w).map(|col| data[col + (h - 1) * w]).collect(),
        };

        let min_cost = *data.iter().min().unwrap();
        let offset = if min_cost == Steps::MAX {
            usize::MAX
        } else {
            data.iter_mut()
                .filter(|d| **d != Steps::MAX)
                .for_each(|d| *d -= min_cost);

            offset + min_cost as usize
        };

        Self {
            data,
            heading,
            offset,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Grid<'a> {
    pub(crate) data: &'a [u8],
    pub(crate) w: usize,
    pub(crate) h: usize,
}

mod grid {
    use super::Steps;

    pub(crate) fn cost_counts(costs: &[Steps]) -> Vec<usize> {
        costs.iter().filter(|&&cost| cost != Steps::MAX).fold(
            Vec::<usize>::new(),
            |mut counts, &cost| {
                counts.resize(counts.len().max(cost as usize + 1), 0);
                counts[cost as usize] += 1;
                counts
            },
        )
    }

    pub(crate) fn count_reachable(costs: &[Steps], offset: usize, steps: usize) -> usize {
        costs
            .iter()
            .map(|&cost| offset + cost as usize)
            .filter(|&cost| cost <= steps && cost & 1 == steps & 1)
            .count()
    }
}

impl<'a, T> From<&'a T> for Grid<'a>
where
    T: AsRef<[u8]> + 'a + ?Sized,
{
    fn from(value: &'a T) -> Self {
        let data = value.as_ref();
        let w = data.iter().position(|&b| b == b'\n').unwrap_or(data.len());
        let h = (data.len() + 1) / (w + 1);
        Self { data, w, h }
    }
}

impl Grid<'_> {
    pub fn get_start(&self) -> (usize, usize) {
        self.data
            .iter()
            .position(|&b| b == b'S')
            .map(|p| (p % (self.w + 1), p / (self.w + 1)))
            .unwrap()
    }

    pub fn reachable_in_steps(&self, steps: usize) -> usize {
        assert!(self.w == self.h);

        let (mut total_count, boundaries) = self.center_tile_boundaries(steps);

        for boundary in boundaries
            .as_ref()
            .iter()
            .filter(|boundary| boundary.offset != usize::MAX && boundary.data[0] != Steps::MAX)
        {
            let offset = boundary.offset + boundary.data[0] as usize + 1;
            let data = (0..boundary.data.len() as Steps).rev().collect();
            let boundary = Boundary {
                data,
                offset,
                heading: boundary.heading,
            };
            let (_, costs) = self.next_boundary(&boundary, usize::MAX);
            let cost_counts = grid::cost_counts(&costs);

            // fits if offset + (s - 1) * self.w + max_steps <= steps
            // (s - 1) * self.w <= (steps - offset - max_steps)
            let n = (steps + self.w).saturating_sub(offset + cost_counts.len() - 1) / self.w;

            // short cut as long as max_step fits
            if n > 0 {
                let cost_counts_even = cost_counts.iter().step_by(2).skip(1).sum::<usize>();
                let cost_counts_odd = cost_counts.iter().skip(1).step_by(2).sum::<usize>();

                // sum of even numbers from 1 to n
                let sum_even = 2 * (n / 2 + 1) * (n / 2) / 2;
                // sum of odd numbers from 1 to n
                let sum_odd = 2 * ((n + 1) / 2 + 1) * ((n + 1) / 2) / 2 - (n + 1) / 2;

                total_count += if (steps - offset) & 1 == 0 {
                    sum_odd * cost_counts_even + sum_even * cost_counts_odd
                } else {
                    sum_odd * cost_counts_odd + sum_even * cost_counts_even
                };
            }

            // do the counting for the remaining tiles
            for s in n + 1.. {
                let off = offset + (s - 1) * self.w;
                let count = grid::count_reachable(&costs, off, steps);
                total_count += s * count;

                if count == 0 {
                    break;
                }
            }
        }

        for boundary in boundaries
            .into_iter()
            .filter(|boundary| boundary.offset != usize::MAX)
        {
            // find steady state
            let mut cur = boundary;
            let steady_state = loop {
                let (next, costs) = self.next_boundary(&cur, steps);
                total_count += grid::count_reachable(&costs, cur.offset, steps);

                let Some(next) = next else { break None };
                if cur.data == next.data {
                    break Some((next, costs));
                }
                cur = next;
            };

            let Some((boundary, costs)) = steady_state else {
                continue;
            };
            let offset = boundary.offset;

            let cost_counts = grid::cost_counts(&costs);

            // fits if offset + (s - 1) * self.w + max_steps <= steps
            // (s - 1) * self.w <= (steps - offset - max_steps)
            let n = (steps + self.w).saturating_sub(offset + cost_counts.len() - 1) / self.w;

            if n > 0 {
                let cost_counts_even = cost_counts.iter().step_by(2).skip(1).sum::<usize>();
                let cost_counts_odd = cost_counts.iter().skip(1).step_by(2).sum::<usize>();

                total_count += if (steps - offset) & 1 == 0 {
                    (n + 1) / 2 * cost_counts_even + (n / 2) * cost_counts_odd
                } else {
                    (n + 1) / 2 * cost_counts_odd + (n / 2) * cost_counts_even
                };
            }

            for s in n + 1.. {
                let count = grid::count_reachable(&costs, offset + (s - 1) * self.w, steps);
                total_count += count;
                if count == 0 {
                    break;
                }
            }
        }

        total_count
    }

    pub fn next_boundary(
        &self,
        boundary: &Boundary,
        steps: usize,
    ) -> (Option<Boundary>, Vec<Steps>) {
        let (off_c, fac_c, off_r, fac_r) = match boundary.heading {
            Heading::East => (0, 0, self.h - 1, -1),
            Heading::North => (self.w - 1, -1, self.h - 1, 0),
            Heading::West => (self.w - 1, 0, 0, 1),
            Heading::South => (0, 1, 0, 0),
        };
        let (mut queue, mut costs) = boundary
            .data
            .iter()
            .enumerate()
            .map(|(v, &d)| {
                (
                    d.saturating_add(1),
                    (
                        off_c.wrapping_add_signed(fac_c * v as isize),
                        off_r.wrapping_add_signed(fac_r * v as isize),
                    ),
                )
            })
            .fold(
                (BinaryHeap::new(), vec![Steps::MAX; self.w * self.h]),
                |(mut heap, mut costs), (cost, (col, row))| {
                    heap.push((!cost, (col, row)));
                    costs[col + row * self.w] = cost;
                    (heap, costs)
                },
            );

        while let Some((cost, (col, row))) = queue.pop() {
            if boundary.offset == usize::MAX || boundary.offset + !cost as usize >= steps {
                break;
            }

            // add adjacents
            for (col, row) in [(1, 0), (0, -1), (-1, 0), (0, 1)]
                .into_iter()
                .map(|(dc, dr)| (col.wrapping_add_signed(dc), row.wrapping_add_signed(dr)))
                .filter(|&(col, row)| {
                    (col < self.w && row < self.h) && self.data[col + row * (self.w + 1)] != b'#'
                })
            {
                if costs[col + row * self.w] == Steps::MAX {
                    costs[col + row * self.w] = !cost + 1;
                    queue.push((cost - 1, (col, row)));
                }
            }
        }

        let next = Boundary::from_matrix(&costs, self.w, self.h, boundary.heading, boundary.offset);

        ((next.offset != usize::MAX).then_some(next), costs)
    }

    pub fn center_tile_costs(&self, steps: Steps) -> (Steps, Vec<Steps>) {
        let start = self.get_start();
        let mut costs = vec![Steps::MAX; self.w * self.h];
        let mut queue = VecDeque::new();
        costs[start.0 + start.1 * self.w] = 0;
        queue.push_back((0, start));

        while let Some((cost, (col, row))) = queue.pop_front() {
            if cost >= steps {
                break;
            }

            for (col, row) in [(1, 0), (0, -1), (-1, 0), (0, 1)]
                .into_iter()
                .map(|(dc, dr)| (col.wrapping_add_signed(dc), row.wrapping_add_signed(dr)))
                .filter(|&(col, row)| {
                    (col < self.w && row < self.h) && self.data[col + row * (self.w + 1)] != b'#'
                })
            {
                if costs[col + row * self.w] == Steps::MAX {
                    costs[col + row * self.w] = cost + 1;
                    queue.push_back((cost + 1, (col, row)));
                }
            }
        }

        let count = costs
            .iter()
            .filter(|&&c| c <= steps && (c & 1) == (steps & 1))
            .count();
        (count as _, costs)
    }

    pub fn center_tile_boundaries(&self, steps: usize) -> (usize, [Boundary; 4]) {
        let (count, costs) = self.center_tile_costs(steps.min(Steps::MAX as _) as _);

        (
            count as _,
            [
                Boundary::from_matrix(&costs, self.w, self.h, Heading::East, 0),
                Boundary::from_matrix(&costs, self.w, self.h, Heading::North, 0),
                Boundary::from_matrix(&costs, self.w, self.h, Heading::West, 0),
                Boundary::from_matrix(&costs, self.w, self.h, Heading::South, 0),
            ],
        )
    }
}
// end::solution[]

pub fn star_1(grid: &Grid) -> Steps {
    grid.center_tile_costs(64).0
}

pub fn star_2(grid: &Grid) -> usize {
    grid.reachable_in_steps(26_501_365)
}

// tag::tests[]
#[cfg(test)]
mod tests {
    use super::*;

    const CONTENT: &str = r#"...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........
"#;

    #[test]
    pub fn test_center_tile_costs() {
        let grid = Grid::from(CONTENT);

        let (count, _) = grid.center_tile_costs(6);

        assert_eq!(16, count);
    }

    #[test]
    pub fn test_boundary_from_matrix() {
        // 2 1 1
        // 2   0
        // 3 3 0
        let data = [2, 1, 1, 2, 99, 0, 3, 3, 0];

        for heading in [Heading::East, Heading::North, Heading::West, Heading::South] {
            let boundary = Boundary::from_matrix(&data, 3, 3, heading, 7);
            let (data, offset) = match heading {
                Heading::South => (vec![3, 3, 0], 7),
                heading => (vec![0, 0, 1], 7 + heading as usize),
            };
            assert_eq!(
                Boundary {
                    data,
                    heading,
                    offset
                },
                boundary
            );
        }
    }

    pub fn do_test_next_boundary(grid: Grid) {
        for (heading, a, offset) in [
            (Heading::East, grid.h, grid.w),
            (Heading::North, grid.w, grid.h),
            (Heading::West, grid.h, grid.w),
            (Heading::South, grid.w, grid.h),
        ] {
            let boundary = Boundary {
                data: (0..a as Steps).collect(),
                heading,
                offset: 0,
            };
            let next = grid.next_boundary(&boundary, usize::MAX);
            assert_eq!(Some(Boundary { offset, ..boundary }), next.0);
        }
    }

    #[test]
    pub fn test_next_boundary() {
        do_test_next_boundary(CONTENT.into());
        let input = read_input();
        do_test_next_boundary((&input).into());
    }

    #[test]
    pub fn test_reachable_in_steps() {
        let grid = Grid::from(CONTENT);
        for (exp, steps) in [
            (16, 6),
            (50, 10),
            (1_594, 50),
            (6_536, 100),
            (167_004, 500),
            (668_697, 1000),
            (16_733_044, 5000),
        ] {
            println!("{}", steps);
            assert_eq!(exp, grid.reachable_in_steps(steps));
        }
    }

    pub fn multi_grid<'a>(grid: &Grid, n: usize) -> (Vec<u8>, usize, usize) {
        let w = n * grid.w;
        let h = n * grid.h;
        let mut data = vec![b'\n'; (w + 1) * h];
        for row in 0..grid.h {
            for col in 0..grid.w {
                let d = grid.data[col + row * (grid.w + 1)];
                for y in 0..n {
                    for x in 0..n {
                        let d = match d {
                            b'S' if x != n / 2 || y != n / 2 => b'.',
                            b => b,
                        };
                        data[(col + x * grid.w) + (row + y * grid.h) * (w + 1)] = d;
                    }
                }
            }
        }

        (data, w, h)
    }

    #[test]
    pub fn test_compare() {
        let n = 7;
        let data = read_input();
        let grid0: Grid = (&data).into();
        let (data, w, h) = multi_grid(&grid0, n);
        let grid1 = Grid { data: &data, w, h };

        for steps in (40..=400).step_by(40) {
            let (count1, costs) = grid1.center_tile_costs(steps);
            let enough_space = (0..grid1.w)
                .map(|col| (col, 0))
                .chain((0..grid1.h).map(|row| (grid1.w - 1, row)))
                .chain((0..grid1.w).map(|col_inv| (grid1.w - 1 - col_inv, grid1.h - 1)))
                .chain((0..grid1.h).map(|row_inv| (0, grid1.h - 1 - row_inv)))
                .map(|(col, row)| costs[col + row * grid1.w])
                .filter(|&d| d < Steps::MAX)
                .count();
            assert_eq!(0, enough_space);

            let count0 = grid0.reachable_in_steps(steps as usize);

            assert_eq!(count1 as usize, count0);
        }
    }
}
// end::tests[]
