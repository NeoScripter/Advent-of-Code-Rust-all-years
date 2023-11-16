use itertools::Itertools;
use color_eyre::eyre::{self};

struct Instruction {
    amount: usize,
    from: usize,
    to: usize,
}

fn parse_input() -> Option<(Vec<Vec<char>>, Vec<Instruction>)> {
    let input = include_str!("input5.txt");
    let binding = input.replace("\r\n", "\n");
    let (left, instructions_str) = binding.split_once("\n\n")?;
    let binding1 = left.replace("\r\n", "\n");
    let (stack_str, platforms) = binding1.rsplit_once('\n')?;

    let num_stacks = platforms.split_whitespace().last()?.parse().ok()?;
    let mut stacks = vec![Vec::new(); num_stacks];

    for line in stack_str.lines().rev() {
        for (idx, mut chunk) in line.chars().chunks(4).into_iter().enumerate() {
            let second = chunk.nth(1).unwrap_or_default();
            if second.is_alphabetic() {
                stacks[idx].push(second);
            }
        }
    }
    let mut instructions = Vec::new();
    for line in instructions_str.lines() {
        let rest = line.strip_prefix("move ")?;
        let (amount, rest) = rest.split_once(" from ")?;
        let (from, to) = rest.split_once(" to ")?;
        let instruction = Instruction {
            amount: amount.parse().ok()?,
            from: from.parse::<usize>().ok()? - 1,
            to: to.parse::<usize>().ok()? - 1,
        };
        instructions.push(instruction);
    }

    Some((stacks, instructions))
}

fn main() -> color_eyre::Result<()> {
    let (mut stacks, instructions) = parse_input().unwrap();
    for Instruction { amount, from, to } in instructions {
        let from_stack_len = stacks[from].len();
        let removed = stacks[from].split_off(from_stack_len - amount);
        stacks[to].extend(removed);
    }
    let result: String = stacks
        .iter()
        .filter_map(|stack| stack.iter().last())
        .collect();

    println!("{}", result);
    Ok(())
}