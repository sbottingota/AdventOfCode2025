const INPUT_FILE: &str = "day09.txt";

fn area_between((x1, y1): (u64, u64), (x2, y2): (u64, u64)) -> u64 {
    (x1.abs_diff(x2) + 1) * (y1.abs_diff(y2) + 1)
}

fn largest_area(tiles: &[(u64, u64)]) -> u64 {
    let mut max_enclosed_area = 0_u64;

    for tile1 in tiles {
        for tile2 in tiles {
            let enclosed_area = area_between(*tile1, *tile2);
            if enclosed_area > max_enclosed_area {
                max_enclosed_area = enclosed_area;
            }
        }
    }

    max_enclosed_area
}

fn main() {
    let mut tiles: Vec<(u64, u64)> = Vec::new();
    for line in std::fs::read_to_string(INPUT_FILE)
        .expect("Error reading input file")
        .lines() {
        let numbers: Vec<u64> = line
            .split(',')
            .map(|substr| substr.parse().unwrap())
            .collect();

        tiles.push((numbers[0], numbers[1]));
   }

    println!("{}", largest_area(&tiles));
}

