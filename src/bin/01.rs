use std::collections::HashMap;

advent_of_code::solution!(1);
fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() != 2 {
            eprintln!("Invalid input line: {}", line);
            continue;
        }

        let left_num: i32 = parts[0].parse().unwrap();
        let right_num: i32 = parts[1].parse().unwrap();
        left.push(left_num);
        right.push(right_num);
    }
    (left, right)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut left, mut right) = parse_input(input);

    left.sort_unstable();
    right.sort_unstable();
    let total_dist: i32 = left
        .iter()
        .zip(right.iter())
        .map(|(l, r)| (l - r).abs())
        .sum();

    Some(total_dist as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (left, right) = parse_input(input);

    let right_freq_map: HashMap<i32, i32> = right.iter().fold(HashMap::new(), |mut acc, &x| {
        *acc.entry(x).or_insert(0) += 1;
        acc
    });

    let similarity: i32 = left
        .iter()
        .map(|&x| x * right_freq_map.get(&x).unwrap_or(&0))
        .sum();
    Some(similarity as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
