use itertools::Itertools;

fn main() {
    let input = include_str!("input3.txt");

    let result: u32 = input
        .lines()
        .map(|line| line.as_bytes())
        .tuples()
        .filter_map(|(sack1, sack2, sack3)| {
            sack1
                .iter()
                .find(|item| sack2.contains(item) && sack3.contains(item))
                .map(|item| match item {
                    b'a'..=b'z' => (item - b'a') + 1,
                    _ => (item - b'A') + 1 + 26,
                } as u32)
        }).sum();

    println!("{}", result);
}
