const INPUT_FILE: &str = "day01.txt";

fn main() {
    let mut current_rotation = 50_u32;

    let mut point_counter = 0_u32; // counts the number of times the dial points at 0 (the puzzle output)

    for line in std::fs::read_to_string(INPUT_FILE)
        .expect("Error reading input file")
        .lines() {

        let direction: char = line.chars().next().expect("Invalid input");
        let turn_dist: u32 = line[1..].parse::<u32>().expect("Invalid input") % 100;

        if direction == 'R' {
            current_rotation = (current_rotation + turn_dist) % 100;
        } else if direction == 'L' {
            current_rotation = (current_rotation + 100 - turn_dist) % 100;
        } else {
            panic!("Invalid input");
        }

        if current_rotation == 0 {
            point_counter += 1;
        }
    }

    println!("{}", point_counter);
}

