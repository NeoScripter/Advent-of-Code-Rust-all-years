use color_eyre::eyre;
use std::collections::HashMap;
use std::fmt;

struct PrettyGrid(Vec<Vec<char>>);

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

fn transpose_and_rotate(grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
  let cols = grid[0].len();

  let new_grid: Vec<Vec<char>> = (0..cols)
      .rev()
      .map(|i| {
          grid.iter()
              .map(|row| row[i])
              .collect::<Vec<char>>()
      })
      .collect();
  new_grid
}

fn load(map: &Vec<Vec<char>>) -> u32 {
  (0..map.len())
    .map(|r| (0..map[0].len())
      .filter(|&c| map[r][c] == 'O')
      .map(|_| (map.len() - r) as u32)
      .sum::<u32>()
    )
    .sum()
}
fn part2(input: &str) -> u32 {
  let mut grid: Vec<Vec<char>> = input.lines().map(|line|line.chars().collect()).collect();

  let mut seen = HashMap::new();
  for i in 1..1000000000 {
      swap_north(&mut grid);
      swap_west(&mut grid);
      swap_south(&mut grid);
      swap_east(&mut grid);
    if let Some(seen_at) = seen.insert(grid.clone(), i) {
      if (1000000000 - i) % (i - seen_at) == 0 {
        break;
      }
    }
  }

  load(&grid)
}

fn part1(input: &str) -> u32 {
  let mut grid: Vec<Vec<char>> = input.lines().map(|line|line.chars().collect()).collect();
  let grid = transpose_and_rotate(grid);
  grid.into_iter().map(|line| process_line(line)).sum()
}

fn process_line(line: Vec<char>) -> u32 {
  let mut limit = 0;
  let mut acc = 0;
  for (idx, c) in line.iter().enumerate() {
    match c {
      'O' => if limit <= idx {
          acc += line.len() - limit;
          limit += 1;
        },
      '#' => limit = idx + 1,
      _ => {},
    }
  }
  acc as u32
}

fn swap_north(grid: &mut Vec<Vec<char>>) {
  let rows = grid.len();
  let cols = grid[0].len();

  for col in 0..cols {
      let mut last_free_spot = 0;

      for row in (0..rows) {
          match grid[row][col] {
              'O' => {
                      if last_free_spot < row {
                        grid[last_free_spot][col] = 'O';
                        grid[row][col] = '.';
                        last_free_spot += 1;
                      } else {
                        last_free_spot += 1
                      }
              },
              '#' => {
                  last_free_spot = row + 1;
              },
              _ => {},
          }
      }
  }
}

fn swap_south(grid: &mut Vec<Vec<char>>) {
  let rows = grid.len();
  let cols = grid[0].len();

  for col in 0..cols {
      let mut last_free_spot = rows - 1;

      for row in (0..rows).rev() {
          match grid[row][col] {
              'O' => {
                  if last_free_spot > row {
                      grid[last_free_spot][col] = 'O';
                      grid[row][col] = '.';
                      if let Some(new_spot) = last_free_spot.checked_sub(1) {
                          last_free_spot = new_spot;
                      }
                  } else if let Some(new_spot) = last_free_spot.checked_sub(1) {
                      last_free_spot = new_spot;
                  }
              },
              '#' => {
                  if let Some(new_spot) = row.checked_sub(1) {
                      last_free_spot = new_spot;
                  }
              },
              _ => {},
          }
      }
  }
}

fn swap_east(grid: &mut Vec<Vec<char>>) {
  let rows = grid.len();
  let cols = grid[0].len();

  for row in 0..rows {
      let mut last_free_spot = cols - 1;

      for col in (0..cols).rev() {
          match grid[row][col] {
              'O' => {
                  if last_free_spot > col {
                      grid[row][last_free_spot] = 'O';
                      grid[row][col] = '.';
                      if let Some(new_spot) = last_free_spot.checked_sub(1) {
                          last_free_spot = new_spot;
                      }
                  } else if let Some(new_spot) = last_free_spot.checked_sub(1) {
                      last_free_spot = new_spot;
                  }
              },
              '#' => {
                  if let Some(new_spot) = col.checked_sub(1) {
                      last_free_spot = new_spot;
                  }
              },
              _ => {},
          }
      }
  }
}

fn swap_west(grid: &mut Vec<Vec<char>>) {
  let rows = grid.len();
  let cols = grid[0].len();

  for row in 0..rows {
      let mut last_free_spot = 0;

      for col in 0..cols {
          match grid[row][col] {
              'O' => {
                  if last_free_spot < col {
                      grid[row][last_free_spot] = 'O';
                      grid[row][col] = '.';
                      last_free_spot += 1;
                  } else {
                      last_free_spot += 1;
                  }
              },
              '#' => {
                  last_free_spot = col + 1;
              },
              _ => {},
          }
      }
  }
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let input = include_str!("input14.txt");
    println!("{:?}", part2(input));
    Ok(())
}