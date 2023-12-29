#![allow(unused)]
#![allow(dead_code)]
#![allow(unused_variables)]
use itertools::Itertools;
use std::{collections::{HashSet, HashMap}, fmt, str::FromStr};
use anyhow::{Result, anyhow};

#[derive(Debug, Clone, Hash, Eq, PartialEq, Copy)]
enum Category {
    X,
    M,
    A,
    S,
}
impl FromStr for Category {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "a" => Ok(Category::A),
            "x" => Ok(Category::X),
            "m" => Ok(Category::M),
            "s" => Ok(Category::S),
            _ => Err(anyhow! ("Error parsing the category")),
        }
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
enum Sign {
    Less,
    Greater,
}
impl FromStr for Sign {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "<" => Ok(Sign::Less),
            ">" => Ok(Sign::Greater),
            _ => Err(anyhow! ("Error parsing the sign")),
        }
    }
}

#[derive(Debug, Clone, Hash)]
struct Condition {
    cater: Option<Category>,
    sign: Option<Sign>,
    value: Option<u32>,
    send_to: String,
}

#[derive(Debug, Clone)]
struct Workflows {
    cnds: HashMap<String, Vec<Condition>>,
    seen: HashSet<String>,
    rejected: Vec<HashMap<Category, u32>>,
    accepted: Vec<HashMap<Category, u32>>,
}

impl Workflows {
    fn new() -> Self {
        Self {
            cnds: HashMap::new(),
            seen: HashSet::new(),
            rejected: Vec::new(),
            accepted: Vec::new(),
        }
    }
    fn sort_parts(&mut self, name: String, part: HashMap<Category, u32>) {
        let cur_inst = self.cnds.get(&name).unwrap();
        for cnd in cur_inst {
            match cnd.cater {
                Some(ctg) => {
                     if !self.seen.insert(cnd.send_to.clone()) {
                        continue;
                     }
                     let num = part.get(&ctg).unwrap();
                     match cnd.sign.unwrap() {
                        Sign::Less => {
                            if cnd.value.unwrap() < *part.get(&ctg).unwrap() {
                                if !self.seen.insert(cnd.send_to.clone()) {
                                    continue;
                                 }
                                 self.sort_parts(cnd.send_to, part)
                            }
                        },
                        _ => {

                        },
                     }
                },
                None => {
                    match cnd.send_to.as_str() {
                        "A" => &self.accepted.push(part),
                        "R" => &self.rejected.push(part),
                        _ => self.sort_parts(cnd.send_to, part),
                    }
                },
            }
        }
    }
}

fn part1(input: &str) -> Result<u32> {
    let (wfs_map, parts) = input.split_once("\r\n\r\n").expect("Error parsing input");
    let ctgs = [Category::X, Category::M, Category::A, Category::S];
    let part_map: Vec<HashMap<Category, u32>> = parts.lines().map(|line| {
        ctgs.iter().copied().zip(line.split(|c| c == '=' || c == '}' || c == ',')
            .filter_map(|part| part.parse::<u32>().ok()))
            .collect::<HashMap<Category, u32>>()
    }).collect();

    let mut wfs = Workflows::new();

    for line in wfs_map.lines() {
        let (name, rest) = line.split_once('{').expect("error splitting at {");
        let cnds = rest.strip_suffix('}').expect("error stripping }");
    
        let vec: Vec<Condition> = cnds.split(',').map(|cnd| {
            if let Some((ins, send_to)) = cnd.split_once(':') {
                let parts: Vec<&str> = ins.splitn(4, "").collect();
                Condition {
                    cater: parts[1].parse().ok(),
                    sign: parts[2].parse().ok(),
                    value: parts[3].parse::<u32>().ok(),
                    send_to: send_to.to_string(),
                }
            } else {
                Condition {
                    cater: None,
                    sign: None,
                    value: None,
                    send_to: cnd.to_string(),
                }
            }
        }).collect();
    
        wfs.cnds.insert(name.to_string(), vec);
    }
    
    println!("{:?}", wfs);
    Ok(62)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_output() {
        let input = include_str!("input_lib.txt");
        assert_eq!(62, part1(input).unwrap());
    }
}