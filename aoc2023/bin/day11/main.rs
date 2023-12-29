use color_eyre::eyre::{self, Result};
use std::{fmt, cmp};
use itertools::Itertools;

#[derive(Debug, Clone)]
struct Image<T>(Vec<Vec<T>>)
where
    T: fmt::Display + PartialEq + Copy; 

impl<T> fmt::Display for Image<T>
where
    T: fmt::Display + PartialEq + Copy,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.0 {
            for num in row {
                write!(f, "{:>1}", num)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
impl<T> Image<T>
where
    T: fmt::Display + PartialEq + Clone + Copy,
{
    fn find_indexes_of_empty_lines(&self) -> (Vec<i64>, Vec<i64>) {
        let mut empty_rows = Vec::new();
        for (row_idx, row) in self.0.iter().enumerate() {
            if row.iter().all_equal() {
                empty_rows.push(row_idx as i64)
            }
        }
        let mut empty_columns = Vec::new();
        for col_idx in 0..self.0[0].len() {
            let mut column = Vec::new();
            for col in self.0.iter() {
                column.push(col[col_idx]);
            }
            if column.iter().all_equal() {
                empty_columns.push(col_idx as i64)
            }
        }
        (empty_rows, empty_columns)
    }
    fn find_galaxies<F>(&self, predicate: F) -> Vec<(i64, i64)>
    where
        F: Fn(&T) -> bool,
    {
        let mut coordinates = Vec::new();
        for (y, row) in self.0.iter().enumerate() {
            for (x, element) in row.iter().enumerate() {
                if predicate(element) {
                    coordinates.push((y as i64, x as i64));
                }
            }
        }
        coordinates
    }
}

fn run(input: &str, expansion_rate: i64) -> i64 {
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|s| s.chars().collect())
        .collect();

    let image = Image(grid);
    let (ref empty_rows, ref empty_columns) = image.find_indexes_of_empty_lines();
    let galaxies = image.find_galaxies(|&element| element == '#');
    galaxies.iter().flat_map(|&(x1, y1)| {
        galaxies.iter().map(move |&(x2, y2)| {
            let x_diff = empty_rows.iter().filter(|&&n| n > x1.min(x2) && n < x1.max(x2)).count() as i64;
            let y_diff = empty_columns.iter().filter(|&&n| n > y1.min(y2) && n < y1.max(y2)).count() as i64;
    
            (x1 - x2).abs() as i64 + x_diff * (expansion_rate - 1) + (y1 - y2).abs() as i64 + y_diff * (expansion_rate - 1)
        })
    }).sum::<i64>() / 2  
}
fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let expansion_rate = 1_000_000;
    let input = include_str!("input11.txt");
    println!("{}", run(input, expansion_rate));
    Ok(())
}