const INPUT_FILE: &str = "day06.txt";

fn main() {
    let mut digits: Vec<Vec<u64>> = Vec::new();
    let mut operators: Vec<char> = Vec::new();

    for line in std::fs::read_to_string(INPUT_FILE)
        .expect("Error reading from input file")
        .lines() {

        let first_char = line.chars().next().unwrap();
        if first_char != '+' && first_char != '*' {
            digits.push(line.chars().map(|c| c.to_digit(10).unwrap_or(0) as u64).collect());
        } else {
            operators = line
                .split_whitespace()
                .map(|substr| substr.chars().next().unwrap())
                .collect();
            break;
        }
    }

    let mut numbers: Vec<Vec<u64>> = Vec::new();

    let mut section: Vec<u64> = Vec::new();
    for i in 0..digits[0].len() {
        if digits.iter().all(|row| row[i] == 0) {
            numbers.push(section.clone());
            section.clear();

        } else {
            let mut num = 0_u64;
            for row in &digits {
                if row[i] == 0 {
                    if num == 0 { continue; }
                    else { break; }
                }

                num *= 10;
                num += row[i];
            }
            
            section.push(num);
        }
    }
    numbers.push(section.clone());

    let mut grand_total = 0_u64;
    for (i, op) in operators.iter().enumerate() {
        if *op == '+' {
            grand_total += numbers[i].iter().sum::<u64>();
        } else if *op == '*' {
            grand_total += numbers[i].iter().product::<u64>();
        }
    }

    println!("{}", grand_total);
}

