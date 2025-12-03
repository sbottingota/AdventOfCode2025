const INPUT_FILE: &str = "day03.txt";

fn get_highest_joltage(battery: &[u32]) -> u32 {
    let mut highest_joltage = 0;

    for (i, digit1) in battery.iter().enumerate() {
        for digit2 in &battery[i+1..] {
            let joltage = digit1 * 10 + digit2;

            if joltage > highest_joltage {
                highest_joltage = joltage;
            }
        }
    }

    highest_joltage
}

fn main() {
    let mut batteries: Vec<Vec<u32>> = Vec::new();
    for line in std::fs::read_to_string(INPUT_FILE)
        .expect("Error reading input file")
        .lines() {

        batteries.push(line.trim().chars().map(|c| c.to_digit(10).unwrap()).collect());
    }

    let total_joltage: u32 = batteries
        .iter()
        .map(|battery| get_highest_joltage(battery))
        .sum();

    println!("{}", total_joltage);
}

