use std::cmp::Ordering;

// Speed(x) = x mm / ms
// Distance(x, t) = speed * time = Speed(x) * (t - x), t = total time, x = time increasing speed
// Distance(x,t) = xt - x^2 = -1*x^2 + t*x + 0
// Find all values of x where d(x, t) > best

pub fn part1(lines: Vec<String>) -> u32 {
    let data: Vec<Vec<u32>> = lines
        .iter()
        .map(|line| {
            line.split(':')
                .nth(1)
                .unwrap()
                .split_ascii_whitespace()
                .map(|s| s.parse().unwrap())
                .collect()
        })
        .collect();

    let mut total = 1;
    for (t, d) in std::iter::zip(data[0].iter(), data[1].iter()) {
        total *= hold_option_count(*t as i64, *d as i64);
    }

    total
}

pub fn part2(lines: Vec<String>) -> u32 {
    // Need to parse as u64 to avoid overflow
    let data: Vec<u64> = lines
        .iter()
        .map(|line| {
            line.split(':')
                .nth(1)
                .unwrap()
                .replace(' ', "")
                .parse()
                .unwrap()
        })
        .collect();
    hold_option_count(data[0] as i64, data[1] as i64)
}

fn hold_option_count(time: i64, distance: i64) -> u32 {
    // valid --> -x^2 + tx - d > 0
    let discriminant = time * time - 4 * (distance + 1);
    let discriminant = discriminant as f64;
    match discriminant.partial_cmp(&0.).unwrap() {
        // Should never happen, would mean 0 velocity is a valid solution
        Ordering::Less => panic!("Unimplemented imaginary roots"),
        // Could happen (only 1 possible solution), but didn't in my input
        Ordering::Equal => panic!("unimplemented one root"),
        Ordering::Greater => {
            let r1 = (-time as f64 + discriminant.sqrt()) / -2.;
            let r1 = r1.ceil() as u32;
            let r2 = (-time as f64 - discriminant.sqrt()) / -2.;
            let r2 = r2.floor() as u32;
            1 + r2 - r1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let data = ["Time:      7  15   30", "Distance:  9  40  200"]
            .map(String::from)
            .to_vec();
        assert_eq!(part1(data), 288);
    }

    #[test]
    fn test_part2() {
        let data = ["Time:      7  15   30", "Distance:  9  40  200"]
            .map(String::from)
            .to_vec();
        assert_eq!(part2(data), 71503);
    }
}

