use std::collections::HashMap;

pub fn part1(lines: Vec<String>) -> u32 {
    let mut total = 0;
    for row in 0..lines.len() {
        // Find indexes that are numbers
        let numbers_iter = lines[row]
            .chars()
            .enumerate()
            .filter(|(_, c)| c.is_ascii_digit());
        let mut start = None;
        let mut end = 0;
        let mut number = 0;
        for (i, c) in numbers_iter {
            if start.is_none() {
                start = Some(i);
                end = i;
                number = c.to_digit(10).unwrap();
                continue;
            }
            if i == end + 1 {
                // Same number continuing
                end = i;
                number *= 10;
                number += c.to_digit(10).unwrap();
                continue;
            }

            // Number ended, check if next to symbol
            if neighboring_symbol(&lines, row, start.unwrap(), end) {
                total += number;
            }
            start = Some(i);
            end = i;
            number = c.to_digit(10).unwrap();
        }
        // Last number ended (if any)
        if let Some(start) = start {
            if neighboring_symbol(&lines, row, start, end) {
                total += number;
            }
        }
    }
    total
}

pub fn part2(lines: Vec<String>) -> u32 {
    let mut gears = HashMap::new();
    for row in 0..lines.len() {
        // Find indexes that are numbers
        let numbers_iter = lines[row]
            .chars()
            .enumerate()
            .filter(|(_, c)| c.is_ascii_digit());
        let mut start = None;
        let mut end = 0;
        let mut number = 0;
        for (i, c) in numbers_iter {
            if start.is_none() {
                start = Some(i);
                end = i;
                number = c.to_digit(10).unwrap();
                continue;
            }
            if i == end + 1 {
                // Same number continuing
                end = i;
                number *= 10;
                number += c.to_digit(10).unwrap();
                continue;
            }

            // Number ended, add to neighboring gear if any
            mark_gears(&lines, &mut gears, row, start.unwrap(), end, number);
            start = Some(i);
            end = i;
            number = c.to_digit(10).unwrap();
        }
        // Last number ended (if any)
        if let Some(start) = start {
            mark_gears(&lines, &mut gears, row, start, end, number);
        }
    }

    gears
        .values()
        .filter(|v| v.len() == 2)
        .map(|v| v[0] * v[1])
        .sum()
}

fn neighboring_symbol(lines: &[String], row: usize, start: usize, end: usize) -> bool {
    let row_length = lines[0].len();
    let diag_start = if start > 0 { start - 1 } else { 0 };
    let diag_end = std::cmp::min(end + 1, row_length - 1);

    // Check above
    if row > 0
        && lines[row - 1][diag_start..diag_end + 1]
            .chars()
            .any(|c| !c.is_ascii_digit() && c != '.')
    {
        return true;
    }

    // Check below
    if row < lines.len() - 1
        && lines[row + 1][diag_start..diag_end + 1]
            .chars()
            .any(|c| !c.is_ascii_digit() && c != '.')
    {
        return true;
    }

    // Check left and right
    let left_char = lines[row].chars().nth(diag_start).unwrap();
    let right_char = lines[row].chars().nth(diag_end).unwrap();
    if (!left_char.is_ascii_digit() && left_char != '.')
        || (!right_char.is_ascii_digit() && right_char != '.')
    {
        return true;
    }

    false
}

fn mark_gears(
    lines: &[String],
    gears: &mut HashMap<(usize, usize), Vec<u32>>,
    row: usize,
    start: usize,
    end: usize,
    number: u32,
) {
    let row_length = lines[0].len();
    let diag_start = if start > 0 { start - 1 } else { 0 };
    let diag_end = std::cmp::min(end + 1, row_length - 1);
    let mut gear_positions = Vec::new();

    // Check above
    if row > 0 {
        for (i, _) in lines[row - 1][diag_start..diag_end + 1]
            .chars()
            .enumerate()
            .filter(|(_, c)| *c == '*')
        {
            gear_positions.push((row - 1, diag_start + i));
        }
    }

    // Check below
    if row < lines.len() - 1 {
        for (i, _) in lines[row + 1][diag_start..diag_end + 1]
            .chars()
            .enumerate()
            .filter(|(_, c)| *c == '*')
        {
            gear_positions.push((row + 1, diag_start + i));
        }
    }

    // Check left and right
    if lines[row].chars().nth(diag_start).unwrap() == '*' {
        gear_positions.push((row, diag_start));
    }
    if lines[row].chars().nth(diag_end).unwrap() == '*' {
        gear_positions.push((row, diag_end));
    }

    for pos in gear_positions {
        gears
            .entry(pos)
            .and_modify(|v| v.push(number))
            .or_insert(vec![number]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let data = [
            "467..114..",
            "...*......",
            "..35..633.",
            "......#...",
            "617*......",
            ".....+.58.",
            "..592.....",
            "......755.",
            "...$.*....",
            ".664.598..",
        ]
        .map(String::from)
        .to_vec();

        assert_eq!(part1(data), 4361);
    }

    #[test]
    fn test_part2() {
        let data = [
            "467..114..",
            "...*......",
            "..35..633.",
            "......#...",
            "617*......",
            ".....+.58.",
            "..592.....",
            "......755.",
            "...$.*....",
            ".664.598..",
        ]
        .map(String::from)
        .to_vec();

        assert_eq!(part2(data), 467835);
    }
}

