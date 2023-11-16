use std::collections::HashMap;
use std::path::PathBuf;

fn main() {
    let input = include_str!("input7.txt");
    let mut sizes = HashMap::new();
    let mut affected = Vec::new();

    for line in input.lines() {
        if line.starts_with("$ ls") || line.starts_with("dir") {
            continue;
        }

        let parts: Vec<_> = line.split_whitespace().collect();
        match parts[..] {
            ["$", "cd", ".."] => {
                affected.pop();
            }
            ["$", "cd", name] => {
                affected.push(name);
            }
            [size, _name] => {
                let size: u32 = size.parse().unwrap();
                for idx in 0..affected.len() {
                    let path = PathBuf::from_iter(&affected[..=idx]);
                    *sizes.entry(path).or_insert(0) += size;
                }
            }
            _ => {}
        };
    }

    let disk = 70_000_000;
    let needed = 30_000_000;
    let root = sizes.get(&PathBuf::from("/")).unwrap();
    let available = disk - root;

    let result = sizes
        .into_values()
        .filter(|size| available + size >= needed)
        .min()
        .unwrap();

    println!("{}", result);
}