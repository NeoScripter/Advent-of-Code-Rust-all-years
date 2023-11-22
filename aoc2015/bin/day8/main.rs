fn part1(input: &str) -> usize {
    input.lines().map(|line| {
        let literal = line.len();
        let mut in_memory = 0;
        let line = &line[1..literal - 1];
        let mut chars = line.chars().peekable();
        while let Some(c) = chars.next() {
            in_memory += 1;
            if c == '\\' {
                let next = chars.peek();
                if let Some('x') = next {
                    chars.next();
                    chars.next();
                    chars.next();
                } else {
                    chars.next();
                }
            }
        }
        literal - in_memory
    }).sum::<usize>()
}
fn part2(input: &str) -> usize {
    input.lines().map(|line| {
        let literal = line.len();
        let in_memory = line
        .chars()
        .map(|c| match c {
            '\\' | '"' => 2,
            _ => 1,
        })
        .sum::<usize>()
        + 2;
        in_memory - literal
    }).sum::<usize>()
}
fn main() {
    let input = include_str!("input8.txt");
    println!("{}", part2(input));
}