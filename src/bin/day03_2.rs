const INPUT_FILE: &str = "day03.txt";

fn max_idx(list: &[u64]) -> usize {
    let mut max_elem = 0_u64;
    let mut max_idx = 0_usize;

    for (i, n) in list.iter().enumerate() {
        if *n > max_elem {
            max_idx = i;
            max_elem = *n;
        }
    }

    max_idx
}

fn get_highest_joltage(battery: &[u64]) -> u64 {
    const N_ELEMENTS: usize = 12;
    
    let mut highest_joltage = 0;
    let mut left = 0_usize;

    for i in 0..N_ELEMENTS {
        let right = battery.len() - N_ELEMENTS + i;
        let digit_idx = left + max_idx(&battery[left..=right]);

        highest_joltage *= 10;
        highest_joltage += battery[digit_idx];

        left = digit_idx + 1;
    }

    // println!("{}", highest_joltage);
    highest_joltage
}

fn main() {
    let mut batteries: Vec<Vec<u64>> = Vec::new();
    for line in std::fs::read_to_string(INPUT_FILE)
        .expect("Error reading input file")
        .lines() {

        batteries.push(line.trim().chars().map(|c| c.to_digit(10).unwrap() as u64).collect());
    }

    let total_joltage: u64 = batteries
        .iter()
        .map(|battery| get_highest_joltage(battery))
        .sum();

    println!("{}", total_joltage);
}

