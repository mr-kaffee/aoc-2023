use input::*;
use std::{collections::HashMap, fs::read_to_string, iter::successors};

// tag::prelude[]
pub const IDENTIFIER: &str = "2023/19";

pub type SolT = usize;
pub type InputT<'a> = PuzzleData<'a>;

pub fn read_input() -> String {
    read_to_string("../../../inputs/input19").unwrap()
}
// end::prelude[]

// tag::input[]
pub mod input {
    use std::{cmp::Ordering, collections::HashMap};

    pub type ValT = usize;
    pub type RuleT<'a> = (Option<(usize, Ordering, ValT)>, &'a str);
    pub type PartT = [ValT; 4];

    #[derive(Debug)]
    pub struct PuzzleData<'a>(pub HashMap<&'a str, Vec<RuleT<'a>>>, pub Vec<PartT>);

    fn parse_rule(rule: &str) -> RuleT {
        rule.split_once(':')
            .map(|(cond, target)| {
                (
                    Some((
                        match cond.as_bytes()[0] {
                            b'x' => 0,
                            b'm' => 1,
                            b'a' => 2,
                            b's' => 3,
                            b => panic!("Bad symbol: '{}'", b as char),
                        },
                        match cond.as_bytes()[1] {
                            b'>' => Ordering::Greater,
                            b'<' => Ordering::Less,
                            b => panic!("Bad symbol: '{}'", b as char),
                        },
                        cond[2..].parse().unwrap(),
                    )),
                    target,
                )
            })
            .unwrap_or((None, rule))
    }

    fn parse_workflows(workflow: &str) -> HashMap<&str, Vec<RuleT>> {
        workflow
            .lines()
            .map(|line| {
                line.split_once('{')
                    .and_then(|(label, rules)| rules.strip_suffix('}').map(|rules| (label, rules)))
                    .unwrap()
            })
            .map(|(label, rules)| (label, rules.split(',').map(parse_rule).collect()))
            .collect()
    }

    fn parse_parts(parts: &str) -> Vec<PartT> {
        parts
            .lines()
            .map(|part| {
                let mut values = part
                    .trim_matches::<&[char]>(&['{', '}'])
                    .split(',')
                    .map(|n| n[2..].parse().unwrap());
                [
                    values.next().unwrap(),
                    values.next().unwrap(),
                    values.next().unwrap(),
                    values.next().unwrap(),
                ]
            })
            .collect()
    }

    impl<'a, T> From<&'a T> for PuzzleData<'a>
    where
        T: AsRef<str> + 'a + ?Sized,
    {
        fn from(s: &'a T) -> Self {
            s.as_ref()
                .split_once("\n\n")
                .map(|(workflows, parts)| Self(parse_workflows(workflows), parse_parts(parts)))
                .unwrap()
        }
    }
}
// end::input[]

// tag::star_1[]
pub fn star_1(PuzzleData(workflows, parts): &PuzzleData) -> SolT {
    parts
        .iter()
        .filter(|part| {
            Some("A")
                == successors(Some("in"), |&target| {
                    workflows.get(target).and_then(|workflow| {
                        workflow
                            .iter()
                            .find(|(cond, _)| {
                                cond.map(|(idx, ord, val)| part[idx].cmp(&val) == ord)
                                    .unwrap_or(true)
                            })
                            .map(|&(_, target)| target)
                    })
                })
                .last()
        })
        .map(|part| part.iter().sum::<SolT>())
        .sum()
}
// end::star_1[]

// tag::star_2[]
pub fn star_2_rec(
    workflows: &HashMap<&str, Vec<RuleT>>,
    target: &str,
    mut v_min: PartT,
    mut v_max: PartT,
) -> SolT {
    match target {
        "A" => {
            return (0..4)
                .map(|idx| (v_max[idx] - v_min[idx]) as SolT)
                .product()
        }
        "R" => return 0,
        _ => (),
    }

    let mut count = 0;
    for (rule, next) in workflows.get(target).unwrap() {
        match rule
            .map(|(idx, ord, val)| {
                (
                    idx,
                    val,
                    v_min[idx].cmp(&val) == ord,
                    (v_max[idx] - 1).cmp(&val) == ord,
                )
            })
            .unwrap_or((0, 0, true, true))
        {
            (_, _, false, false) => (), // rule does not match at all -> check next rule
            (_, _, true, true) => {
                // rule fully matches -> recurse and stop checking further rules here
                return count + star_2_rec(workflows, next, v_min, v_max);
            }
            (idx, val, true, false) => {
                // rule matches for values_min_match[idx]..val -> recurse
                let (v_min_rec, mut v_max_rec) = (v_min, v_max);
                v_max_rec[idx] = val;
                count += star_2_rec(workflows, next, v_min_rec, v_max_rec);
                // rule does not match for val..values_max[idx] -> check next rule
                v_min[idx] = val;
            }
            (idx, val, false, true) => {
                // rule matches for val + 1..values_max_match[idx] -> recurse
                let (mut v_min_rec, v_max_rec) = (v_min, v_max);
                v_min_rec[idx] = val + 1;
                count += star_2_rec(workflows, next, v_min_rec, v_max_rec);
                // rule does not match for values_min[idx]..val + 1 -> check next rule
                v_max[idx] = val + 1;
            }
        }
    }

    unreachable!("Eventually everything shall match")
}

pub fn star_2(PuzzleData(workflows, _): &PuzzleData) -> SolT {
    star_2_rec(workflows, "in", [1; 4], [4001; 4])
}
// end::star_2[]

// tag::tests[]
#[cfg(test)]
mod tests {
    use super::*;

    const CONTENT: &str = r#"px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}
"#;

    #[test]
    pub fn test_from() {
        let data = PuzzleData::from(CONTENT);
        println!("{data:?}");
    }

    #[test]
    pub fn test_star_1() {
        assert_eq!(19_114, star_1(&CONTENT.into()));
    }

    #[test]
    pub fn test_star_2() {
        assert_eq!(167_409_079_868_000, star_2(&CONTENT.into()));
    }
}
// end::tests[]
