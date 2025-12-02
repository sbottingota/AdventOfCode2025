const INPUT_FILE: &str = "day02.txt";

fn is_invalid(n: u64) -> bool {
    let digits = n.to_string();
    digits.len() % 2 == 0 && digits[..digits.len()/2] == digits[digits.len()/2..]
}

fn main() {
    let mut ranges: Vec<(u64, u64)> = Vec::new();
    for range_str in std::fs::read_to_string(INPUT_FILE)
        .expect("Error reading input file")
        .split(',') {

        let range: Vec<u64> = range_str.split('-')
            .map(|substr| substr.trim().parse().unwrap())
            .collect();
        ranges.push((range[0], range[1]));
    }

    let mut invalid_range_sum = 0_u64;
    for (min, max) in ranges {
        for i in min..=max {
            if is_invalid(i) {
                invalid_range_sum += i;
                // println!("invalid: {}", i);
            }
        }
    }

    println!("{}", invalid_range_sum);
}

