use mr_kaffee_2023_00::*;
use std::time::Instant;

fn main() {
    let t = Instant::now();

    let input = parse_input();

    let t_1 = Instant::now();
    let sol_1 = star_1(input);
    println!(
        "Solved {}, star 1 in {:?}: {}",
        IDENTIFIER,
        t_1.elapsed(),
        sol_1
    );

    println!("Solved {} in {:?}", IDENTIFIER, t.elapsed());
}
