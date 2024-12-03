advent_of_code::solution!(2);

fn is_safe(numbers: &[i32]) -> bool {
    numbers.len() < 2 || {
        let mut diffs = numbers.windows(2).map(|w| w[1] - w[0]);
        let first_diff = diffs.next().unwrap();
        if (1..=3).contains(&first_diff) {
            diffs.all(|d| (1..=3).contains(&d))
        } else if (-3..=-1).contains(&first_diff) {
            diffs.all(|d| (-3..=-1).contains(&d))
        } else {
            false
        }
    }
}

#[inline]
fn is_safe2(numbers: &[i32]) -> bool {
    if is_safe(numbers) {
        return true;
    }

    let mut modified = Vec::with_capacity(numbers.len());

    for i in 0..numbers.len() {
        modified.clear();
        modified.extend(numbers[..i].iter());
        modified.extend(numbers[i + 1..].iter());

        if is_safe(&modified) {
            return true;
        }
    }
    false
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .filter_map(|n| n.parse::<i32>().ok())
                    .collect::<Vec<_>>()
            })
            .filter(|nums| is_safe(nums))
            .count() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .filter_map(|n| n.parse::<i32>().ok())
                    .collect::<Vec<_>>()
            })
            .filter(|nums| is_safe2(nums))
            .count() as u32,
    )
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
