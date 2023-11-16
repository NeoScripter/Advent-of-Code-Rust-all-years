use nom::{
    sequence::tuple,
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::map_res,
    IResult,
};
#[derive(Debug)]
enum Action {
    Turnon,
    Turnoff,
    Toggle,
}
#[derive(Debug)]
struct Commands {
    x_left: usize,
    x_right: usize,
    y_left: usize,
    y_right: usize,
}
#[derive(Debug)]
struct Instruction {
    action: Action,
    com: Commands,
}
#[derive(Debug, Default)]
struct Grid(Vec<Vec<u32>>);

impl Grid {
    fn new() -> Grid {
        Grid(vec![vec![0; 1000]; 1000])
    }
    fn turn_on(&mut self, h_left: usize, v_left: usize, h_right: usize, v_right: usize) {
        for y in v_left..=v_right {
            for x in h_left..=h_right {
                self.0[y][x] += 1
            }
        }
    }
    fn turn_off(&mut self, h_left: usize, v_left: usize, h_right: usize, v_right: usize) {
        for y in v_left..=v_right {
            for x in h_left..=h_right {
                self.0[y][x] -= 1
            }
        }
    }
    fn toggle(&mut self, h_left: usize, v_left: usize, h_right: usize, v_right: usize) {
        for y in v_left..=v_right {
            for x in h_left..=h_right {
                self.0[y][x] += 2
            }
        }
    }
    fn count_lights(&self) -> u32 {
        self.0.iter().flat_map(|v| v.iter()).sum()
    }
}
fn parse(input: &str) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    for line in input.lines() {
        if let Ok((_, (action, x_left, y_left, x_right, y_right))) = parse_instr(line.trim()) {
            let com = Commands { x_left, x_right, y_left, y_right };
            let action = match action.trim() {
                "turn on" => Action::Turnon,
                "toggle" => Action::Toggle,
                "turn off" => Action::Turnoff,
                _ => continue,
            };

            instructions.push(Instruction { action, com });
        }
    }
    println!("{:?}", &instructions);
    instructions
}
fn parse_instr(s: &str) -> IResult<&str, (&str, usize, usize, usize, usize)> {
    let mut parser = tuple((
        alt((tag("turn on "), tag("toggle "), tag("turn off "))),
        map_res(digit1, |digit: &str| digit.parse::<usize>()),
        tag(","),
        map_res(digit1, |digit: &str| digit.parse::<usize>()),
        tag(" through "),
        map_res(digit1, |digit: &str| digit.parse::<usize>()),
        tag(","),
        map_res(digit1, |digit: &str| digit.parse::<usize>()),
    ));
    let (remainder, (action, x_start, _, y_start, _, x_end, _, y_end)) = parser(s)?;
    Ok((remainder, (action, x_start, x_end, y_start, y_end)))
}
fn test_part1(input: &str) -> u32 {
    let instructions = parse(input);
    let mut grid = Grid::new();
    for ins in instructions {
        match ins.action {
            Action::Turnon => grid.turn_on(ins.com.x_left, ins.com.x_right, ins.com.y_left, ins.com.y_right),
            Action::Toggle => grid.toggle(ins.com.x_left, ins.com.x_right, ins.com.y_left, ins.com.y_right),
            Action::Turnoff => grid.turn_off(ins.com.x_left, ins.com.x_right, ins.com.y_left, ins.com.y_right),
        }
    }
    grid.count_lights()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(1, test_part1("turn on 0,0 through 0,0"));
    }
    #[test]
    fn test_2() {
        assert_eq!(2_000_000, test_part1("toggle 0,0 through 999,999"));
    }
}