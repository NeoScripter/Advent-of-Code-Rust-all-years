pub fn process_output(input: &str) -> u64 {
    let mut v = Vec::new();
    
    for block in input.split("$ cd ") {
        let mut sum: u64 = 0;
        
        if block.trim() == ".." {
            continue;
        }
        
        for line in block.lines() {
            sum += line
                .split_whitespace()
                .filter_map(|word| word.parse::<u64>().ok())
                .sum::<u64>();
        }
        
        if sum > 0 {
            v.push(sum);
        }
    }
    
    let result: u64 = v.iter().filter(|num| *num <= &100000).sum();
    result
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_output() {
        let input = include_str!("input_lib.txt");
        assert_eq!(94853, process_output(input));
    }
}