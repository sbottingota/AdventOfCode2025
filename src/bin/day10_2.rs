use z3::Solver;
use z3::ast::Int;

const INPUT_FILE: &str = "day10.txt";

#[derive(Debug)]
struct Machine {
    joltage_requirements: Vec<u64>,
    buttons: Vec<Vec<u64>>,
}

impl Machine {
    fn new(joltage_requirements: Vec<u64>, buttons: Vec<Vec<u64>>) -> Self {
        Self {
            joltage_requirements,
            buttons,
        }
    }


    fn min_buttonpresses(&self) -> u64 {
        let buttonpresses: Vec<Int> = (0..self.buttons.len())
            .map(|i| Int::fresh_const(&format!("button{}", i)))
            .collect();

        let solver = Solver::new();

        for buttonpress in &buttonpresses {
            solver.assert(buttonpress.ge(0));
        }

        for (i, joltage_target) in self.joltage_requirements.iter().enumerate() {
            let joltage = self.buttons
                .iter()
                .enumerate()
                .filter(|(_, button)| button.contains(&(i as u64)))
                .fold(Int::from_u64(0), |acc, (j, _)| acc + &buttonpresses[j]);

            solver.assert(joltage.eq(*joltage_target));
        }

        let best_solution = solver
            .solutions(buttonpresses, false)
            .map(|solution| solution.iter().map(Int::as_u64).map(Option::unwrap).sum())
            .min();

        best_solution.expect("No solutions found")
    }
}

fn parse_parens(s: &str) -> Vec<u64> {
    let mut chars = s.chars();
    chars.next();
    chars.next_back();
    let s = chars.as_str();

    s.split(',')
        .map(|substr| substr.parse().unwrap())
        .collect()
}

fn main() {
    let mut machines: Vec<Machine> = Vec::new();

    for line in std::fs::read_to_string(INPUT_FILE)
        .expect("Error reading input file")
        .lines() {

        let mut tokens: Vec<_> = line.split_whitespace().collect();

        let joltage_requirements = parse_parens(tokens[tokens.len()-1]);

        let mut buttons = Vec::new();

        let tokens_len = tokens.len();
        for s in &mut tokens[1..tokens_len-1] {
            buttons.push(parse_parens(s));
        }

        machines.push(Machine::new(joltage_requirements, buttons));
    }

    let mut total_buttonpresses = 0_u64;
    for machine in machines {
        let buttonpresses = machine.min_buttonpresses();
        // println!("{}", buttonpresses);
        total_buttonpresses += buttonpresses;
    }

    println!("{}", total_buttonpresses);
}

