use itertools::Itertools;
use std::collections::HashMap;

fn test_part1(input: &str) -> String {
    let mut answer = String::new();
    let init_input: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let width = init_input[0].len();
    let transposed: Vec<Vec<char>> = (0..width).map(|i| {
        init_input.iter().map(|row| row[i]).collect()
    }).collect();
    for line in transposed.into_iter() {
        let mut map = HashMap::new();
        line.iter().for_each(|&c| *map.entry(c).or_insert(0) += 1);
        let (max_key, _max_value) = map
        .into_iter()
        .max_by(|a, b| b.1.cmp(&a.1))
        .unwrap();
        answer.push(max_key);
    }
    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!("advent".to_string(), test_part1(include_str!("input_lib.txt")));
    }
}