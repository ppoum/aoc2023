pub fn part1(lines: Vec<String>) -> u32 {
    const MAX_RED: u32 = 12;
    const MAX_GREEN: u32 = 13;
    const MAX_BLUY: u32 = 14;

    let mut total = 0;
    // Line = Game n: n color, n color; n color...
    'line_loop: for (i, line) in lines.iter().enumerate() {
        let sets = line.split(':').nth(1).unwrap();
        // Sets = n color, n color; n color...
        for set in sets.split(';').map(|s| s.trim()) {
            // Split set in items
            let items = set
                .split(", ")
                .map(|s| s.split_whitespace().collect::<Vec<&str>>());
            for item in items {
                // item = [n, color]
                // Check if certain color has more than the max
                match item[1] {
                    "red" => {
                        if item[0].parse::<u32>().unwrap() > MAX_RED {
                            continue 'line_loop;
                        }
                    }
                    "green" => {
                        if item[0].parse::<u32>().unwrap() > MAX_GREEN {
                            continue 'line_loop;
                        }
                    }
                    "blue" => {
                        if item[0].parse::<u32>().unwrap() > MAX_BLUY {
                            continue 'line_loop;
                        }
                    }
                    _ => panic!("Invalid color"),
                }
            }
        }
        total += i + 1;
    }
    total.try_into().unwrap()
}

pub fn part2(lines: Vec<String>) -> u32 {
    let mut total = 0;
    for line in lines {
        let (mut red, mut green, mut blue) = (0, 0, 0);
        let sets = line.split(':').nth(1).unwrap();
        for set in sets.split(';').map(|s| s.trim()) {
            let items = set
                .split(", ")
                .map(|s| s.split_whitespace().collect::<Vec<&str>>());
            for item in items {
                // item = [n, color]
                // Check if certain color has more than the max
                match item[1] {
                    "red" => red = red.max(item[0].parse::<u32>().unwrap()),
                    "green" => green = green.max(item[0].parse::<u32>().unwrap()),
                    "blue" => blue = blue.max(item[0].parse::<u32>().unwrap()),
                    _ => panic!("Invalid color"),
                }
            }
        }
        total += red * green * blue;
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let data = [
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        ]
        .map(String::from)
        .to_vec();
        assert_eq!(part1(data), 8);
    }

    #[test]
    fn test_part2() {
        let data = [
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        ]
        .map(String::from)
        .to_vec();
        assert_eq!(part2(data), 2286);
    }
}

