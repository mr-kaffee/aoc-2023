use std::fs::read_to_string;
use std::{collections::HashMap, iter::once};

// tag::prelude[]
pub const IDENTIFIER: &str = "2023/12";

pub type SolT = usize;
pub type InputT = String;

pub fn read_input() -> String {
    read_to_string("../../../inputs/input12").unwrap()
}
// end::prelude[]

// tag::star_1[]
pub fn parse_line(line: &str) -> (&[u8], Vec<SolT>) {
    line.split_once(' ')
        .map(|(data, groups)| {
            (
                data.as_bytes(),
                groups
                    .split(',')
                    .map(str::parse)
                    .collect::<Result<Vec<_>, _>>()
                    .unwrap(),
            )
        })
        .unwrap()
}

type Cache<'a> = HashMap<(&'a [u8], &'a [SolT]), SolT>;

pub fn check(data: &[u8], groups: &[SolT], cached: bool) -> SolT {
    let mut cache = if cached { Some(Cache::default()) } else { None };
    check_recursive(data, groups, &mut cache)
}

fn check_recursive<'a>(data: &'a [u8], groups: &'a [SolT], cache: &mut Option<Cache<'a>>) -> SolT {
    if let Some(&result) = cache.as_mut().and_then(|cache| cache.get(&(data, groups))) {
        // return cached result
        return result;
    }

    let result = if groups.is_empty() {
        if data.iter().all(|&d| d != b'#') {
            1
        } else {
            0
        }
    } else {
        // minimum elements still required
        let min_len = groups.iter().sum::<SolT>() + groups.len() - 1;

        // current group (group <= min_len guaranteed)
        let group = groups[0];

        // result
        let mut result = 0;
        for pos in 0..(data.len() + 1).saturating_sub(min_len) {
            if data[pos..pos + group].iter().all(|&b| b != b'.') {
                // next group elements can be damaged
                if data.len() == pos + group {
                    // no more elements
                    if groups.len() == 1 {
                        // no more groups
                        result += 1;
                    }
                } else if data[pos + group] != b'#' {
                    // next element afterwards can be operational
                    result += check_recursive(&data[pos + group + 1..], &groups[1..], cache);
                }
            }

            if data[pos] == b'#' {
                // current element is damaged
                break;
            }
        }
        result
    };

    // cache and return result
    cache
        .as_mut()
        .map(|cache| cache.insert((data, groups), result));
    result
}

pub fn star_1(data: &str) -> SolT {
    data.lines()
        .map(parse_line)
        .map(|(data, groups)| check(data, &groups, false))
        .sum()
}
// end::star_1[]

// tag::star_2[]
pub fn star_2(data: &str) -> SolT {
    data.lines()
        .map(parse_line)
        .map(|(data, groups)| {
            let new_data_len = 5 * (data.len() + 1) - 1;
            let new_groups_len = 5 * groups.len();
            (
                data.iter()
                    .copied()
                    .chain(once(b'?'))
                    .cycle()
                    .take(new_data_len)
                    .collect::<Vec<_>>(),
                groups
                    .into_iter()
                    .cycle()
                    .take(new_groups_len)
                    .collect::<Vec<_>>(),
            )
        })
        .map(|(data, groups)| check(&data, &groups, true))
        .sum()
}
// end::star_2[]

// tag::tests[]
#[cfg(test)]
mod tests {
    use super::*;

    const CONTENT: &str = r#"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
"#;

    #[test]
    pub fn test_check() {
        assert_eq!(1, check(".###.##.#...".as_bytes(), &[3, 2, 1], false));
        assert_eq!(0, check(".###.##.#...".as_bytes(), &[3, 2, 2], true));
        assert_eq!(0, check(".###.##.#...".as_bytes(), &[3, 2, 1, 1], false));
        assert_eq!(1, check(".###.##....#".as_bytes(), &[3, 2, 1], true));

        const EXP: &[SolT] = &[1, 4, 1, 1, 4, 10];
        for (k, (line, &exp)) in CONTENT.lines().zip(EXP.iter()).enumerate() {
            let (data, groups) = parse_line(line);
            assert_eq!(exp, check(data, &groups, k & 1 == 0), "{}", line);
        }
    }

    #[test]
    pub fn test_star_1() {
        assert_eq!(21, star_1(CONTENT));
    }

    #[test]
    pub fn test_star_2() {
        assert_eq!(525_152, star_2(CONTENT));
    }
}
// end::tests[]
