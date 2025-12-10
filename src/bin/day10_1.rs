use std::collections::HashSet;

const INPUT_FILE: &str = "day10.txt";

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Machine {
    lights: Vec<bool>,
    required_lights: Vec<bool>,
    buttons: Vec<Vec<usize>>,
}

impl Machine {
    fn new(required_lights: Vec<bool>, buttons: Vec<Vec<usize>>) -> Self {
        let lights_len = required_lights.len();
        Self { required_lights, buttons, lights: vec![false; lights_len] }
    }

    fn min_buttonpresses(&self) -> u32 {
        let mut seen: HashSet<Self> = HashSet::from([self.clone()]);
        let mut to_visit: Vec<Self> = vec![self.clone()];

        for buttonpresses in 1.. {
            let mut new_to_visit: Vec<Self> = Vec::new();
            while let Some(current) = to_visit.pop() {
                for i in 0..current.buttons.len() {
                    let next = current.pressed(i);

                    if next.lights == next.required_lights {
                        return buttonpresses;
                    } else if !seen.contains(&next) {
                        new_to_visit.push(next.clone());
                        seen.insert(next);
                    }
                }
            }

            to_visit = new_to_visit;
        }

        unreachable!()
    }

    fn pressed(&self, button: usize) -> Self {
        let mut new = self.clone();
        for light in &self.buttons[button] {
            new.lights[*light] = !new.lights[*light];
        }

        new
    }
}

fn parse_parens(s: &str) -> Vec<usize> {
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

        let mut light_chars = tokens[0].chars();
        light_chars.next();
        light_chars.next_back();

        let lights: Vec<bool> = light_chars
            .map(|c|
                match c {
                    '#' => true,
                    '.' => false,
                    _   => panic!("Invalid character '{}' found while parsing input file", c),
                }
            ).collect();

        let mut buttons = Vec::new();

        let tokens_len = tokens.len();
        for s in &mut tokens[1..tokens_len-1] {
            buttons.push(parse_parens(s));
        }

        machines.push(Machine::new(lights, buttons));
    }

    let mut total_buttonpresses = 0_u32;
    for machine in machines {
        total_buttonpresses += machine.min_buttonpresses();
    }

    println!("{}", total_buttonpresses);
}
