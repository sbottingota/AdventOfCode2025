use std::collections::HashSet;

const INPUT_FILE: &str = "day08.txt";

const N_CONNECTIONS: usize = 10;

// since if d1 < d2, d1^2 < d2^2, the square of the distance is used when comparing
fn dist_square((x1, y1, z1): (u64, u64, u64), (x2, y2, z2): (u64, u64, u64)) -> u64 {
    let dx = x1.abs_diff(x2);
    let dy = y1.abs_diff(y2);
    let dz = z1.abs_diff(z2);

    dx*dx + dy*dy + dz*dz
}

fn main() {
    let mut jboxes: Vec<(u64, u64, u64)> = Vec::new();

    for line in std::fs::read_to_string(INPUT_FILE)
        .expect("Error reading input file")
        .lines() {
        let values: Vec<u64> = line
            .split(',')
            .map(|substr| substr.parse().unwrap())
            .collect();

        jboxes.push((values[0], values[1], values[2]));
    }

    let mut connections: Vec<Vec<(u64, u64, u64)>> = Vec::new();
    let mut already_connected: HashSet<[(u64, u64, u64); 2]> = HashSet::new();

    for _ in 0..N_CONNECTIONS {
        let mut min_box1 = (0_u64, 0_u64, 0_u64);
        let mut min_box2 = (u32::MAX as u64, 0_u64, 0_u64); // replace with a better dummy value if necessary

        for jbox1 in &jboxes {
            let jbox2 = jboxes
                .iter()
                .filter(|jbox2| jbox1 != *jbox2 && !already_connected.contains(&[*jbox1, **jbox2]) && !already_connected.contains(&[**jbox2, *jbox1]))
                .min_by_key(|pos| dist_square(**pos, *jbox1))
                .unwrap();

            if dist_square(*jbox1, *jbox2) < dist_square(min_box1, min_box2) {
                min_box1 = *jbox1;
                min_box2 = *jbox2;
            }
        }

        let mut connected = false;
        for connection in connections.iter_mut() {
            let contains_box1 = connection.contains(&min_box1);
            let contains_box2 = connection.contains(&min_box2);

            if contains_box1 && contains_box2 {
                connected = true;
                break;
            } else if contains_box1 {
                connection.push(min_box2);
                connected = true;
                break;
            } else if contains_box2 {
                connection.push(min_box1);
                connected = true;
                break;
            }
        }

        if !connected {
            connections.push(vec![min_box1, min_box2]);
        }

        already_connected.insert([min_box1, min_box2]);

        // jboxes1.retain(|jbox| *jbox != min_box1 && *jbox != min_box2);
    }

    connections.sort_by(|connection1, connection2| connection2.len().cmp(&connection1.len()));

    println!("{:?}", connections.iter().map(|connection| connection.len()).collect::<Vec<_>>());
    // println!("{:?}", connections.iter().take(3).collect::<Vec<_>>());
    println!("{}", connections.into_iter().take(3).map(|connection| connection.len()).product::<usize>());
}

