use std::fs::read_to_string;

// tag::prelude[]
pub const IDENTIFIER: &str = "2023/15";

pub type InputT<'a> = &'a str;

pub fn read_input() -> String {
    read_to_string("../../../inputs/input15").unwrap()
}
// end::prelude[]

// tag::star_1[]
pub fn hash(item: &str) -> usize {
    item.bytes()
        .fold(0, |hash, b| ((hash + (b as usize)) * 17) & 255)
}

pub fn star_1(data: &str) -> usize {
    data.split(',').map(str::trim).map(hash).sum()
}
// end::star_1[]

// tag::star_2[]
pub fn star_2(data: &str) -> usize {
    let init_boxes = || {
        let mut boxes = Vec::with_capacity(256);
        boxes.resize(256, Vec::new());
        boxes
    };

    data.split(',')
        .map(str::trim)
        .map(|step| {
            step.split_once('=')
                .map(|(label, value)| (label, Some(value.parse::<usize>().unwrap())))
                .or_else(|| step.strip_suffix('-').map(|label| (label, None)))
                .unwrap()
        })
        .fold(init_boxes(), |mut boxes, (label, value)| {
            let box_ = &mut boxes[hash(label)];
            let idx = box_.iter().position(|(other, _)| other == &label);
            match (value, idx) {
                (Some(value), Some(idx)) => box_[idx] = (label, value),
                (Some(value), _) => box_.push((label, value)),
                (_, Some(idx)) => _ = box_.remove(idx),
                _ => (),
            }
            boxes
        })
        .iter()
        .enumerate()
        .map(|(box_no, box_)| {
            box_.iter()
                .enumerate()
                .map(|(lens_no, &(_, value))| (box_no + 1) * (lens_no + 1) * value)
                .sum::<usize>()
        })
        .sum()
}
// end::star_2[]

// tag::tests[]
#[cfg(test)]
mod tests {
    use super::*;

    const CONTENT: &str = r#"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"#;

    #[test]
    pub fn test_star_1() {
        assert_eq!(1_320, star_1(CONTENT));
    }

    #[test]
    pub fn test_star_2() {
        assert_eq!(145, star_2(CONTENT));
    }
}
// end::tests[]
