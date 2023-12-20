pub mod day01;
pub mod day02;

pub fn parse_file(path: &str) -> Vec<String> {
    // Read file, return line iterator
    std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

