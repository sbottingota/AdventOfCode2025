const INPUT_FILE: &str = "day06.txt";

fn main() {
    let mut numbers: Vec<Vec<u64>> = Vec::new();
    let mut operators: Vec<char> = Vec::new();

    for line in std::fs::read_to_string(INPUT_FILE)
        .expect("Error reading from input file")
        .lines() {

        let line_nums: Vec<_> = line.split_whitespace()
            .map(|substr| substr.parse::<u64>())
            .collect();

        if line_nums[0].is_ok()  {
            numbers.push(line_nums.into_iter().map(|num| num.unwrap()).collect());
        } else {
            operators = line
                .split_whitespace()
                .map(|substr| substr.chars().next().unwrap())
                .collect();
            break;
        }
    }

    let mut grand_total = 0_u64;
    for (i, op) in operators.iter().enumerate() {
        let mut res = if *op == '+' {
            numbers[0][i] + numbers[1][i]
        } else {
            numbers[0][i] * numbers[1][i]
        };

        for j in 2..numbers.len() {
            if *op == '+' {
                res += numbers[j][i];
            } else if *op == '*' {
                res *= numbers[j][i];
            }
        }

        // println!("{}", res);
        grand_total += res;
    }

    println!("{}", grand_total);
}

