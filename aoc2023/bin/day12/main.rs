use color_eyre::eyre::{self, Result};
use itertools::Itertools;
use std::collections::HashMap;

use std::str::FromStr;

fn is_valid(springs: &str, instrs: &[usize]) -> bool {
    let springs = springs
        .split('.')
        .filter(|a| !a.is_empty())
        .collect::<Vec<_>>();

    if springs.len() != instrs.len() {
        return false;
    }

    springs
        .iter()
        .zip(instrs.iter())
        .all(|(spring, instr)| spring.len() == *instr)
}

fn get_combinations(springs: &str, instrs: &[usize]) -> usize {
    if springs.find('?').is_some() {
        let with_spring = springs.replacen('?', "#", 1);
        let without_spring = springs.replacen('?', ".", 1);
        get_combinations(&with_spring, instrs) + get_combinations(&without_spring, instrs)
    } else {
        match is_valid(springs, instrs) {
            true => 1,
            false => 0,
        }
    }
}

pub fn solve(input: &str) -> usize {
    let lines: Vec<_> = input
        .lines()
        .map(|line| line.split(' ').collect::<Vec<_>>())
        .map(|line| {
            (
                line[0].to_string(),
                line[1]
                    .split(',')
                    .map(usize::from_str)
                    .map(|a| a.unwrap())
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>();

    lines
        .iter()
        .map(|(a, b)| get_combinations(a, b))
        .sum::<usize>()
}


fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let input = include_str!("input12.txt");
    println!("{}", solve(input));
    Ok(())
}