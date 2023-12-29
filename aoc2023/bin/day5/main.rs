use std::collections::HashMap;
use std::vec;

#[derive(Debug, Clone)]
struct Range(i128, i128);

fn _process_almanac(initial_values: Vec<u64>, mut rest: Vec<&str>) -> Vec<u64> {
    let mut start_values = initial_values;
    let range_end = start_values.iter().copied().max().unwrap();
    let mut instruction = HashMap::new();
    if let Some(chunk) = rest.pop() {
        for line in chunk.lines() {
            if line.ends_with("map:") {continue};
            let split_line: Vec<u64> = line.split_whitespace().map(|x| x.parse::<u64>().unwrap()).collect();
            let (dest_range, sourse_range, range_length) = (split_line[0], split_line[1], split_line[2]);
            (sourse_range..(sourse_range + range_length)).into_iter().zip((dest_range..(dest_range + range_length))).for_each(|(sourse, dest)| {
                if start_values.contains(&sourse) {
                    instruction.insert(sourse, dest);
                }
            })
        }
        for num in start_values.clone() {
            if !instruction.contains_key(&num) {
                instruction.insert(num, num);
            }
        }
        start_values = start_values.into_iter()
        .filter_map(|key| instruction.get(&key).cloned())
        .collect::<Vec<u64>>();
        start_values = _process_almanac(start_values, rest);
    }
    start_values
}

fn part2(input: &str) -> String {
    let lines: Vec<_> = input.split("\r\n\r\n").collect();
    let seeds_ranges: Vec<Range> = lines
        .first()
        .unwrap()
        .split(": ")
        .collect::<Vec<_>>()
        .iter()
        .last()
        .unwrap()
        .split(' ')
        .collect::<Vec<_>>()
        .iter()
        .map(|f| f.parse::<i128>().unwrap())
        .collect::<Vec<_>>()
        .chunks(2)
        .map(|f| Range(f[0], f[0] + f[1]))
        .collect::<Vec<_>>();

    let translations = lines
        .iter()
        .skip(1)
        .map(|translation| {
            let mut a = translation
                .split(":\r\n")
                .skip(1)
                .collect::<Vec<_>>()
                .iter()
                .flat_map(|f| f.split("\r\n").collect::<Vec<_>>())
                .map(|x| {
                    x.split(' ')
                        .collect::<Vec<_>>()
                        .iter()
                        .map(|f| f.parse::<i128>().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();
            a.sort_by(|a, b| a[1].cmp(&b[1]));
            a
        })
        .collect::<Vec<_>>();

    let mut ranges = seeds_ranges.clone();
    for stage in translations.iter() {
        let mut new_ranges: Vec<Range> = vec![];

        for range in ranges.iter() {
            let mut curr = range.clone();
            for translation in stage.iter() {
                let offset = translation[0] - translation[1];
                if curr.0 <= curr.1
                    && curr.0 < translation[1] + translation[2]
                    && translation[1] <= curr.1
                {
                    if curr.0 < translation[1] {
                        new_ranges.push(Range(curr.0, translation[1] - 1));
                        curr.0 = translation[1];
                        if curr.1 < translation[1] + translation[2] {
                            new_ranges.push(Range(curr.0 + offset, curr.1 + offset));
                            curr.0 = curr.1 + 1;
                        } else {
                            new_ranges.push(Range(
                                curr.0 + offset,
                                translation[1] + translation[2] - 1 + offset,
                            ));
                            curr.0 = translation[1] + translation[2];
                        }
                    } else if curr.1 < translation[1] + translation[2] {
                        new_ranges.push(Range(curr.0 + offset, curr.1 + offset));
                        curr.0 = curr.1 + 1;
                    } else {
                        new_ranges.push(Range(
                            curr.0 + offset,
                            translation[1] + translation[2] - 1 + offset,
                        ));
                        curr.0 = translation[1] + translation[2];
                    }
                }
            }
            if curr.0 <= curr.1 {
                new_ranges.push(curr);
            }
        }
        ranges = new_ranges;
    }

    ranges.sort_by(|a, b| a.0.cmp(&b.0));

    ranges.first().unwrap().0.to_string()
}
fn main() {
    let input = include_str!("input5.txt");
    println!("{}", part2(input));
}