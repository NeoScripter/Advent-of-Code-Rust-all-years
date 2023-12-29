use std::collections::HashSet;

#[derive(Debug, Clone)]
struct Num {
    num: u32,
    area: HashSet<(usize, usize)>,
}

impl Num {
    fn new(n: u32, area: HashSet<(usize, usize)>) -> Self {
        Self { num: n, area }
    }
}

fn part1(input: &str) -> u32 {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut numbers: Vec<Num> = Vec::new();
    let mut sign_map = HashSet::new();

    for (idx1, x) in grid.into_iter().enumerate() {
        let mut digit = 0;
        let mut area = HashSet::new();

        for (idx2, ch) in x.into_iter().enumerate() {
            match ch {
                '.' => push_num(&mut numbers, &mut digit, &mut area),
                _ if ch.is_digit(10) => {
                    add_coord(&mut area, idx1, idx2);
                    digit = digit * 10 + ch.to_digit(10).unwrap();
                },
                _ => {
                    push_num(&mut numbers, &mut digit, &mut area);
                    sign_map.insert((idx1, idx2));
                }
            }
        }
        push_num(&mut numbers, &mut digit, &mut area);
    }

    numbers.iter().filter(|n| n.area.iter().any(|coord| sign_map.contains(coord))).map(|n| n.num).sum()
}

fn part2(input: &str) -> u32 {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut numbers: Vec<Num> = Vec::new();
    let mut gear_map = HashSet::new();

    for (idx1, x) in grid.into_iter().enumerate() {
        let mut digit = 0;
        let mut area = HashSet::new();

        for (idx2, ch) in x.into_iter().enumerate() {
            match ch {
                '*' => {
                    gear_map.insert((idx1, idx2));
                    push_num(&mut numbers, &mut digit, &mut area);
                },
                _ if ch.is_digit(10) => {
                    add_coord(&mut area, idx1, idx2);
                    digit = digit * 10 + ch.to_digit(10).unwrap();
                },
                _ => push_num(&mut numbers, &mut digit, &mut area),
            }
        }
        push_num(&mut numbers, &mut digit, &mut area);
    }

    gear_map.iter().fold(0, |mut answer, &coord| {
        let mut last_num = 0;
        let mut counter = 0;

        for n in &numbers {
            if n.area.contains(&coord) {
                if counter > 0 {
                    answer += last_num * n.num;
                    counter = 0;
                }
                last_num = n.num;
                counter += 1;
            }
        }
        answer
    })
}

fn add_coord(area: &mut HashSet<(usize, usize)>, x: usize, y: usize) {
    if let Some(x1) = x.checked_sub(1) {
        area.insert((x1, y));
        area.insert((x1, y + 1));
        if let Some(y1) = y.checked_sub(1) {
            area.insert((x1, y1));
        }
    }
    if let Some(y1) = y.checked_sub(1) {
        area.insert((x, y1));
        area.insert((x + 1, y1));
    }
    area.insert((x, y + 1));
    area.insert((x + 1, y));
    area.insert((x + 1, y + 1));
}
fn push_num(numbers: &mut Vec<Num>, digit: &mut u32, area: &mut HashSet<(usize, usize)>) {
    numbers.push(Num::new(*digit, area.clone()));
    area.clear();
    *digit = 0;
}


fn main() {
    let input = include_str!("input3.txt");
    println!("{}, {}", part1(input), part2(input));
}
