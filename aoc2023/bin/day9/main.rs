
fn part1(history: Vec<i64>) -> i64 {
    let mut acc = history.last().copied().unwrap();
    let next_difference: Vec<i64> = history.windows(2).map(|slice| slice[1] - slice[0]).collect();
    if !next_difference.iter().all(|&x| x == 0) {
        acc += part1(next_difference);
    }
    acc
}
fn part2(history: Vec<i64>) -> i64 {
    let mut acc = history.first().copied().unwrap();
    let next_difference: Vec<i64> = history.windows(2).map(|slice| slice[1] - slice[0]).collect();
    if !next_difference.iter().all(|&x| x == 0) {
        acc -= part2(next_difference);
    }
    acc
}
fn parse(input: &str) -> i64 {
    input.lines().map(|line| {
        let history: Vec<i64> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();
        part2(history)
    }).sum()
}

fn main() {
    let input = include_str!("input9.txt");
    println!("{}", parse(input));
}
