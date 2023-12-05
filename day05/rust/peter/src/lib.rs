use input::*;
use std::{collections::HashMap, fs::read_to_string, iter::successors};

// tag::prelude[]
pub const IDENTIFIER: &str = "2023/04";

pub type SolT = i64;
// end::prelude[]

// tag::input[]
pub mod input {
    use crate::*;
    use std::collections::HashMap;

    #[derive(Debug)]
    pub struct PuzzleData {
        pub seeds: Vec<SolT>,
        pub maps: HashMap<String, (String, Vec<(SolT, SolT, SolT)>)>,
    }

    fn parse_ranges(ranges: &str) -> (SolT, SolT, SolT) {
        let mut numbers = ranges.split_ascii_whitespace().map(|r| r.parse().unwrap());
        (
            numbers.next().unwrap(),
            numbers.next().unwrap(),
            numbers.next().unwrap(),
        )
    }

    fn parse_group(group: &str) -> (String, (String, Vec<(SolT, SolT, SolT)>)) {
        let mut lines = group.lines();

        let mut header = lines.next().unwrap().split(&['-', ' ']);
        let (source, destination) = (
            header.next().unwrap().to_string(),
            header.nth(1).unwrap().to_string(),
        );

        (source, (destination, lines.map(parse_ranges).collect()))
    }

    impl<T: AsRef<str>> From<T> for PuzzleData {
        fn from(s: T) -> Self {
            let mut groups = s.as_ref().split("\n\n");
            Self {
                seeds: groups
                    .next()
                    .and_then(|group| group.strip_prefix("seeds: "))
                    .and_then(|seeds| {
                        seeds
                            .split_ascii_whitespace()
                            .map(|seed| seed.parse())
                            .collect::<Result<Vec<_>, _>>()
                            .ok()
                    })
                    .unwrap(),
                maps: groups.map(parse_group).collect(),
            }
        }
    }
}
// end::input[]

pub fn parse_input() -> PuzzleData {
    read_to_string("../../../inputs/input05").unwrap().into()
}

// tag::star_1[]
fn star<T, R, S, M>(
    maps: &HashMap<String, (String, Vec<(SolT, SolT, SolT)>)>,
    seeds: T,
    mut step: S,
    mut minimize: M,
) -> SolT
where
    R: ?Sized,
    T: AsRef<R>,
    S: FnMut(&R, &[(SolT, SolT, SolT)]) -> T,
    M: FnMut(T) -> Option<SolT>,
{
    successors(Some(("seed", seeds)), |(src, items)| {
        maps.get(*src)
            .map(|(dst, map)| (dst.as_str(), step(items.as_ref(), map)))
    })
    .last()
    .and_then(|(_, locations)| minimize(locations))
    .unwrap()
}

fn step_1(items: &[SolT], map: &[(SolT, SolT, SolT)]) -> Vec<i64> {
    items
        .iter()
        .map(|&item| {
            map.iter()
                .find(|(_, src_0, len)| (*src_0..*src_0 + len).contains(&item))
                .map(|(dst_0, src_0, _)| item + dst_0 - src_0)
                .unwrap_or(item)
        })
        .collect()
}

pub fn star_1(data: &PuzzleData) -> SolT {
    star(&data.maps, data.seeds.to_owned(), step_1, |items| {
        items.into_iter().min()
    })
}
// end::star_1[]

// tag::star_2[]
fn step_2(ranges: &[(SolT, SolT)], map: &[(SolT, SolT, SolT)]) -> Vec<(SolT, SolT)> {
    ranges.iter().fold(Vec::new(), |mut result, &rng| {
        let mut rng = Some(rng);
        while let Some((rng_0, rng_n)) = rng {
            // find a map range that overlaps the current range
            // return the transformed contained part and optionally
            // a residual untransformed range
            let (transformed, residual) = map
                .iter()
                .map(|&(dst_0, src_0, len)| (dst_0 - src_0, src_0, src_0 + len))
                .find(|&(_, src_0, src_n)| {
                    (src_0..src_n).contains(&rng_0) || (src_0..src_n).contains(&(rng_n - 1))
                })
                .map(|(dlt, src_0, src_n)| {
                    match (
                        (src_0..src_n).contains(&rng_0),
                        (src_0..src_n).contains(&(rng_n - 1)),
                    ) {
                        (true, false) => ((rng_0 + dlt, src_n + dlt), Some((src_n, rng_n))),
                        (false, true) => ((src_0 + dlt, rng_n + dlt), Some((rng_0, src_0))),
                        _ => ((rng_0 + dlt, rng_n + dlt), None),
                    }
                })
                .unwrap_or(((rng_0, rng_n), None));

            result.push(transformed);
            rng = residual;
        }
        result
    })
}

pub fn star_2(data: &PuzzleData) -> SolT {
    star(
        &data.maps,
        data.seeds
            .iter()
            .step_by(2)
            .zip(data.seeds.iter().skip(1).step_by(2))
            .map(|(&from, &len)| (from, from + len))
            .collect::<Vec<_>>(),
        step_2,
        |rng| rng.into_iter().map(|(start, _)| start).min(),
    )
}
// end::star_2[]

// tag::tests[]
#[cfg(test)]
mod tests {
    use super::*;

    const CONTENT: &str = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
"#;

    #[test]
    pub fn test_from() {
        let data = PuzzleData::from(CONTENT);

        println!("{data:?}");

        // check seeds
        assert_eq!(vec![79, 14, 55, 13], data.seeds);

        // check that map finds way from seed to location
        let mut current = "seed";
        while let Some((next, _)) = data.maps.get(current) {
            current = next;
        }
        assert_eq!("location", current);

        // check one map entry
        assert_eq!(
            Some(&("light".to_string(), vec![(88, 18, 7), (18, 25, 70)])),
            data.maps.get("water")
        );
    }

    #[test]
    pub fn test_star_1() {
        assert_eq!(35, star_1(&CONTENT.into()));
    }

    #[test]
    pub fn test_star_2() {
        assert_eq!(46, star_2(&CONTENT.into()));
    }
}
// end::tests[]
