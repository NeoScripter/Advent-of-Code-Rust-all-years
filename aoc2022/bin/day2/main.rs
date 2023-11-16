fn main() {
    let input = include_str!("input2.txt");

    let result = input
        .lines()
        .map(|line| {
            // transform A B C and X Y Z to 0 1 2 respectively by using their ASCII order
            let bytes = line.as_bytes();
            let left = (bytes[0] - b'A') as i8;
            let right = (bytes[2] - b'X') as i8;

            // 0 for rock, 1 for paper, 2 for scissors
            // 0 for loss, 1 for draw, 2 for win
            let opponent_shape = left;
            let outcome = right;
            let my_shape = (outcome + opponent_shape - 1).rem_euclid(3);

            let shape_score = my_shape + 1;
            let outcome_score = 3 * outcome;
            (shape_score + outcome_score) as u32
        })
        .sum::<u32>()
        .to_string();

    println!("{}", result);
}