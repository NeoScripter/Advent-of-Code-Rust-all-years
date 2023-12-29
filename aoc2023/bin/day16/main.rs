use std::collections::HashSet;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
enum Dir {
    Up,
    Down,
    Right,
    Left,
}
#[derive(Debug)]
struct Grid {
    ctrp: Vec<Vec<char>>,
    repeated: HashSet<(usize, usize, Dir)>,
    energized: HashSet<(usize, usize)>,
}

impl Grid {
    fn new(grid: Vec<Vec<char>>) -> Grid {
        Grid {
            ctrp: grid,
            repeated: HashSet::new(),
            energized: HashSet::new(),
        }
    }
    fn beam(&mut self, mut direction: Dir, mut x: usize, mut y: usize) {
        loop {
            if !self.repeated.insert((y, x, direction.clone())) {
                break;
            }
            self.energized.insert((y, x));
            
            match self.ctrp[y][x] {
                '/' => direction = match direction {
                    Dir::Up => Dir::Right,
                    Dir::Down => Dir::Left,
                    Dir::Right => Dir::Up,
                    Dir::Left => Dir::Down,
                },
                '\\' => direction = match direction {
                    Dir::Up => Dir::Left,
                    Dir::Down => Dir::Right,
                    Dir::Right => Dir::Down,
                    Dir::Left => Dir::Up,
                },
                '-' => if direction == Dir::Up || direction == Dir::Down {
                    self.beam(Dir::Right, x, y);
                    direction = Dir::Left;
                },
                '|' => if direction == Dir::Left || direction == Dir::Right {
                    self.beam(Dir::Up, x, y);
                    direction = Dir::Down;
                },
                _ => {},
            }
    
            match direction {
                Dir::Up if y > 0 => y -= 1,
                Dir::Down if y < self.ctrp.len() - 1 => y += 1,
                Dir::Left if x > 0 => x -= 1,
                Dir::Right if x < self.ctrp[0].len() - 1 => x += 1,
                _ => break,
            }
        }
    }    
}
fn part1(input: &str) -> usize {
    let parsed: Vec<Vec<char>> = input.lines().map(|line| {
        line.chars().collect()
    }).collect();
    let mut grid = Grid::new(parsed);
    grid.beam(Dir::Right, 0, 0);
    grid.energized.len()
}

fn part2(input: &str) -> usize {
    let parsed: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let x_len = parsed[0].len() - 1;
    let y_len = parsed.len() - 1;
    let mut grid = Grid::new(parsed);
    let mut max = 0;
    // Corners
    process_grid(&mut grid, Dir::Right, 0, 0, &mut max);
    process_grid(&mut grid, Dir::Down, 0, 0, &mut max);
    process_grid(&mut grid, Dir::Left, x_len, 0, &mut max);
    process_grid(&mut grid, Dir::Down, x_len, 0, &mut max);
    process_grid(&mut grid, Dir::Up, 0, y_len, &mut max);
    process_grid(&mut grid, Dir::Right, 0, y_len, &mut max);
    process_grid(&mut grid, Dir::Up, x_len, y_len, &mut max);
    process_grid(&mut grid, Dir::Left, x_len, y_len, &mut max);

    // Edges excluding corners
    for x in 1..x_len {
        process_grid(&mut grid, Dir::Down, x, 0, &mut max);
        process_grid(&mut grid, Dir::Up, x, y_len, &mut max);
    }
    for y in 1..y_len {
        process_grid(&mut grid, Dir::Right, 0, y, &mut max);
        process_grid(&mut grid, Dir::Left, x_len, y, &mut max);
    }

    max
}

fn process_grid(grid: &mut Grid, direction: Dir, x: usize, y: usize, max: &mut usize) {
    grid.beam(direction, x, y);
    *max = grid.energized.len().max(*max);
    grid.repeated.clear();
    grid.energized.clear();
}
fn main() {
    let input = include_str!("input16.txt");
    println!("{}", part2(input));
}