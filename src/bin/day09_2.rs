use itertools::Itertools;
use std::collections::HashSet;

const INPUT_FILE: &str = "day09.txt";

fn area_between((x1, y1): (u64, u64), (x2, y2): (u64, u64)) -> u64 {
    (x1.abs_diff(x2) + 1) * (y1.abs_diff(y2) + 1)
}

fn is_valid_enclosure((x1, y1): (u64, u64), (x2, y2): (u64, u64), red_tiles: &[(u64, u64)], green_tiles: &HashSet<(u64, u64)>) -> bool {
    // swap coords if necessary
    let (x1, x2) = if x1 > x2 { (x2, x1) } else { (x1, x2) };
    let (y1, y2) = if y1 > y2 { (y2, y1) } else { (y1, y2) };

    for x in x1..=x2 {
        for y in y1..=y2 {
            if !green_tiles.contains(&(x, y)) && !red_tiles.contains(&(x, y)) {
                return false;
            }
        }
    }

    true
}

// find the largest area with red tiles for 2 of its vertices which only contains red and green tiles
fn largest_area(red_tiles: &[(u64, u64)], green_tiles: &HashSet<(u64, u64)>) -> u64 {
    let mut max_enclosed_area = 0_u64;

    for &tile1 in red_tiles {
        for &tile2 in red_tiles {
            let enclosed_area = area_between(tile1, tile2);
            if enclosed_area > max_enclosed_area
                && is_valid_enclosure(tile1, tile2, red_tiles, green_tiles) {

                max_enclosed_area = enclosed_area;
            }
        }
    }

    max_enclosed_area
}

fn get_green_tiles(red_tiles: &[(u64, u64)]) -> HashSet<(u64, u64)> {
    let mut green_tiles = HashSet::new();

    for (&(x1, y1), &(x2, y2)) in red_tiles.iter().circular_tuple_windows() {
        if x1 == x2 {
            let (y1, y2) = if y1 > y2 { (y2, y1) } else { (y1, y2) }; // swap if they're the other way around
            for y in y1+1..y2 {
                green_tiles.insert((x1, y));
            }
        } else { // y1 == y2
            let (x1, x2) = if x1 > x2 { (x2, x1) } else { (x1, x2) }; // swap if they're the other way around
            for x in x1+1..x2 {
                green_tiles.insert((x, y1));
            }
        }
    }

    let red_tiles: HashSet<(u64, u64)> = HashSet::from_iter(red_tiles.to_vec());
    println!("step 1 done");

    for (i, mut tile) in green_tiles.clone().into_iter().enumerate() {
        if i % 5_000 == 0 {
            println!("{}/{}", i, green_tiles.len());
        }

        let mut new_tiles = HashSet::new();
        while tile.0 > 0 {
            tile.0 -= 1;
            if green_tiles.contains(&tile) || red_tiles.contains(&tile) {
                green_tiles.extend(&new_tiles);
                break;
            }

            new_tiles.insert(tile);
        }

        // if the edge of the grid is reached, do nothing
    }


    green_tiles
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

    let green_tiles = get_green_tiles(&red_tiles);

    println!("parse");

    println!("{}", largest_area(&red_tiles, &green_tiles));
}

