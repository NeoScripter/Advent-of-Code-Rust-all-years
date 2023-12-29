use std::collections::HashMap;

fn part1(input: &str) -> u32 {
    let (instructions, nodes) = input.split_once("\r\n\r\n").unwrap();
    let mut left = HashMap::new();
    let mut right = HashMap::new();

    for line in nodes.lines() {
        let (origin, dest) = line.split_once(" = (").unwrap();
        let (dest_left, dest_right) = dest.strip_suffix(")").unwrap().split_once(", ").unwrap();
        left.insert(origin, dest_left);
        right.insert(origin, dest_right);
    }
    let mut steps = 0;
    let mut curr_origin = "AAA";
    'outer: loop {
        for instruction in instructions.chars() {
            steps += 1;
            curr_origin = match instruction {
                'L' => left.get(curr_origin).unwrap(),
                'R' => right.get(curr_origin).unwrap(),
                _ => panic!("Invalid input"),
            };
            if curr_origin == "ZZZ" {
                break 'outer;
            }
        }
    }
    steps
}

fn part2(input: &str) -> u64 {
    let (instructions, nodes) = input.split_once("\r\n\r\n").unwrap();
    let mut nodes_map: HashMap<&str, (&str, &str)> = HashMap::new();
    for line in nodes.lines() {
        let (origin, dest) = line.split_once(" = (").unwrap();
        let (dest_left, dest_right) = dest.strip_suffix(")").unwrap().split_once(", ").unwrap();
        nodes_map.insert(origin, (dest_left, dest_right));
    }
    let start_nodes: Vec<&str> = nodes_map
        .keys()
        .filter(|&k| k.ends_with('A'))
        .cloned()
        .collect();
    let mut all_steps: Vec<u64> = vec![];
    for start_node in start_nodes.iter() {
        let mut curr_node = *start_node;
        let mut steps: u64 = 0;
        let mut turn_idx: usize = 0;
        while !curr_node.ends_with('Z') {
            let turn: char = instructions.chars().nth(turn_idx).unwrap();
            curr_node = match turn {
                'L' => nodes_map.get(&curr_node).unwrap().0,
                'R' => nodes_map.get(&curr_node).unwrap().1,
                _ => panic!("Invalid input"),
            };
            steps += 1;
            turn_idx = (turn_idx + 1) % instructions.len();
        }
        all_steps.push(steps);
    }
    all_steps.iter().fold(1, |acc, x| num_integer::lcm(acc, *x))
}

fn main() {
    let input = include_str!("input8.txt");
    println!("{}", part2(input));
}
