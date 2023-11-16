fn part1(input: &str) -> u32 {
    let secret_key = input;
    let mut answer = 0;
    loop {
        answer += 1;
        let concatenated_string = format!("{}{}", secret_key, answer);

        let md5 = md5::compute(concatenated_string);
    
        let hex_representation: String = md5
            .iter()
            .map(|byte| format!("{:02x}", byte))
            .collect();

            if hex_representation.starts_with("00000") {
                return answer;
            }
        if answer > 10_000_000 {break;}
    }
    answer
}
fn part2(input: &str) -> u32 {
    let secret_key = input;
    let mut answer = 0;
    loop {
        answer += 1;
        let concatenated_string = format!("{}{}", secret_key, answer);

        let md5 = md5::compute(concatenated_string);
    
        let hex_representation: String = md5
            .iter()
            .map(|byte| format!("{:02x}", byte))
            .collect();

            if hex_representation.starts_with("000000") {
                return answer;
            }
        if answer > 10_000_000 {break;}
    }
    answer
}

fn main() {
    let input = "ckczppom";
    println!("{}, {}", part1(input), part2(input));
}