use std::collections::HashSet;

const INPUT_FILE: &str = "day07.txt";

#[derive(PartialEq)]
enum Square {
    Empty,
    Splitter,
}

fn beams_split(initial_beam: (usize, usize), grid: &[Vec<Square>]) -> u32 {
    let mut split_count = 0_u32;

    let mut past_beams = HashSet::new();
    let mut beams = vec![initial_beam];

    while let Some(beam) = beams.pop() {
        if past_beams.contains(&beam) {
            continue;
        } else {
            past_beams.insert(beam);
        }

        if grid[beam.0 + 1][beam.1] == Square::Splitter {
            split_count += 1;

            if beam.0 + 2 < grid.len() {
                beams.push((beam.0 + 1, beam.1 - 1));
                beams.push((beam.0 + 1, beam.1 + 1));
            }
            
        } else { // no split
            if beam.0 + 2 < grid.len() {
                beams.push((beam.0 + 1, beam.1));
            }
        }
    }

    split_count
}

fn main() {
    let mut grid: Vec<Vec<Square>> = Vec::new(); // true = splitter, false = empty space
    let mut start = (0_usize, 0_usize);

    for line in std::fs::read_to_string(INPUT_FILE)
        .expect("Error reading input file")
        .lines() {
        
        let row = line
            .trim()
            .chars()
            .enumerate()
            .map(|(i, c)| match c {
                '.' => Square::Empty,
                'S' => { start = (0, i); Square::Empty },
                '^' => Square::Splitter,
                _   => panic!("Invalid character '{}' in input", c),
            })
            .collect();

        grid.push(row);
    }

    println!("{}", beams_split(start, &grid));
}

