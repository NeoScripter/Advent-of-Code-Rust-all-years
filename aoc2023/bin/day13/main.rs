// This is not my solution

use color_eyre::eyre;
pub fn parse_input(input: &str) -> Vec<Vec<Vec<u8>>> {
    input
        .split("\r\n\r\n")
        .map(|grid| grid.lines().map(|l| l.as_bytes().to_vec()).collect())
        .collect()
}
 
fn part1(input: &[Vec<Vec<u8>>]) -> usize {
    let mut answer = 0;
 
    for grid in input {
        let mut x = find_horizontal_line(grid, 0, 0) * 100;
        if x == 0 {
            x = find_vertical_line(grid, 0, 0);
        }
 
        answer += x;
    }
 
    answer
}
fn part2(input: &[Vec<Vec<u8>>]) -> usize {
    let mut answer = 0;
 
    for grid in input {
        let h = find_horizontal_line(grid, 0, 0);
        let mut x = find_horizontal_line(grid, 1, h) * 100;
 
        if x == 0 {
            let v = find_vertical_line(grid, 0, 0);
            x = find_vertical_line(grid, 1, v);
        }
 
        answer += x;
    }
 
    answer
}
fn find_horizontal_line(grid: &[Vec<u8>], max_diffs: usize, skip_line: usize) -> usize {
    for r in 0..grid.len() - 1 {
        if has_horizontal_symmetry(grid, max_diffs, r) {
            if r + 1 != skip_line {
                return r + 1;
            }
        }
    }
 
    0
}
 
fn has_horizontal_symmetry(grid: &[Vec<u8>], max_diffs: usize, line: usize) -> bool {
    let mut up = line;
    let mut down = line + 1;
    let mut diffs = 0;
 
    loop {
        for c in 0..grid[up].len() {
            if grid[up][c] != grid[down][c] {
                diffs += 1;
 
                if diffs > max_diffs {
                    return false;
                }
            }
        }
 
        if up == 0 || down == grid.len() - 1 {
            return true;
        }
 
        up -= 1;
        down += 1;
    }
}
 
fn find_vertical_line(grid: &[Vec<u8>], max_diffs: usize, skip_line: usize) -> usize {
    for c in 0..grid[0].len() - 1 {
        if has_vertical_symmetry(grid, max_diffs, c) {
            if c + 1 != skip_line {
                return c + 1;
            }
        }
    }
 
    0
}
 
fn has_vertical_symmetry(grid: &[Vec<u8>], max_diffs: usize, line: usize) -> bool {
    let mut left = line;
    let mut right = line + 1;
    let mut diffs = 0;
 
    loop {
        for r in 0..grid.len() {
            if grid[r][left] != grid[r][right] {
                diffs += 1;
 
                if diffs > max_diffs {
                    return false;
                }
            }
        }
 
        if left == 0 || right == grid[0].len() - 1 {
            return true;
        }
 
        left -= 1;
        right += 1;
    }
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let input = include_str!("input13.txt");
    println!("{:?}", part2(&parse_input(input)));
    Ok(())
}