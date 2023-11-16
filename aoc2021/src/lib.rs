fn part1(input: &str) -> i32 {
    let positions: Vec<i32> = input.split(",").filter_map(|x| x.parse::<i32>().ok()).collect();
    let clone = positions.clone();
    positions.iter().map(|pos| {
        clone.iter().map(|fuel| (pos - fuel).abs()).sum()
    }).min().unwrap()
}
fn part2(input: &str) -> i32 {
    let positions: Vec<i32> = input.split(",").filter_map(|x| x.parse::<i32>().ok()).collect();
    let max_position = *positions.iter().max().unwrap();

    (0..=max_position).map(|target_pos| {
        positions.iter().map(|pos| (1..=(target_pos - pos).abs()).fold(0, |acc, num| {
            acc + num
        })).sum()
    }).min().unwrap()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(168, part2(include_str!("input_lib.txt")));
    }
}
