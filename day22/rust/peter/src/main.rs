use mr_kaffee_2023_22::*;
use std::time::Instant;

fn main() {
    let t = Instant::now();

    let input = read_input();
    let data = InputT::from(&input);

    let t_1_and_2 = Instant::now();
    let (sol_1, sol_2) = star_1_and_2(&data);
    println!(
        "Solved {}, star 1 and 2 in {:?}: ({}, {})",
        IDENTIFIER,
        t_1_and_2.elapsed(),
        sol_1,
        sol_2
    );

    println!("Solved {} in {:?}", IDENTIFIER, t.elapsed());
}
