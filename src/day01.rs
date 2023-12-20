pub fn part1(lines: Vec<String>) -> u32 {
    let mut total = 0;
    for line in lines {
        let first_digit = line
            .chars()
            .find(|c| c.is_ascii_digit())
            .expect("No digits");
        let last_digit = line.chars().rev().find(|c| c.is_ascii_digit()).unwrap();
        total += format!("{}{}", first_digit, last_digit)
            .parse::<u32>()
            .unwrap();
    }
    total
}

pub fn part2(lines: Vec<String>) -> u32 {
    let digits_str = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let mut total = 0;

    for line in lines {
        // Find letter digits
        let mut min = usize::MAX;
        let mut min_digit = 0;
        let mut max = 0;
        let mut max_digit = 0;
        for (i, digit_str) in digits_str.iter().enumerate() {
            if let Some(x) = line.find(digit_str) {
                if x < min {
                    min = x;
                    min_digit = i;
                }
            }
            if let Some(x) = line.rfind(digit_str) {
                if (x + digit_str.len() - 1) > max {
                    max = x + digit_str.len() - 1;
                    max_digit = i;
                }
            }
        }

        // Find number digits
        if let Some(n) = line.chars().take(min).find(|c| c.is_ascii_digit()) {
            min_digit = n.to_digit(10).unwrap() as usize;
        }

        // Only check characters from the end that would yield a better result
        if let Some(n) = line
            .chars()
            .rev()
            .take(line.len() - max)
            .find(|c| c.is_ascii_digit())
        {
            max_digit = n.to_digit(10).unwrap() as usize;
            println!("{} - {} {}", line, min_digit, max_digit);
        }
        total += min_digit * 10 + max_digit;
    }
    total.try_into().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let data = ["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"]
            .map(String::from)
            .to_vec();

        assert_eq!(part1(data), 142);
    }

    #[test]
    fn test_part2() {
        let data = [
            "two1nine",
            "eightwothree",
            "abcone2threexyz",
            "xtwone3four",
            "4nineeightseven2",
            "zoneight234",
            "7pqrstsixteen",
        ]
        .map(String::from)
        .to_vec();

        assert_eq!(part2(data), 281);
    }
}

