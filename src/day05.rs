use std::ops::Range;

pub fn part1(lines: Vec<String>) -> u32 {
    // Parse wanted seeds
    let seeds: Vec<usize> = lines[0]
        .split(": ")
        .nth(1)
        .unwrap()
        .split_ascii_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Parse mappings (starts at line 2, each map group goes until empty line)
    let mappings = parse_mappings(&lines[2..]);

    let mut result = usize::MAX;
    for seed in seeds {
        let mut val = seed;
        for mapping in mappings.iter() {
            val = convert_via_mapping(val, mapping);
        }
        result = result.min(val);
    }

    result.try_into().unwrap()
}

pub fn part2(lines: Vec<String>) -> u32 {
    let seed_ranges = lines[0]
        .split(": ")
        .nth(1)
        .unwrap()
        .split_ascii_whitespace()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<usize>>()
        .chunks_exact(2)
        .map(|chunk| chunk[0]..(chunk[0] + chunk[1]))
        .collect::<Vec<Range<usize>>>();

    let mappings = parse_mappings(&lines[2..]);

    println!("{:?}", mappings[0]);
    let mut temp = Vec::new();
    let mut ranges = seed_ranges;
    for mapping in mappings {
        for r in ranges {
            temp.append(convert_range(r, &mapping).as_mut());
        }
        ranges = temp.clone();
        temp = Vec::new();
    }

    ranges
        .iter()
        .map(|r| r.start.try_into().unwrap())
        .min()
        .unwrap()
    // for r in seed_ranges {
    //     for seed in r {
    //         let mut val = seed;
    //         for mapping in mappings.iter() {
    //             val = convert_via_mapping(val, mapping);
    //         }
    //         result = result.min(val);
    //     }
    // }
    // result.try_into().unwrap()
}

fn parse_mappings(lines: &[String]) -> Vec<Vec<(usize, usize, usize)>> {
    let mut mappings = Vec::new();
    let mut current_mapping = Vec::new();
    let mut line_idx = 0;
    while let Some(line) = lines.get(line_idx) {
        line_idx += 1;
        if line.is_empty() {
            // TODO go to new map group
            mappings.push(current_mapping);
            current_mapping = Vec::new();
            continue;
        } else if line.contains(':') {
            // Header line, skip
            continue;
        }
        let mapping: Vec<usize> = line
            .split_ascii_whitespace()
            .map(|s| s.parse().unwrap())
            .take(3)
            .collect();
        current_mapping.push((mapping[0], mapping[1], mapping[2]));
    }
    // Add last mapping
    mappings.push(current_mapping);
    mappings
}

fn convert_via_mapping(original: usize, mapping: &[(usize, usize, usize)]) -> usize {
    for (dst, src, length) in mapping {
        if original >= *src && original < (src + length) {
            let delta = original - src;
            return dst + delta;
        }
    }
    // No matching mapping, return same
    original
}

fn convert_range(original: Range<usize>, mapping: &[(usize, usize, usize)]) -> Vec<Range<usize>> {
    let mut ranges = Vec::new();
    let start = original.start;
    let end = original.end;

    // sort by 2nd element (src)
    let mut mapping = mapping.to_vec();
    mapping.sort_by(|a, b| a.1.cmp(&b.1));

    let mut last_idx = None;

    for (dst, src, length) in mapping {
        if src >= end {
            // Mapping is outside range (and all future mappings)
            break;
        } else if (src + length) < start {
            // Mapping fully before range, skip
            continue;
        }

        // Add gaps between mappings
        if let Some(val) = last_idx {
            if val < src {
                ranges.push(val..src);
            }
        } else {
            // First mapping, add range from start to src
            if src > start {
                ranges.push(start..src);
            }
        }

        if src < start && original.contains(&(src + length)) {
            // Case 1: map starts outside range, ends inside
            let offset = start - src;
            ranges.push((dst + offset)..(dst + length));
            last_idx = Some(src + length);
        } else if original.contains(&src) && original.contains(&(src + length)) {
            // Case 2: map is fully inside range
            ranges.push(dst..(dst + length));
            last_idx = Some(src + length);
        } else if original.contains(&src) && (src + length) >= end {
            // Case 3: map starts inside range, ends outside
            let inside_length = end - src;
            ranges.push(dst..(dst + inside_length));
            last_idx = Some(src + inside_length);
        } else if src < start && (src + length) >= end {
            // Case 4: map includes full range
            let delta = start - src;
            let l = end - start;
            ranges.push((dst + delta)..(dst + delta + l));
            last_idx = Some(end);
        } else {
            panic!("Unhandled case: {:?} -> {:?}", original, (dst, src, length));
        }
    }

    // Add last gap if needed
    if let Some(val) = last_idx {
        if val < end {
            ranges.push(val..end);
        }
    } else {
        // No used mappings, no change
        ranges.push(original);
    }

    ranges
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let data = [
            "seeds: 79 14 55 13",
            "",
            "seed-to-soil map:",
            "50 98 2",
            "52 50 48",
            "",
            "soil-to-fertilizer map:",
            "0 15 37",
            "37 52 2",
            "39 0 15",
            "",
            "fertilizer-to-water map:",
            "49 53 8",
            "0 11 42",
            "42 0 7",
            "57 7 4",
            "",
            "water-to-light map:",
            "88 18 7",
            "18 25 70",
            "",
            "light-to-temperature map:",
            "45 77 23",
            "81 45 19",
            "68 64 13",
            "",
            "temperature-to-humidity map:",
            "0 69 1",
            "1 0 69",
            "",
            "humidity-to-location map:",
            "60 56 37",
            "56 93 4",
        ]
        .map(String::from)
        .to_vec();

        assert_eq!(part1(data), 35);
    }

    #[test]
    fn test_part2() {
        let data = [
            "seeds: 79 14 55 13",
            "",
            "seed-to-soil map:",
            "50 98 2",
            "52 50 48",
            "",
            "soil-to-fertilizer map:",
            "0 15 37",
            "37 52 2",
            "39 0 15",
            "",
            "fertilizer-to-water map:",
            "49 53 8",
            "0 11 42",
            "42 0 7",
            "57 7 4",
            "",
            "water-to-light map:",
            "88 18 7",
            "18 25 70",
            "",
            "light-to-temperature map:",
            "45 77 23",
            "81 45 19",
            "68 64 13",
            "",
            "temperature-to-humidity map:",
            "0 69 1",
            "1 0 69",
            "",
            "humidity-to-location map:",
            "60 56 37",
            "56 93 4",
        ]
        .map(String::from)
        .to_vec();

        assert_eq!(part2(data), 46);
    }

    #[test]
    fn test_testing() {
        // dst, src, length
        // 0..69 -> 1..70, 69..70 -> 0..1
        // let mut mappings = vec![(1, 0, 69), (0, 69, 1)];
        let mut mappings = vec![(50, 98, 2), (52, 50, 48)];
        mappings.sort_by(|a, b| a.1.cmp(&b.1));
        println!("{:?}", mappings);

        // let original = 68..74;
        // 68, 69, 70, 71, 72, 73
        // 69,  0, 70, 71, 72, 73
        let original = 79..93;
        let new_ranges = convert_range(original, &mappings);
        println!("{:?}", new_ranges);
        // assert_eq!(0, 1);
    }
}

