use input::*;
use std::{collections::VecDeque, fs::read_to_string};

// tag::prelude[]
pub const IDENTIFIER: &str = "2023/20";

pub type InputT = PuzzleData;

pub fn read_input() -> String {
    read_to_string("../../../inputs/input20").unwrap()
}
// end::prelude[]

// tag::input[]
pub mod input {
    #[derive(Debug, PartialEq, Eq)]
    pub struct PuzzleData {
        pub bc: usize,
        pub rx: Option<usize>,
        pub targets: Vec<Vec<usize>>,
        pub types: Vec<u8>,
    }

    fn get_id<'a>(nodes: &mut Vec<&'a str>, node: &'a str) -> usize {
        let id = nodes.iter().position(|&n| n == node).unwrap_or(nodes.len());
        if id == nodes.len() {
            nodes.push(node);
        }
        id
    }

    impl<T> From<&T> for PuzzleData
    where
        T: AsRef<str> + ?Sized,
    {
        fn from(s: &T) -> Self {
            let mut nodes = Vec::new();
            let mut targets = Vec::new();
            let mut types = Vec::new();
            for line in s.as_ref().lines() {
                let (source, targets_part) = line.split_once(" -> ").unwrap();
                let (source_type, source) = match source.as_bytes()[0] {
                    b'%' => (b'%', &source[1..]),
                    b'&' => (b'&', &source[1..]),
                    b if b.is_ascii_alphabetic() => (0, source),
                    _ => panic!(),
                };

                let source_id = get_id(&mut nodes, source);
                let target_ids = targets_part
                    .split(", ")
                    .collect::<Vec<_>>()
                    .iter()
                    .map(|&target| get_id(&mut nodes, target))
                    .collect::<Vec<_>>();

                targets.resize(nodes.len(), Vec::default());
                targets[source_id] = target_ids;

                types.resize(nodes.len(), 0);
                types[source_id] = source_type;
            }

            let bc = nodes
                .iter()
                .position(|&name| name == "broadcaster")
                .unwrap();
            let rx = nodes.iter().position(|&name| name == "rx");

            Self {
                bc,
                rx,
                targets,
                types,
            }
        }
    }
}
// end::input[]

// tag::memory[]
#[derive(Clone, PartialEq, Eq)]
pub enum Mem {
    Plain,
    FlipFlop(bool),
    Conjunction(Vec<(usize, bool)>),
}

pub fn dump_memory(mems: &[Mem]) -> (u128, u128) {
    mems.iter().fold((0, 0), |(dump, mask), mem| match mem {
        Mem::Plain => (dump, mask),
        Mem::FlipFlop(val) => (dump << 1 | *val as u128, mask << 1 | 1),
        Mem::Conjunction(values) => values.iter().fold((dump, mask), |(dump, mask), (_, val)| {
            (dump << 1 | *val as u128, mask << 1 | 1)
        }),
    })
}

pub fn init(targets: &[Vec<usize>], types: &[u8]) -> Vec<Mem> {
    let mut memories = types
        .iter()
        .map(|&t| match t {
            0 => Mem::Plain,
            b'&' => Mem::Conjunction(Vec::new()),
            b'%' => Mem::FlipFlop(false),
            _ => panic!(),
        })
        .collect::<Vec<_>>();

    for k in 0..types.len() {
        for &target in &targets[k] {
            if let Mem::Conjunction(ref mut vec) = &mut memories[target] {
                vec.push((k, false));
            }
        }
    }

    memories
}
// end::memory[]

// tag::press_button[]
pub fn press_button<F>(memories: &mut [Mem], bc: usize, targets: &[Vec<usize>], mut callback: F)
where
    F: FnMut(usize, usize, bool) -> (),
{
    let mut queue = VecDeque::new();

    for &target in &targets[bc] {
        queue.push_back((target, bc, false));
    }

    while let Some((target, source, value)) = queue.pop_front() {
        callback(target, source, value);

        match &mut memories[target] {
            Mem::FlipFlop(ref mut mem) if !value => {
                *mem = !*mem;
                for &t in &targets[target] {
                    queue.push_back((t, target, *mem));
                }
            }
            Mem::Conjunction(ref mut mems) => {
                let (_, mem) = mems.iter_mut().find(|(id, _)| *id == source).unwrap();
                *mem = value;
                let upd = !mems.iter().all(|(_, v)| *v);
                for &t in &targets[target] {
                    queue.push_back((t, target, upd));
                }
            }
            _ => (),
        }
    }
}
// end::press_button[]

// tag::star_1[]
#[derive(Debug, Default, PartialEq, Eq)]
struct SendCounter(usize, usize);

impl SendCounter {
    fn callback(&mut self) -> impl FnMut(usize, usize, bool) -> () + '_ {
        |_, _, value| self.count(value)
    }

    fn count(&mut self, value: bool) {
        if value {
            self.0 += 1
        } else {
            self.1 += 1
        }
    }

    fn get(&self) -> usize {
        self.0 * self.1
    }
}

pub fn star_1(
    PuzzleData {
        bc,
        rx: _,
        targets,
        types,
    }: &PuzzleData,
) -> usize {
    let mut mem = init(targets, types);

    let mut counter = SendCounter::default();
    for _ in 0..1_000 {
        counter.count(false); // button press
        press_button(&mut mem, *bc, targets, counter.callback());
    }
    counter.get()
}
// end::star_1[]

// tag::star_2[]
struct Sources {
    values: Vec<Option<usize>>,
    source: usize,
    source_sources: Vec<usize>,
    buttons: usize,
}

pub fn find_sources(targets: &[Vec<usize>], target: usize) -> Vec<usize> {
    targets
        .iter()
        .enumerate()
        .filter(|(_, targets)| targets.contains(&target))
        .map(|(pos, _)| pos)
        .collect()
}

impl Sources {
    fn create(targets: &[Vec<usize>], rx: usize) -> Self {
        let sources = find_sources(&targets, rx);
        assert_eq!(1, sources.len());
        let source = sources[0];
        let source_sources = find_sources(&targets, source);
        let values = vec![None; source_sources.len()];

        Self {
            values,
            source,
            source_sources,
            buttons: 0,
        }
    }

    fn callback(&mut self) -> impl FnMut(usize, usize, bool) -> () + '_ {
        |target, source, value| {
            if target == self.source && value {
                let id = self
                    .source_sources
                    .iter()
                    .position(|&id| id == source)
                    .unwrap();
                if self.values[id].is_none() {
                    self.values[id] = Some(self.buttons);
                }
            }
        }
    }

    fn button(&mut self) {
        self.buttons += 1;
    }

    fn done(&self) -> bool {
        self.values.iter().all(|v| v.is_some())
    }

    fn get(&mut self) -> usize {
        self.values.iter().flatten().product()
    }
}

pub fn star_2(
    PuzzleData {
        bc,
        rx,
        targets,
        types,
    }: &PuzzleData,
) -> usize {
    let mut memories = init(&targets, &types);

    let mut sources = Sources::create(&targets, rx.unwrap());
    while !sources.done() {
        sources.button();
        press_button(&mut memories, *bc, &targets, sources.callback());
    }
    sources.get()
}
// end::star_2[]

// tag::tests[]
#[cfg(test)]
mod tests {
    use super::*;

    const CONTENT_1: &str = r#"broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a
"#;

    const CONTENT_2: &str = r#"broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output
"#;

    #[test]
    pub fn test_from() {
        let data = PuzzleData::from(CONTENT_1);
        // "broadcaster", "a", "b", "c", "inv"
        let exp = PuzzleData {
            bc: 0,
            rx: None,
            targets: vec![vec![1, 2, 3], vec![2], vec![3], vec![4], vec![1]],
            types: vec![0, b'%', b'%', b'%', b'&'],
        };
        println!("{data:?}");
        assert_eq!(exp, data);

        let data = PuzzleData::from(CONTENT_2);
        // "broadcaster", "a", "inv", "con", "b", "output"
        let exp = PuzzleData {
            bc: 0,
            rx: None,
            targets: vec![vec![1], vec![2, 3], vec![4], vec![5], vec![3], vec![]],
            types: vec![0, b'%', b'&', b'&', b'%', 0],
        };
        println!("{data:?}");
        assert_eq!(exp, data);
    }

    #[test]
    pub fn test_star_1() {
        assert_eq!(32_000_000, star_1(&CONTENT_1.into()));
        assert_eq!(11_687_500, star_1(&CONTENT_2.into()));
    }

    #[test]
    pub fn test_press_button_with_send_counter() {
        let PuzzleData {
            bc,
            rx: _,
            targets,
            types,
        } = CONTENT_2.into();
        let mut mem = init(&targets, &types);

        let mut counter = SendCounter::default();

        press_button(&mut mem, bc, &targets, counter.callback());
        assert_eq!(SendCounter(4, 3), counter);

        press_button(&mut mem, bc, &targets, counter.callback());
        assert_eq!(SendCounter(4 + 2, 3 + 3), counter);
    }

    #[test]
    pub fn test_dump_memory() {
        let PuzzleData {
            bc: _,
            rx: _,
            targets,
            types,
        } = CONTENT_1.into();
        let mem = init(&targets, &types);
        assert_eq!((0, 0xf), dump_memory(&mem));
    }
}
// end::tests[]
