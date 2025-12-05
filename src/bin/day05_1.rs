use std::ops::RangeInclusive;

const INPUT_FILE: &str = "day05.txt";

fn main() {
    let mut ranges: Vec<RangeInclusive<u64>> = Vec::new();
    let mut ingredients: Vec<u64> = Vec::new();

    let mut is_reading_ranges = true;
    for line in std::fs::read_to_string(INPUT_FILE)
        .expect("Error reading from input file")
        .lines() {
        if line.trim().is_empty() {
            is_reading_ranges = false;
            continue;
        }

        if is_reading_ranges {
            let parts: Vec<u64> = line
                .split('-')
                .map(|substr| substr.parse().unwrap())
                .collect();
            ranges.push(parts[0]..=parts[1]);
        } else {
            ingredients.push(line.trim().parse().unwrap());
        }
    }

    let mut fresh_count = 0_u32;

    for ingredient in ingredients {
        for range in &ranges {
            if range.contains(&ingredient) {
                fresh_count += 1;
                break;
            }
        }
    }

    println!("{}", fresh_count);
}

