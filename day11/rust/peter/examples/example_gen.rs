use clap::Parser;
use mr_kaffee_2023_11::{calc_offsets, input::PuzzleData, sum_shortest_path};
use rand::seq::SliceRandom;

#[derive(Parser, Debug)]
struct Cli {
    /// expansion factor
    #[arg(short, long, default_value_t = 1_000_000)]
    expansion: usize,

    /// width
    #[arg(short, default_value_t = 15)]
    w: usize,

    /// height
    #[arg(short, default_value_t = 15)]
    h: usize,

    /// fill ratio, every r-th spot is a galaxy
    #[arg(short, default_value_t = 20)]
    r: usize,

    /// number of iterations
    #[arg(short, default_value_t = 10)]
    n: usize,

    /// plot expanded universe (has no effect if expansion > 20)
    #[arg(short)]
    plot_expanded: bool,
}

pub fn make_universe(w: usize, h: usize, r: usize) -> String {
    let mut grid = vec!['.'; (w + 1) * h];
    for row in 0..h {
        grid[(w + 1) * (row + 1) - 1] = '\n';
    }

    let mut indices = Vec::<usize>::from_iter(0..w * h);
    let mut rng = rand::thread_rng();
    indices.shuffle(&mut rng);

    for &k in indices[0..(w * h + r - 1) / r].into_iter() {
        grid[k + k / w] = '#';
    }

    String::from_iter(grid)
}

pub fn main() {
    let cli = Cli::parse();
    for _ in 0..cli.n {
        let grid = make_universe(cli.w, cli.h, cli.r);
        let data = PuzzleData::from(&grid);
        let result = sum_shortest_path(&data, cli.expansion);

        println!("\n========================================\n");
        if cli.plot_expanded {
            println!("Original universe:");
        }
        println!("{}", grid);
        println!("Expansion factor: {}", cli.expansion);
        println!("Result:           {}", result);

        if cli.plot_expanded && cli.expansion <= 20 {
            let col_offsets = calc_offsets(&data.count_in_cols, cli.expansion);
            let row_offsets = calc_offsets(&data.count_in_rows, cli.expansion);
            let w_e = col_offsets.last().unwrap() + 1;
            let h_e = row_offsets.last().unwrap() + 1;
            let grid_e =
                data.galaxies
                    .iter()
                    .fold(vec!['.'; w_e * h_e], |mut grid_e, &(col, row)| {
                        grid_e[col_offsets[col] + row_offsets[row] * w_e] = '#';
                        grid_e
                    });
            println!("\nExpanded universe:");
            for row in 0..h_e {
                println!("{}", String::from_iter(&grid_e[row * w_e..(row + 1) * w_e]));
            }
        }
    }
}
