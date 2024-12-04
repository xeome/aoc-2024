advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Vec<&[u8]> = input.lines().map(|line| line.as_bytes()).collect();
    let rows = grid.len() as isize;
    let cols = grid.first()?.len() as isize;

    let word = b"XMAS";
    let word_len = word.len() as isize;

    // All 8 possible directions (dx, dy)
    let directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let mut count = 0;

    for y in 0..rows {
        for x in 0..cols {
            // x is the column, y is the row index
            for &(dx, dy) in &directions {
                // dx is the column direction, dy is the row direction
                let mut matched = true;
                for i in 0..word_len {
                    let nx = x + dx * i; // New x
                    let ny = y + dy * i; // New y
                    if nx < 0 || ny < 0 || nx >= cols || ny >= rows {
                        // Out of bounds
                        matched = false;
                        break;
                    }
                    if grid[ny as usize][nx as usize] != word[i as usize] {
                        matched = false;
                        break;
                    }
                }
                if matched {
                    count += 1;
                }
            }
        }
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid: Vec<&[u8]> = input.lines().map(|line| line.as_bytes()).collect();
    let rows = grid.len();
    let cols = grid.first()?.len();

    let mut count = 0;

    // Iterate through the grid, avoiding the borders to prevent out-of-bounds
    for y in 1..rows - 1 {
        for x in 1..cols - 1 {
            if grid[y][x] != b'A' {
                continue;
            }

            // Check Diagonal 1 (Top-Left to Bottom-Right)
            let diag1 = match (
                grid.get(y.wrapping_sub(1))
                    .and_then(|row| row.get(x.wrapping_sub(1))),
                grid[y][x],
                grid.get(y + 1).and_then(|row| row.get(x + 1)),
            ) {
                (Some(&c1), b'A', Some(&c3)) => {
                    (c1 == b'M' && c3 == b'S') || (c1 == b'S' && c3 == b'M')
                }
                _ => false,
            };

            // Check Diagonal 2 (Top-Right to Bottom-Left)
            let diag2 = match (
                grid.get(y.wrapping_sub(1)).and_then(|row| row.get(x + 1)),
                grid[y][x],
                grid.get(y + 1).and_then(|row| row.get(x.wrapping_sub(1))),
            ) {
                (Some(&c2), b'A', Some(&c4)) => {
                    (c2 == b'M' && c4 == b'S') || (c2 == b'S' && c4 == b'M')
                }
                _ => false,
            };

            if diag1 && diag2 {
                count += 1;
            }
        }
    }

    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2532));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1941));
    }
}
