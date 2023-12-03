use std::fs;
use regex::Regex;
use std::collections::HashMap;

fn main() {
    let input = match fs::read_to_string("/workspace/inputs/input02.txt") {
        Ok(input_str) => input_str,
        Err(error) => panic!("Problem opening input file: {:?}", error),
    };

    let result_1 = calculate_result_part_1(&input);
    print!("Solution to part 1: {}\n", result_1);
}

fn calculate_result_part_1(input: &String) -> u32 {
    let cubes = HashMap::from([
        ("red", 12),
        ("green", 13),
        ("blue", 14),
    ]);

    let mut sum_ids: u32 = 0;
    let re = Regex::new(r" (([0-9]+) ([a-z]+),?)+").unwrap();

    for (id, game) in input.split('\n').enumerate() {
        if game.is_empty() { continue; }
        let prefix = ["Game ", &(id+1).to_string(), ":"].join("");

        let mut valid_game: bool = true;
        'outer: for samples in game.strip_prefix(&prefix).unwrap().split(';') {
            for c in re.captures_iter(samples) {
                if cubes.get(c.get(3).unwrap().into()).unwrap() < &c.get(2).unwrap().as_str().parse::<i32>().unwrap() {
                    valid_game = false;
                    break 'outer;
                }
            }
        }

        if valid_game {
            sum_ids += (id+1) as u32;
        }
    }

    sum_ids
}

#[cfg(test)]
mod tests {
    use crate::calculate_result_part_1;

    #[test]
    fn one_game() {
        let input = "Game 1: 5 red\n".to_string();
        let result = calculate_result_part_1(&input);
        assert_eq!(result, 1);
    }

    #[test]
    fn two_games() {
        let input = "Game 1: 5 red\nGame 2: 45 green\n".to_string();
        let result = calculate_result_part_1(&input);
        assert_eq!(result, 1);
    }
}