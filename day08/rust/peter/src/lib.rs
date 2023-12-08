use input::*;
use mr_kaffee_utils::euclid::gcd;
use std::{collections::HashMap, fs::read_to_string, iter::successors};

// tag::prelude[]
pub const IDENTIFIER: &str = "2023/08";

pub type SolT = usize;
pub type InputT<'a> = PuzzleData<'a>;

pub fn read_input() -> String {
    read_to_string("../../../inputs/input08").unwrap()
}
// end::prelude[]

// tag::input[]
pub mod input {
    use std::collections::HashMap;

    #[derive(Debug, PartialEq, Eq)]
    pub struct PuzzleData<'a>(pub &'a [u8], pub HashMap<&'a str, (&'a str, &'a str)>);

    impl<'a, T> From<&'a T> for PuzzleData<'a>
    where
        T: AsRef<str> + 'a + ?Sized,
    {
        fn from(s: &'a T) -> Self {
            let mut lines = s.as_ref().lines();

            let dirs = lines.next().unwrap().as_bytes();
            let map = lines
                .skip(1)
                .filter_map(|line| line.split_once(" = "))
                .map(|(key, values)| {
                    (
                        key,
                        values
                            .strip_prefix('(')
                            .and_then(|values| values.strip_suffix(')'))
                            .and_then(|values| values.split_once(", "))
                            .unwrap(),
                    )
                })
                .collect();
            Self(dirs, map)
        }
    }
}
// end::input[]

// tag::star_1[]
pub fn map_iter<'a>(
    dirs: &'a [u8],
    map: &'a HashMap<&'a str, (&'a str, &'a str)>,
    node: &'a str,
) -> impl Iterator<Item = (usize, &'a str)> + 'a {
    successors(Some((0, node)), |&(k, node)| {
        map.get(node)
            .map(|&(left, right)| match dirs[k % dirs.len()] {
                b'L' => (k + 1, left),
                _ => (k + 1, right),
            })
    })
}

pub fn star_1(PuzzleData(dirs, map): &PuzzleData) -> SolT {
    map_iter(dirs, map, "AAA")
        .find(|&(_, node)| node == "ZZZ")
        .map(|(steps, _)| steps)
        .unwrap()
}
// end::star_1[]

// tag::star_2[]
pub fn star_2(PuzzleData(dirs, map): &PuzzleData) -> SolT {
    map.keys()
        .filter(|key| key.ends_with('A'))
        .map(|&node| {
            let mut it = map_iter(dirs, map, node)
                .step_by(dirs.len()) // only find result that used all dirs
                .filter(|(_, node)| node.ends_with('Z'));
            let (steps_0, node_0) = it.next().unwrap();
            if cfg!(feature = "check-periodicity") {
                // the solution assumes periodicity, so let's check it
                let (steps_1, node_1) = it.next().unwrap();
                if steps_1 != 2 * steps_0 || node_1 != node_0 {
                    panic!("Periodicity assumption not satisfied!");
                }
            }
            steps_0
        })
        .fold(1, |result, steps| result * steps / gcd(result, steps))
}
// end::star_2[]

// tag::tests[]
#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    const CONTENT: &str = r#"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
"#;

    #[test]
    pub fn test_from() {
        let data = PuzzleData::from(CONTENT);
        println!("{data:?}");
        assert_eq!(
            PuzzleData(
                "LLR".as_bytes(),
                HashMap::from([
                    ("AAA", ("BBB", "BBB")),
                    ("BBB", ("AAA", "ZZZ")),
                    ("ZZZ", ("ZZZ", "ZZZ"))
                ])
            ),
            data
        );
    }

    #[test]
    pub fn test_star_1() {
        assert_eq!(6, star_1(&CONTENT.into()));
    }

    const CONTENT_2: &str = r#"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
"#;

    #[test]
    pub fn test_star_2() {
        assert_eq!(6, star_2(&CONTENT_2.into()));
    }

    pub fn do_understand(data: &str, n: usize) {
        let PuzzleData(dirs, map) = data.into();
        for node in map
            .keys()
            .filter(|node| node.ends_with('A'))
            .map(|node| *node)
        {
            println!("Node {}", node);
            let mut base = None;
            for (pos, node) in map_iter(dirs, &map, node)
                .step_by(dirs.len())
                .filter(|&(_, x)| x.ends_with('Z'))
                .take(n)
            {
                let base = *base.get_or_insert(pos);
                println!(
                    "    reached {} at step {} = {} * {} + {} = {} * {} + {}",
                    node,
                    pos,
                    pos / base,
                    base,
                    pos % base,
                    pos / dirs.len(),
                    dirs.len(),
                    pos % dirs.len()
                );
            }
        }
    }

    #[test]
    pub fn test_understand() {
        println!("\nSample data 2");
        println!("=============");
        do_understand(CONTENT_2, 4);

        println!("\nPuzzle input");
        println!("============");
        do_understand(&read_input(), 2);
    }
}
// end::tests[]
