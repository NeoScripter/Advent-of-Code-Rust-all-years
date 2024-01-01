use color_eyre::eyre;

use serde_json::{Value, from_str};
use itertools::EitherOrBoth;
use itertools::Itertools;

#[derive(Debug, Clone)]
enum Order {
    Right,
    Wrong,
    Unknown,
}

fn compare(left: &Value, right: &Value) -> Order {
    match (left, right) {
        (Value::Number(num_left), Value::Number(num_right)) => {
            let l = num_left.as_i64().unwrap_or(0);
            let r = num_right.as_i64().unwrap_or(0);
            if l < r {
                Order::Right
            } else if l > r {
                Order::Wrong
            } else {
                Order::Unknown
            }
        }
        (Value::Array(list_left), Value::Array(list_right)) => {
            for element in list_left.iter().zip_longest(list_right.iter()) {
                match element {
                    EitherOrBoth::Both(x, y) => match compare(x, y) {
                        Order::Unknown => continue,
                        order => return order,
                    },
                    EitherOrBoth::Left(_) => return Order::Wrong,
                    EitherOrBoth::Right(_) => return Order::Right,
                }
            }
            Order::Unknown
        }
        (Value::Array(_), _) => compare(left, &Value::Array(vec![right.clone()])),
        (_, Value::Array(_)) => compare(&Value::Array(vec![left.clone()]), right),
        _ => Order::Unknown,
    }
}

fn part1(input: &str) -> usize {
    let pairs: Vec<&str> = input.split("\r\n\r\n").collect();
    let mut sum = 0;
    for (idx, pair) in pairs.iter().enumerate() {
        if let Some((left, right)) = pair.split_once("\r\n") {
            let left_json = from_str(left).expect("Failed to parse JSON");
            let right_json = from_str(right).expect("Failed to parse JSON");
            match compare(&left_json, &right_json) {
                Order::Right => sum += idx + 1,
                _ => {},
            }
        }
    }
    sum
}
fn part2(input: &str) -> usize {
    let mut pairs = input
        .replace("[]", "[0]")
        .split("\r\n")
        .filter(|&pair| !pair.is_empty())
        .chain(["[[2]]", "[[6]]"].iter().copied())
        .map(|line| line.split(|c| c == ',' || c == '[' || c == ']').filter_map(|x| x.parse::<u8>().ok()).collect::<Vec<u8>>())
        .collect::<Vec<_>>();

    pairs.sort_by_key(|v| (v.first().copied(), v.len()));
    
    let mut mul = 1;
    for (idx, pair) in pairs.iter().enumerate() {
        if pair == &[2] || pair == &[6] {
            mul *= idx + 1
        }
    }
    mul
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let input = include_str!("input13.txt");
    println!("{}", part2(input));
    Ok(())
}