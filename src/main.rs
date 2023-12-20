use std::env;

use aoc2023::parse_file;

fn main() {
    let args = env::args().skip(1).collect::<Vec<String>>();

    let challenge = args.get(0).expect("Missing argument");
    let day = challenge[..challenge.len() - 1].parse::<usize>().unwrap();

    let day = if day < 10 {
        format!("0{}", day)
    } else {
        format!("{}", day)
    };

    let input = parse_file(&format!("data/day{}.txt", day));

    match challenge.as_str() {
        "1a" => println!("{}", aoc2023::day01::part1(input)),
        "1b" => println!("{}", aoc2023::day01::part2(input)),
        _ => eprintln!("Invalid day"),
    }
}
