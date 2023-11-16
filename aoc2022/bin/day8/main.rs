use itertools::Itertools;

fn parse() -> Vec<Vec<u32>> {
    let input = include_str!("input8.txt");

    input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

fn directions(grid: &[Vec<u32>], x: usize, y: usize) -> [Vec<u32>; 4] {
    let row = grid[y].clone();
    let column = grid.iter().map(|row| row[x]).collect::<Vec<u32>>();

    let (up, down) = column.split_at(y);
    let (left, right) = row.split_at(x);

    let up = up.iter().copied().rev().collect();
    let left = left.iter().copied().rev().collect();
    let right = right[1..].to_vec();
    let down = down[1..].to_vec();

    [up, down, left, right]
}

fn main() {
    let trees = parse();
    let len = trees.len();

    let result: Vec<usize> = (1..len - 1)
        .cartesian_product(1..len - 1)
        .map(|(y, x)| {
            let height = trees[y][x];
            directions(&trees, x, y)
                .iter()
                .map(|direction| {
                    let mut count = 0;
                    for &h in direction.iter() {
                        count += 1;
                        if h >= height {
                            break;
                        }
                    }
                    count
                })
                .product()
        }).collect();

    println!("{:?}", result.iter().max());
}