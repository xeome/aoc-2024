use std::collections::HashMap;

advent_of_code::solution!(1);

fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    let capacity = input.len() / 8;
    let mut left = Vec::with_capacity(capacity);
    let mut right = Vec::with_capacity(capacity);

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        let mut iter = line.split_whitespace();
        if let (Some(l), Some(r)) = (iter.next(), iter.next()) {
            if let (Ok(left_num), Ok(right_num)) = (l.parse::<i32>(), r.parse::<i32>()) {
                left.push(left_num);
                right.push(right_num);
            }
        }
    }
    (left, right)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut left, mut right) = parse_input(input);
    left.sort_unstable();
    right.sort_unstable();

    let mut total_dist = 0i32;
    for i in 0..left.len() {
        total_dist += (left[i] - right[i]).abs();
    }
    Some(total_dist as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (left, right) = parse_input(input);
    let mut right_freq_map = HashMap::with_capacity(right.len());

    for &x in right.iter() {
        *right_freq_map.entry(x).or_insert(0) += 1;
    }

    let mut similarity = 0;
    for &x in left.iter() {
        similarity += x * right_freq_map.get(&x).unwrap_or(&0);
    }
    Some(similarity as u32)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1882714));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(19437052));
    }
}
