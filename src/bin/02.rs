advent_of_code::solution!(2);

fn is_safe(numbers: &[i32]) -> bool {
    if numbers.len() < 2 {
        return true;
    }
    let diffs: Vec<_> = numbers.windows(2).map(|w| w[1] - w[0]).collect();
    let all_positive = diffs.iter().all(|&d| (1..=3).contains(&d));
    let all_negative = diffs.iter().all(|&d| (-3..0).contains(&d));
    all_positive || all_negative
}

fn is_safe2(numbers: &[i32]) -> bool {
    if is_safe(numbers) {
        return true;
    }

    (0..numbers.len()).any(|i| {
        let mut modified = numbers.to_vec();
        modified.remove(i);
        is_safe(&modified)
    })
}

pub fn part_one(input: &str) -> Option<u32> {
    let safe_count = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|n| n.parse::<i32>().ok())
                .collect::<Vec<_>>()
        })
        .filter(|nums| is_safe(nums))
        .count();
    Some(safe_count as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let safe_count = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|n| n.parse::<i32>().ok())
                .collect::<Vec<_>>()
        })
        .filter(|nums| is_safe2(nums))
        .count();
    Some(safe_count as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(306));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(366));
    }
}
