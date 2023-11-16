fn part1(input: &str) -> i32 {
    let result = input
    .chars()
    .fold(0, |mut floor, c| {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => panic!("invalid input"),
        }
        floor
    });
    result
}
fn part2(input: &str) -> usize {
    let result = input
        .chars()
        .scan(0, |floor, c| {
            match c {
                '(' => *floor += 1,
                ')' => *floor -= 1,
                _ => panic!("invalid input"),
            }
            Some(*floor)
        })
        .position(|floor| floor < 0)
        .unwrap()
        + 1;
    result
}
fn main() {
    let input = include_str!("input1.txt");
    println!("{}, {}", part1(input), part2(input));
}