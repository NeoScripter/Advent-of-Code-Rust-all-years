use itertools::Itertools;
fn part1(input: &str) -> u32 {
    let result: u32 = input.lines().map(|line| {
        let numbers: Vec<u32> = line.split("x").filter_map(|n| n.parse().ok()).collect();
        let (min, sum) = numbers.iter()
            .combinations(2)
            .map(|p| p.iter().copied().product())
            .fold((u32::MAX, 0), |(min, sum), prod| (min.min(prod), sum + prod));
        min + sum * 2
    }).sum();
    result 
}
fn part2(input: &str) -> u32 {
    let result: u32 = input.lines().map(|line| {
        let mut numbers: Vec<u32> = line.split("x").filter_map(|n| n.parse().ok()).collect();
        numbers.sort_unstable();
        numbers.iter().take(2).sum::<u32>() * 2 + numbers.iter().product::<u32>()
    }).sum();
    result 
}
fn main() {
    let input = include_str!("input2.txt");
    println!("{}, {}", part1(input), part2(input));
}