use color_eyre::eyre;
use std::collections::{BinaryHeap, HashMap};

fn dijkstra(grid: &[&[u8]], min_steps_allowed: isize, max_steps_allowed: isize) -> i64 {
    let mut min_costs = HashMap::new();
    let mut priority_queue = BinaryHeap::from_iter([(0, (0, 0, (0, 0)))]);

    while let Some((negated_cost, (row, col, direction))) = priority_queue.pop() {
        let current_cost = -negated_cost;

        if (row, col) == (grid.len() - 1, grid[0].len() - 1) {
            return current_cost;
        }

        if min_costs.get(&(row, col, direction))
            .map_or(false, |&recorded_cost| current_cost > recorded_cost) {
            continue;
        }

        for (delta_row, delta_col) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            if direction == (delta_row, delta_col) || direction == (-delta_row, -delta_col) {
                continue;
            }
            let mut next_cost = current_cost;
            for step_distance in 1..=max_steps_allowed {
                let next_row = (row as isize + delta_row * step_distance) as usize;
                let next_col = (col as isize + delta_col * step_distance) as usize;
                if next_row >= grid.len() || next_col >= grid[0].len() {
                    break;
                }
                next_cost += (grid[next_row][next_col] - b'0') as i64;
                if step_distance < min_steps_allowed {
                    continue;
                }
                let next_key = (next_row, next_col, (delta_row, delta_col));
                if next_cost < *min_costs.get(&next_key).unwrap_or(&i64::MAX) {
                    min_costs.insert(next_key, next_cost);
                    priority_queue.push((-next_cost, next_key));
                }
            }
        }
    }

    unreachable!()
}

fn part1(input: &str) -> (i64, i64) {
  let grid = input.lines().map(str::as_bytes).collect::<Vec<_>>();
  (dijkstra(&grid, 1, 3), dijkstra(&grid, 4, 10))
}
fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let input = include_str!("input17.txt");
    println!("{}", part1(input).1);
    Ok(())
}