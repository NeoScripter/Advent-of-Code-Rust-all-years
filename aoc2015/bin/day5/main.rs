fn part1(input: &str) -> usize {
    let vowels = &['a', 'e', 'i', 'o', 'u'];
    let naughty_patterns = &["ab", "cd", "pq", "xy"];
    input.lines()
    .filter(|line| line.chars().filter(|c| vowels.contains(c)).count() >= 3)
    .filter(|line| {
        line.as_bytes()
            .windows(2)
            .any(|window| window[0] == window[1])
    })
    .filter(|line| !naughty_patterns.iter().any(|pat| line.contains(pat)))
    .count()
}

fn part2(input: &str) -> usize {
    input.lines()
    .filter(|line| {
        line.as_bytes().windows(2).enumerate().any(|(i, pair)| {
            if let Some(idx) = line.rfind(std::str::from_utf8(pair).unwrap()) {
                idx > i + 1
            } else {
                false
            }
        })
    })
    .filter(|line| {
        line.as_bytes()
            .windows(3)
            .any(|window| window[0] == window[2])
    })
    .count()
}
fn main() {
    let input = include_str!("input5.txt");
    println!("{}, {}", part1(input), part2(input));
}