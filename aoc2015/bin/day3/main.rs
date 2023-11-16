use std::collections::HashSet;

type Coord = (i32, i32);

fn part1(input: &str) -> usize {
    let mut locs = HashSet::new();
    let mut start: Coord = (0, 0);
    locs.insert(start);

    for c in input.chars() {
        match c {
            '>' => start.0 += 1,
            '^' => start.1 += 1,
            '<' => start.0 -= 1,
            'v' => start.1 -= 1,
            _ => panic!("Invalid input"),
        }
        locs.insert(start);
    }
    locs.len()
}

fn part2(input: &str) -> usize {
    let mut locs = HashSet::new();
    let mut santa: Coord = (0, 0);
    let mut robo_santa: Coord = (0, 0);
    locs.insert(santa);

    for (idx, c) in input.chars().enumerate() {
        match c {
            '>' => {if idx % 2 == 0 {santa.0 += 1} else {robo_santa.0 += 1}},
            '^' => {if idx % 2 == 0 {santa.1 += 1} else {robo_santa.1 += 1}},
            '<' => {if idx % 2 == 0 {santa.0 -= 1} else {robo_santa.0 -= 1}},
            'v' => {if idx % 2 == 0 {santa.1 -= 1} else {robo_santa.1 -= 1}},
            _ => panic!("Invalid input"),
        }
        locs.insert(santa);
        locs.insert(robo_santa);
    }
    locs.len()
}
fn main() {
    let input = include_str!("input3.txt");
    println!("Part 1 result: {}\r\nPart 2 result: {}", part1(input), part2(input));
}