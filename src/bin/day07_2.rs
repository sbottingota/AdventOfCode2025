use memoize::memoize;

const INPUT_FILE: &str = "day07.txt";

#[derive(PartialEq, Eq, Hash, Clone)]
enum Square {
    Empty,
    Splitter,
}

#[memoize]
fn count_timelines(mut beam: (usize, usize), grid: Vec<Vec<Square>>) -> u64 {
    while grid[beam.0][beam.1] == Square::Empty {
        beam.0 += 1;

        if beam.0 + 1 >= grid.len() {
            return 1;
        }
    }

    count_timelines((beam.0, beam.1 - 1), grid.clone()) + count_timelines((beam.0, beam.1 + 1), grid.clone())
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

    println!("{}", count_timelines(start, grid));
}

