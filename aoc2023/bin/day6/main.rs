
fn part1(input: &str) -> usize {
    let document = input.lines().flat_map(|line| {
        line.split_once(": ").unwrap().1.split_whitespace()
            .map(|digit| digit.parse::<u32>().unwrap())
    });

    let len = document.clone().count() / 2;
    document.clone().take(len).zip(document.skip(len))
        .map(|(time, distance)| (1..time).filter(|&speed| (time - speed) * speed > distance).count())
        .product()
}
fn part2(input: &str) -> usize {
    let (time, distance) = input.lines().map(|line| {
        line.split_once(": ").unwrap().1.replace(" ", "")
            .parse::<u64>().unwrap()
    }).fold((0, 0), |(t, d), val| if t == 0 { (val, d) } else { (t, val) });

    (1..time).filter(|&speed| (time - speed) * speed > distance).count()
}
fn main() {
    let input = include_str!("input6.txt");
    println!("{}, {}", part1(input), part2(input));
}