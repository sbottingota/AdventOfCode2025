use std::ops::RangeInclusive;
use std::collections::HashSet;

const INPUT_FILE: &str = "day05.txt";

fn main() {
    let mut ranges: HashSet<RangeInclusive<u64>> = HashSet::new();

    for line in std::fs::read_to_string(INPUT_FILE)
        .expect("Error reading from input file")
        .lines() {
        if line.trim().is_empty() {
            break;
        }

        let parts: Vec<u64> = line
            .split('-')
            .map(|substr| substr.parse().unwrap())
            .collect();
        ranges.insert(parts[0]..=parts[1]);
    }


    let mut change_made = true;
    while change_made {
        change_made = false;

        let mut new_ranges: HashSet<RangeInclusive<u64>> = HashSet::new();
        for range1 in &ranges {
            for range2 in &ranges {
                if range1 == range2 { continue; }

                if range1.contains(range2.start()) {
                    change_made = true;
                    if range1.contains(range2.end()) {
                        new_ranges.insert(range1.clone());
                    } else {
                        new_ranges.insert(*range1.start()..=*range2.end());
                    }
                } else if range1.contains(range2.end()) {
                    change_made = true;
                    new_ranges.insert(*range2.start()..=*range1.end());
                }
            }
        }

        // add unmodified ranges
        for range1 in &ranges {
            if !ranges.iter().any(|range2| range1 != range2 && (range2.contains(range1.start()) || range2.contains(range1.end()))) {
                new_ranges.insert(range1.clone());
            }
        }

        ranges = new_ranges;
    }

    let fresh_count: u64 = ranges
        .into_iter()
        .map(|range| range.end() - range.start() + 1)
        .sum();

    println!("{}", fresh_count);
}

