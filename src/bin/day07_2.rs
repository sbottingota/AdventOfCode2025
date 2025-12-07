use std::collections::HashMap;

const INPUT_FILE: &str = "day07.txt";

#[derive(PartialEq, Eq, Hash, Clone)]
enum Square {
    Empty,
    Splitter,
}

fn count_timelines(mut beam: (usize, usize), grid: Vec<Vec<Square>>, cached: &mut HashMap<(usize, usize), u64>) -> u64 {
    while grid[beam.0][beam.1] == Square::Empty {
        beam.0 += 1;

        if beam.0 + 1 >= grid.len() {
            return 1;
        }
    }

    if cached.contains_key(&beam) {
        return cached[&beam];
    }

    let timelines = count_timelines((beam.0, beam.1 - 1), grid.clone(), cached)
        + count_timelines((beam.0, beam.1 + 1), grid.clone(), cached);

    cached.insert(beam, timelines);
    timelines
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

    println!("{}", count_timelines(start, grid, &mut HashMap::new()));
}

