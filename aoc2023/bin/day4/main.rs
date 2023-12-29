fn part1(input: &str) -> u32 {
    input.lines().map(|line| {
        let (_, rest) = line.split_once(": ").unwrap();
        let (win_num, your_num) = rest.split_once(" | ").unwrap();
        let win_num: Vec<u32> = win_num.split_whitespace().map(|num| num.parse::<u32>().unwrap()).collect();
        let your_num: Vec<u32> = your_num.split_whitespace().map(|num| num.parse::<u32>().unwrap()).collect();
        let len = your_num.into_iter().filter(|n| win_num.contains(&n)).count();
        (1 << len) / 2
    }).sum::<u32>()
}
fn part2(input: &str) -> usize {
    let values: Vec<usize> = input.lines().map(|line| {
        let (_, rest) = line.split_once(": ").unwrap();
        let (win_num, your_num) = rest.split_once(" | ").unwrap();
        let win_num: Vec<usize> = win_num.split_whitespace().map(|num| num.parse::<usize>().unwrap()).collect();
        let your_num: Vec<usize> = your_num.split_whitespace().map(|num| num.parse::<usize>().unwrap()).collect();
        your_num.into_iter().filter(|n| win_num.contains(&n)).count()
    }).collect();
    let answer = total_scratchcards(values);
    answer
}

fn total_scratchcards(matches: Vec<usize>) -> usize {
    let mut card_counts = vec![1; matches.len()];

    for (i, &match_count) in matches.iter().enumerate() {
        for _ in 0..card_counts[i] {
            let end = usize::min(i + match_count + 1, matches.len());
            for j in i + 1..end {
                card_counts[j] += 1;
            }
        }
    }
    card_counts.iter().sum()
}

fn main() {
    let input = include_str!("input4.txt");
    println!("{}, {}", part1(input), part2(input));
}