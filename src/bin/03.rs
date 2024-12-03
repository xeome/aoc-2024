use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let sum: i32 = re
        .captures_iter(input)
        .map(|cap| {
            let x: i32 = cap[1].parse().unwrap();
            let y: i32 = cap[2].parse().unwrap();
            x * y
        })
        .sum();
    Some(sum as u32)
}

#[derive(Debug)]
enum Token {
    Multiply(i32, i32),
    Enable,
    Disable,
}

fn parse_input(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let re = Regex::new(r"(mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\))").unwrap();

    for cap in re.captures_iter(input) {
        let token = match &cap[1] {
            s if s.starts_with("mul") => {
                let nums = re.captures(s).unwrap();
                Token::Multiply(nums[2].parse().unwrap(), nums[3].parse().unwrap())
            }
            "do()" => Token::Enable,
            "don't()" => Token::Disable,
            _ => continue,
        };
        tokens.push(token);
    }
    tokens
}

fn calculate_sum(tokens: Vec<Token>) -> i32 {
    let mut sum = 0;
    let mut enabled = true;

    for token in tokens {
        match token {
            Token::Multiply(x, y) if enabled => sum += x * y,
            Token::Enable => enabled = true,
            Token::Disable => enabled = false,
            _ => {}
        }
    }
    sum
}

pub fn part_two(input: &str) -> Option<u32> {
    let tokens = parse_input(input);
    let sum = calculate_sum(tokens);
    Some(sum as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(174561379));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(106921067));
    }
}
