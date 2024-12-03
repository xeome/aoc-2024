use lazy_static::lazy_static;
use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    }

    let sum: i32 = RE
        .captures_iter(input)
        .map(|cap| {
            let x: i32 = cap[1].parse().unwrap();
            let y: i32 = cap[2].parse().unwrap();
            x * y
        })
        .sum();
    Some(sum as u32)
}

#[derive(Debug, Clone, Copy)]
enum Token {
    Multiply(i32, i32),
    Enable,
    Disable,
}

fn parse_input(input: &str) -> Vec<Token> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap();
    }

    let estimated_size = input.len() / 10;
    let mut tokens = Vec::with_capacity(estimated_size);

    RE.find_iter(input).for_each(|m| {
        let s = m.as_str();
        match s {
            s if s.starts_with("mul") => {
                let mut nums = s[4..s.len() - 1].split(',');
                if let (Some(x), Some(y)) = (nums.next(), nums.next()) {
                    if let (Ok(x), Ok(y)) = (x.parse::<i32>(), y.parse::<i32>()) {
                        tokens.push(Token::Multiply(x, y));
                    }
                }
            }
            "do()" => tokens.push(Token::Enable),
            "don't()" => tokens.push(Token::Disable),
            _ => {}
        }
    });

    tokens
}

fn calculate_sum(tokens: &[Token]) -> i32 {
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
    let sum = calculate_sum(&tokens);
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
