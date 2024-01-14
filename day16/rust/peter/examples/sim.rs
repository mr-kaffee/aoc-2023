use clap::Parser;
use mr_kaffee_2023_16::{input::PuzzleData, read_input, simulate_beam, EAST, NORTH, SOUTH, WEST};
use std::{error::Error, fs::File, iter::once};

#[derive(Parser, Debug)]
struct Cli {
    /// output file
    #[arg(short, long, default_value_t = String::from("sim.png"))]
    out: String,

    /// use example data
    #[arg(short, long)]
    example: bool,

    /// scaling factor for image
    #[arg(short = 'c', long, default_value_t = 3)]
    scale: usize,

    /// frames per second
    #[arg(short, long, default_value_t = 10)]
    fps: u16,

    /// step size between frames
    #[arg(short, long, default_value_t = 25)]
    step_size: usize,

    /// tile number on boundary
    #[arg(short, long, default_value_t = 0)]
    tile: usize,

    /// initial direction
    #[arg(short, long, default_value_t = EAST, value_parser = clap::value_parser!(u8).range(0..4))]
    dir: u8,
}

pub fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    let input = (!cli.example).then(|| read_input());
    let PuzzleData(data, w, h) =
        PuzzleData::from(input.as_ref().map(String::as_str).unwrap_or(EXAMPLE));

    let check = |tile, dim| {
        assert!(tile < dim, "Dim is {} but tile is {}", dim, tile);
        tile
    };

    let file_name = cli.out;
    let scale = cli.scale;
    let fps = cli.fps;
    let step_size = cli.step_size;
    let start = match cli.dir {
        EAST => ((0, check(cli.tile, h)), EAST),
        NORTH => ((check(cli.tile, w), h - 1), NORTH),
        WEST => ((w - 1, h - 1 - check(cli.tile, h)), WEST),
        SOUTH => ((w - 1 - check(cli.tile, w), 0), SOUTH),
        _ => unreachable!(),
    };

    let images = once(get_image_data(data, None, (w, h), scale))
        .chain(
            (step_size..)
                .step_by(step_size)
                .map(|steps| (steps, simulate_beam(data, (w, h), start, steps)))
                .take_while(|(req_steps, (_, act_steps))| req_steps == act_steps)
                .map(|(_, (beam, _))| get_image_data(data, Some(&beam), (w, h), scale)),
        )
        .collect::<Vec<_>>();

    let file = File::create(file_name)?;
    let mut encoder = png::Encoder::new(file, (w * 5 * scale) as _, (h * 5 * scale) as _);
    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);
    encoder.set_animated(images.len() as _, 0)?;
    encoder.set_frame_delay(1, fps)?;
    let mut writer = encoder.write_header()?;

    for image in images {
        writer.write_image_data(&image)?;
    }

    Ok(())
}

fn get_image_data(
    data: &[u8],
    beam: Option<&[u8]>,
    (w, h): (usize, usize),
    scale: usize,
) -> Vec<u8> {
    let (w_fac, h_fac) = (3 * 5 * scale, 5 * scale);
    let mut buf = vec![0; w * w_fac * h * h_fac];
    let mut tile_buf = vec![0; w_fac * h_fac];
    for row in 0..h {
        for col in 0..w {
            get_tile(
                &mut tile_buf,
                data[col + row * (w + 1)],
                beam.map(|beam| beam[col + row * w]).unwrap_or_default(),
                scale,
            );
            for k in 0..h_fac {
                let off = w_fac * col + (h_fac * row + k) * w * w_fac;
                buf[off..off + w_fac].copy_from_slice(&tile_buf[k * w_fac..(k + 1) * w_fac]);
            }
        }
    }
    buf
}

fn get_tile(buf: &mut [u8], tile: u8, beam: u8, scale: usize) {
    for (k, b) in buf.iter_mut().enumerate() {
        *b = COLOR_1[k % 3];
    }

    let dirs = match tile {
        b'|' => [0b1110, 0b1010, 0b1011, 0b1010],
        b'-' => [0b0101, 0b1101, 0b0101, 0b0111],
        b'/' => [0b0110, 0b1001, 0b1001, 0b0110],
        b'\\' => [0b1100, 0b1100, 0b0011, 0b0011],
        _ => [0b0101, 0b1010, 0b0101, 0b1010],
    }
    .into_iter()
    .enumerate()
    .filter(|&(k, _)| (beam >> k) & 1 > 0)
    .fold(0, |acc, (_, dirs)| acc | dirs);

    for (_, (rs, cs)) in [(2..3, 2..5), (0..3, 2..3), (2..3, 0..3), (2..5, 2..3)]
        .into_iter()
        .enumerate()
        .filter(|&(k, _)| (dirs >> k) & 1 > 0)
    {
        for r in rs.start * scale..rs.end * scale {
            for c in cs.start * scale..cs.end * scale {
                let off = c * 3 + r * 3 * 5 * scale;
                buf[off..off + 3].copy_from_slice(&COLOR_4);
            }
        }
    }

    type It = std::iter::Map<std::ops::Range<usize>, fn(usize) -> (usize, usize)>;
    let mut it: It = match tile {
        b'|' => (1..4).map(|row| (2, row)),
        b'/' => (1..4).map(|col| (col, 4 - col)),
        b'-' => (1..4).map(|col| (col, 2)),
        b'\\' => (1..4).map(|col| (col, col)),
        _ => (0..0).map(|_| Default::default()),
    };
    let (mut a, mut b) = (it.next(), it.next());
    while let (Some((c1, r1)), Some((c2, r2))) = (a, b) {
        for k in 0..=scale {
            let c = (scale - k) * c1 + k * c2;
            let r = (scale - k) * r1 + k * r2;
            for r in r..r + scale {
                for c in c..c + scale {
                    let off = c * 3 + r * 3 * 5 * scale;
                    buf[off..off + 3].copy_from_slice(&COLOR_3);
                }
            }
        }
        (a, b) = (b, it.next());
    }
}

const COLOR_1: [u8; 3] = [0x16, 0x1A, 0x30];
const COLOR_3: [u8; 3] = [0x01, 0x74, 0xBE];
const COLOR_4: [u8; 3] = [0xFF, 0xCF, 0x36];

const EXAMPLE: &str = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#;
