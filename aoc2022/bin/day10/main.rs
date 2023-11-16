use color_eyre::eyre::{self, Result};
use itertools::Itertools;

const COLS: usize = 40;
const ROWS: usize = 6;
const SPRITE_WIDTH: u32 = 3;

fn get_pixel(cycle: usize, x: i32) -> char {
    let curr_col = (cycle - 1) % COLS;
    if (curr_col as i32).abs_diff(x) <= SPRITE_WIDTH / 2 {
        '█'
    } else {
        ' '
    }
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let input = include_str!("input10.txt");
    let mut x = 1;
    let mut cycle = 1;
    let mut screen = [' '; COLS * ROWS];

    for line in input.lines() {
        screen[cycle - 1] = get_pixel(cycle, x);
        cycle += 1;

        if let Some(("addx", num)) = line.split_once(' ') {
            screen[cycle - 1] = get_pixel(cycle, x);
            cycle += 1;
            let num: i32 = num.parse().unwrap();
            x += num;
        }
    }

    let image = screen
        .chunks(COLS)
        .map(|row| row.iter().collect())
        .collect::<Vec<String>>()
        .join("\n");

    println!("{}", image);
    Ok(())
}