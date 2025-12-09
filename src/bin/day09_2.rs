use geo::algorithm::contains::Contains;
use geo::{Polygon, Rect, LineString, Coord};

const INPUT_FILE: &str = "day09.txt";

fn area_between((x1, y1): (u64, u64), (x2, y2): (u64, u64)) -> u64 {
    (x1.abs_diff(x2) + 1) * (y1.abs_diff(y2) + 1)
}

// find the largest area with red tiles for 2 of its vertices which only contains red and green tiles
fn largest_area(red_tiles: &[(u64, u64)]) -> u64 {
    let polygon = Polygon::new(LineString::from(red_tiles.iter().map(|(x, y)| (*x as f64, *y as f64)).collect::<Vec<_>>()), vec![]);

    let mut max_enclosed_area = 0_u64;

    for &tile1 in red_tiles {
        for &tile2 in red_tiles {
            let rect = Rect::new(
                Coord { x: tile1.0 as f64, y: tile1.1 as f64 },
                Coord { x: tile2.0 as f64, y: tile2.1 as f64 }
            ).to_polygon();
                
            let enclosed_area = area_between(tile1, tile2);
            if enclosed_area > max_enclosed_area && polygon.contains(&rect) {
                max_enclosed_area = enclosed_area;
            }
        }
    }

    max_enclosed_area
}

fn main() {
    let mut red_tiles: Vec<(u64, u64)> = Vec::new();
    for line in std::fs::read_to_string(INPUT_FILE)
        .expect("Error reading input file")
        .lines() {
        let numbers: Vec<u64> = line
            .split(',')
            .map(|substr| substr.parse().unwrap())
            .collect();

        red_tiles.push((numbers[0], numbers[1]));
   }

    println!("{}", largest_area(&red_tiles));
}

