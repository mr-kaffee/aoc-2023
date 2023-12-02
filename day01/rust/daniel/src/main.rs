use std::fs;

fn main() {
    let input = fs::read_to_string("/workspace/inputs/input01.txt");
    match input {
        Ok(input) => {
            let result_1 = calculate_result_part_1(&input);
            let result_2 = calculate_result_part_2(&input);
            print!("Solution to part 1: {}\n", result_1);
            print!("Solution to part 2: {}\n", result_2);
        },
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
}

fn calculate_result_part_1(input_str: &String) -> u32 {
    let mut sum: u32 = 0;

    for line in input_str.split('\n') {
        if line.is_empty() {
            continue;
        }
        let mut first: Option<u32> = None;
        let mut last: Option<u32> = None;
        for c in line.chars() {
            if c.is_ascii_digit() {
                if first.is_none() {
                    first = c.to_digit(10);
                }
                last = c.to_digit(10);
            }
        }
        if first.is_some() && last.is_some() {
            sum += first.unwrap() * 10 + last.unwrap();
        } else {
            panic!("Bad input");
        }
    }

    sum
}

fn calculate_result_part_2(input: &String) -> u32 {
    let mut sum = 0;

    let digits = [("zero", 0u32),
                                     ("one", 1),
                                     ("two", 2),
                                     ("three", 3),
                                     ("four", 4),
                                     ("five", 5),
                                     ("six", 6),
                                     ("seven", 7),
                                     ("eight", 8),
                                     ("nine", 9),
                                     ("0", 0),
                                     ("1", 1),
                                     ("2", 2),
                                     ("3", 3),
                                     ("4", 4),
                                     ("5", 5),
                                     ("6", 6),
                                     ("7", 7),
                                     ("8", 8),
                                     ("9", 9)];

    for line in input.split('\n') {
        if line.is_empty() {
            continue;
        }

        let mut min_pos: usize = line.len();
        let mut max_pos: usize = 0;
        let mut first: Option<u32> = None;
        let mut last: Option<u32> = None;
        for (str_rep, d) in digits.iter() {
            match line.find(str_rep) {
                Some(pos) => {
                    if pos <= min_pos {
                        min_pos = pos;
                        first = Some(*d);
                    }},
                None => continue
            };

            match line.rfind(str_rep){
                Some(pos) => {
                    if pos >= max_pos {
                        max_pos = pos;
                        last = Some(*d);
                    }
                },
                None => continue
            }
        }
        if first.is_some() && last.is_some() {
            sum += first.unwrap() * 10 + last.unwrap();
        } else {
            panic!("Bad input");
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use crate::calculate_result_part_1;
    use crate::calculate_result_part_2;

    #[test]
    fn single_line_input() {
        let result = calculate_result_part_1(&"aaa1aa2aa4aa\n".to_string());
        assert_eq!(result, 14);
    }

    #[test]
    fn multiple_line_input() {
        let result = calculate_result_part_1(&"10\naa10\n10aa\n".to_string());
        assert_eq!(result, 30);
    }

    #[test]
    fn only_str_digits() {
        let result = calculate_result_part_2(&"onetwothreeaaa\n".to_string());
        assert_eq!(result, 13);
    }

    #[test]
    fn str_digits_and_ascii_digits_mixed() {
        let result = calculate_result_part_2(&"onsssze1nine17aaaone18fivesixzero0011five\n".to_string());
        assert_eq!(result, 15);
    }

    #[test]
    fn only_one_ascii_digits_at_beginning() {
        let result = calculate_result_part_2(&"1qmnxhfjsrq\n".to_string());
        assert_eq!(result, 11);
    }

    #[test]
    fn only_one_ascii_digits_at_end() {
        let result = calculate_result_part_2(&"qmnxhfjsrq7\n".to_string());
        assert_eq!(result, 77);
    }
}
