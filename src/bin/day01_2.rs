const INPUT_FILE: &str = "day01.txt";

fn main() {
    let mut current_rotation = 50_i32;

    let mut point_counter = 0_i32; // counts the number of times the dial points at 0 (the puzzle output)

    for line in std::fs::read_to_string(INPUT_FILE)
        .expect("Error reading input file")
        .lines() {

        let direction: char = line.chars().next().expect("Invalid input");
        let turn_dist: i32 = line[1..].parse().expect("Invalid input");

        if direction == 'R' {
            point_counter += (current_rotation + turn_dist) / 100;

            current_rotation = (current_rotation + turn_dist) % 100;
        } else if direction == 'L' {
            point_counter += (current_rotation - turn_dist) / -100;
            if turn_dist >= current_rotation && current_rotation != 0 {
                point_counter += 1;
            }

            current_rotation = (current_rotation + 100 - turn_dist % 100) % 100;
        } else {
            panic!("Invalid input");
        }
    }

    println!("{}", point_counter);
}

