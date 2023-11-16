fn main() {
    let input = include_str!("input6.txt");
    input.as_bytes()
        .windows(14)
        .position(|window| {
            window
                .iter()
                .enumerate()
                .all(|(idx, c)| !window[..idx].contains(c))
        })
        .unwrap()
        + 14;

    println!("{:?}", input);
}