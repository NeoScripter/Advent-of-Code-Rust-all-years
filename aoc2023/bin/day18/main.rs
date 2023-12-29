//This is not my solution

fn calc_area(movements: impl Iterator<Item=(u8, isize)>) -> isize {
    let (mut area, mut row, mut col) = (0, 0, 0);
    for (direction, steps) in movements {
        let (prev_row, prev_col) = (row, col);
        match direction {
            b'U' => row -= steps,
            b'R' => col += steps,
            b'D' => row += steps,
            b'L' => col -= steps,
            _ => unreachable!()
        };
        area += (col + prev_col) * (row - prev_row) + steps;
    }
    area / 2 + 1
}
  
fn solve(input: &str) -> (isize, isize) {
    let path1 = input.lines().map(|line| {
        let (steps_str, _) = line[2..].split_once(' ').unwrap();
        (line.as_bytes()[0], steps_str.parse::<isize>().unwrap())
    });

    let path2 = input.lines().map(|line| {
        let (_, color_code) = line.split_once(" (#").unwrap();
        let direction = match color_code.as_bytes()[color_code.len() - 2] {
            b'0' => b'R',
            b'1' => b'D',
            b'2' => b'L',
            b'3' => b'U',
            _ => unreachable!(),
        };
        (direction, isize::from_str_radix(&color_code[0..color_code.len() - 2], 16).unwrap())
    });

    (calc_area(path1), calc_area(path2))
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let input = include_str!("input18.txt");
    println!("{:?}", solve(input).1);
    Ok(())
}