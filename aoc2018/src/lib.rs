use std::collections::{HashSet, BTreeSet, HashMap};
use itertools::Itertools;
use nom::{
    sequence::tuple,
    bytes::complete::tag,
    character::complete::alpha1,
    IResult,
};
fn parse_ins(s: &str) -> IResult<&str, (char, char)> {
    let mut parser = tuple((
        tag("Step "),
        alpha1,
        tag(" must be finished before step "),
        alpha1,
        tag(" can begin."),
    ));
    let (remainder, (_, char1, _, char2, _)) = parser(s)?;
    Ok((remainder, (char1.chars().next().unwrap(), char2.chars().next().unwrap())))
}
type Step = char;
type Required = HashMap<Step, HashSet<Step>>;

#[derive(Debug, Eq, PartialEq, Hash)]
struct Dependency {
    step: Step,
    required: Step,
}
fn find_next_step(requirements: &Required, done: &HashSet<Step>, taken: &HashSet<Step>) -> Vec<Step> {
    let mut possible_steps: Vec<Step> = Vec::new();

    for (&step, deps) in requirements {
        if taken.contains(&step) {
            continue;
        }
        if deps.iter().all(|v| done.contains(v)) {
            possible_steps.push(step);
        }
    }

    possible_steps.sort();
    possible_steps.dedup();
    possible_steps.reverse();
    possible_steps
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Status {
    Idle,
    Working {
        step: Step,
        remaining: u32,
    },
}

fn apply_tick(workers: &mut Vec<Status>) -> Vec<Step> {
    let mut finished_steps: Vec<Step> = Vec::new();

    for index in 0..workers.len() {
        match workers[index] {
            Status::Idle => {}
            Status::Working { step, ref mut remaining } => {
                *remaining -= 1;
                if *remaining == 0 {
                    finished_steps.push(step);
                    workers[index] = Status::Idle;
                }
            }
        }
    }

    finished_steps
}
fn all_workers_idle(workers: &Vec<Status>) -> bool {
    workers.iter().all(|&w| w == Status::Idle)
}
fn available_workers(workers: &Vec<Status>) -> Vec<usize> {
    let mut available: Vec<usize> = Vec::new();

    for index in 0..workers.len() {
        let worker = workers[index];
        if worker == Status::Idle {
            available.push(index);
        }
    }

    available
}

fn compute_step_time(step: &Step) -> u32 {
    (*step as u32) - b'A' as u32 + 1
}

fn part2(input: &str) -> u32 {
    let dependencies: Result<Vec<Dependency>, _> = input
    .lines()
    .map(|line| {
        parse_ins(line).map(|(_, (char1, char2))| Dependency { step: char2, required: char1 })
    })
    .collect();

    let dependencies = dependencies.unwrap();

    // Build map with all dependencies for each step.
    let mut requirements: Required = HashMap::new();
    dependencies
        .iter()
        .for_each(|dep| {
            requirements.entry(dep.step).or_default().insert(dep.required);
            requirements.entry(dep.required).or_default();
        });
    // Set that contains the assigned steps
    let mut assigned: HashSet<Step> = HashSet::new();

    // Contains all the registered workers
    let mut workers: Vec<Status> = vec![Status::Idle; 2];

    // Hash set with all finished tasks
    let mut done: HashSet<Step> = HashSet::new();

    let mut seconds: u32 = 0;
    loop {
        let done_steps = apply_tick(&mut workers);
        done.extend(done_steps);
        //println!("done => {:?}", &done);
        let mut next_tasks = find_next_step(&requirements, &done, &assigned);
        //println!("next_tasks => {:?}", &next_tasks);
        if next_tasks.is_empty() && all_workers_idle(&workers) {
            break;
        }

        for index in available_workers(&workers) {
            let step = match next_tasks.pop() {
                None => break,
                Some(step) => step,
            };
            workers[index] = Status::Working {
                step,
                remaining: compute_step_time(&step),
            };
            assigned.insert(step);
        }

        seconds += 1;
    }

    seconds
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(15, part2(include_str!("input_lib.txt")));
    }
}