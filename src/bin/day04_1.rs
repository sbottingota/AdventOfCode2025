const INPUT_FILE: &str = "day04.txt";

// note I'm using grid[x][y], i.e. x: down, y: right
fn neighbors(x: usize, y: usize, grid_width: usize, grid_height: usize) -> Vec<(usize, usize)> {
    let mut ret = Vec::new();

    if x > 0 {
        ret.push((x - 1, y));
        if y > 0 {
            ret.push((x - 1, y - 1));
        }
        if y < grid_width - 1 {
            ret.push((x - 1, y + 1));
        }
    }

    if x < grid_height - 1 {
        ret.push((x + 1, y));
        if y < grid_width - 1 {
            ret.push((x + 1, y + 1));
        }
        if y > 0 {
            ret.push((x + 1, y - 1));
        }
    }

    if y > 0 {
        ret.push((x, y - 1));
    }
    if y < grid_width - 1 {
        ret.push((x, y + 1));
    }

    ret
}

fn main() {
    let mut grid: Vec<Vec<bool>> = Vec::new();

    for line in std::fs::read_to_string(INPUT_FILE)
        .expect("Error reading input file")
        .lines() {
        grid.push(line.trim().chars().map(|c| c == '@').collect());
    }

    let mut rolls_removed = 0_u32;

    for x1 in 0..grid.len() {
        for y1 in 0..grid[x1].len() {
            if !grid[x1][y1] {
                continue;
            }

            let n_adjacent_rolls = neighbors(x1, y1, grid.len(), grid[x1].len())
                .into_iter()
                .filter(|(x2, y2)| grid[*x2][*y2]) // true = has roll, false = doesn't have roll
                .count();

            if n_adjacent_rolls < 4 {
                rolls_removed += 1;
            }
        }
    }

    println!("{}", rolls_removed);
}

