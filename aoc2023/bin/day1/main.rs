fn part1(input: &str) -> u32 {
    input.lines().map(|line| {
        let left = line.chars().find(|c| c.is_digit(10)).unwrap();
        let right = line.chars().rfind(|c| c.is_digit(10)).unwrap();
        let digit = format!("{}{}", left, right);
        digit.parse::<u32>().unwrap()
    }).sum::<u32>()
}
fn part2(input: &str) -> u32 {
    let mut input = input.to_string();
    let spelled_num = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let digit_num = ["o1e", "t2", "3e", "4", "5e", "6", "7n", "8t", "9e"];
    for (s, d) in spelled_num.into_iter().zip(digit_num) {
        input = input.replace(s, d);
    }
    input.lines().map(|line| {
        let left = line.chars().find(|c| c.is_digit(10)).unwrap();
        let right = line.chars().rfind(|c| c.is_digit(10)).unwrap();
        let digit = format!("{}{}", left, right);
        digit.parse::<u32>().unwrap()
    }).sum::<u32>()
}
fn main() {
    let input = include_str!("input1.txt");
    println!("{}, {}", part1(input), part2(input));
}