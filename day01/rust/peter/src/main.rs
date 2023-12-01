use mr_kaffee_2023_01::*;
use std::time::Instant;

fn main() {
    let t = Instant::now();

    let input = parse_input();

    let t_1 = Instant::now();
    let sol_1 = star(input.as_ref(), &map_1);
    println!(
        "Solved {}, star 1 in {:?}: {}",
        IDENTIFIER,
        t_1.elapsed(),
        sol_1
    );

    let t_2 = Instant::now();
    let sol_2 = star(input.as_ref(), &map_2);
    println!(
        "Solved {}, star 2 in {:?}: {}",
        IDENTIFIER,
        t_2.elapsed(),
        sol_2
    );

    println!("Solved {} in {:?}", IDENTIFIER, t.elapsed());
}
