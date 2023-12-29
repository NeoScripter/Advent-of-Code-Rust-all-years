use color_eyre::eyre;
use std::fmt;
struct PrettyGrid(Vec<Vec<u8>>);

impl fmt::Display for PrettyGrid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.0 {
            for &num in row {
                write!(f, "{: >1}", num)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn transpose_and_rotate(grid: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
  let cols = grid[0].len();
  let new_grid: Vec<Vec<u8>> = (0..cols)
      .map(|i| {
          grid.iter()
              .rev() 
              .map(|row| row[i])
              .collect::<Vec<u8>>()
      })
      .collect();
  new_grid
}

fn process_grid(grid: Vec<Vec<u8>>) -> u32 {
    let rotated = transpose_and_rotate(&grid);
    let col = find_symmetry(rotated);
    let rows = find_symmetry(grid);
    rows * 100 + col
}
fn find_symmetry(grid: Vec<Vec<u8>>) -> u32 {
  let mut stack = Vec::new();
  let mut is_middle = false;
  let mut rows_above = 0;
  
  for line in grid.iter() {
      if !is_middle {
          if stack.last().map_or(false, |&last| last == line) {
              is_middle = true;
              rows_above = stack.len() as u32;
              stack.pop();
              continue;
          }
          stack.push(line);
      } else if stack.pop().map_or(true, |last| last != line) {
            rows_above = 0;
            break;
      }
  }  
  rows_above
}
fn part1(input: &str) -> u32 {
  input.split("\r\n\r\n").map(|grid| {
    let field: Vec<Vec<u8>> = grid.lines().map(|line| {
      line.bytes().collect()
    }).collect();
    process_grid(field) as u32
  }).sum()
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let input = include_str!("input19.txt");
    println!("{:?}", part1(input));
    Ok(())
}