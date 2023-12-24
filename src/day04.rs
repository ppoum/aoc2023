pub fn part1(lines: Vec<String>) -> u32 {
    let mut total = 0;
    for line in lines.iter().map(|s| s.split(':').nth(1).unwrap().trim()) {
        let numbers = parse_card(line);
        let n = card_match_count(&numbers[0], &numbers[1]);
        if n > 0 {
            total += 2_u32.pow(n - 1);
        }
    }

    total
}

pub fn part2(lines: Vec<String>) -> u32 {
    let mut total = 0;
    let mut count = vec![0_isize; lines.len() + 1];
    let mut current_count = 1;
    for (i, line) in lines
        .iter()
        .map(|s| s.split(':').nth(1).unwrap().trim())
        .enumerate()
    {
        current_count += count[i];
        total += current_count;
        let numbers = parse_card(line);
        let n = card_match_count(&numbers[0], &numbers[1]);
        if n > 0 {
            // Mark cards i+1 to i+n as having c more copies, where c is the # of copies for the
            // current card.
            count[i + 1] += current_count;
            count[i + n as usize + 1] -= current_count;
        }
    }

    total.try_into().unwrap()
}

fn parse_card(s: &str) -> Vec<Vec<u32>> {
    let mut numbers: Vec<Vec<u32>> = s
        .split(" | ")
        .map(|s| {
            s.split_ascii_whitespace()
                .map(|ss| ss.parse::<u32>().unwrap())
                .collect()
        })
        .collect();
    numbers[0].sort();
    numbers
}

/// Assumes `winning_numbers` is sorted.
fn card_match_count(winning_numbers: &[u32], card_numbers: &[u32]) -> u32 {
    let n = card_numbers
        .iter()
        .filter(|&n| winning_numbers.binary_search(n).is_ok())
        .count()
        .try_into()
        .unwrap();
    n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let data = [
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        ]
        .map(String::from)
        .to_vec();

        assert_eq!(part1(data), 13);
    }

    #[test]
    fn test_part2() {
        let data = [
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        ]
        .map(String::from)
        .to_vec();

        assert_eq!(part2(data), 30);
    }
}

