use std::collections::HashMap;

const INPUT_FILE: &str = "day08.txt";

const N_CONNECTIONS: usize = 1000;

// since if d1 < d2, d1^2 < d2^2, the square of the distance is used when comparing
fn dist_square((x1, y1, z1): (u64, u64, u64), (x2, y2, z2): (u64, u64, u64)) -> u64 {
    let dx = x1.abs_diff(x2);
    let dy = y1.abs_diff(y2);
    let dz = z1.abs_diff(z2);

    dx*dx + dy*dy + dz*dz
}

fn pairs_from_points(points: &[(u64, u64, u64)]) -> Vec<[(u64, u64, u64); 2]> {
    let mut ret = Vec::new();

    for (i, point1) in points.iter().enumerate() {
        ret.append(&mut points.iter().skip(i+1).map(|point2| [*point1, *point2]).collect());
    }

    ret
}

fn adjacency_list_from_pairs(pairs: &[[(u64, u64, u64); 2]]) -> HashMap<(u64, u64, u64), Vec<(u64, u64, u64)>> {
    let mut adjacency_list: HashMap<(u64, u64, u64), Vec<(u64, u64, u64)>> = HashMap::new();

    for [point1, point2] in pairs {
        if let Some(list) = adjacency_list.get_mut(point1) {
            list.push(*point2);
        } else {
            adjacency_list.insert(*point1, vec![*point2]);
        }

        if let Some(list) = adjacency_list.get_mut(point2) {
            list.push(*point1);
        } else {
            adjacency_list.insert(*point2, vec![*point1]);
        }
    }

    adjacency_list
}

fn groups_from_adjacency_list(adjacency_list: &HashMap<(u64, u64, u64), Vec<(u64, u64, u64)>>) -> Vec<Vec<(u64, u64, u64)>> {
    let mut groups: Vec<Vec<(u64, u64, u64)>> = Vec::new();

    for point in adjacency_list.keys() {
        let mut group = adjacency_list[point].clone();
        group.push(*point);
        groups.push(group);
    }

    let mut merged = true;
    'outer: while merged {

        merged = false;

        for (i, group1) in groups.iter().enumerate() {
            for group2 in &groups[i+1..] {
                if group1.iter().any(|point| group2.contains(point)) {
                    let merged_group: Vec<_> = group1
                        .iter()
                        .cloned()
                        .chain(group2.iter().filter(|point| !group1.contains(point)).cloned())
                        .collect();

                    let new_groups: Vec<_> = groups
                        .iter()
                        .filter(|group| *group != group1 && *group != group2)
                        .cloned()
                        .chain(std::iter::once(merged_group))
                        .collect();

                    groups = new_groups;

                    merged = true;

                    continue 'outer;
                }
            }
        }
    }

    groups
}

fn main() {
    let mut points: Vec<(u64, u64, u64)> = Vec::new();

    for line in std::fs::read_to_string(INPUT_FILE)
        .expect("Error reading input file")
        .lines() {
        let values: Vec<u64> = line
            .split(',')
            .map(|substr| substr.parse().unwrap())
            .collect();

        points.push((values[0], values[1], values[2]));
    }

    let mut pairs = pairs_from_points(&points);
    pairs.sort_by_key(|[point1, point2]| dist_square(*point1, *point2));
    pairs.drain(N_CONNECTIONS..);

    let adjacency_list = adjacency_list_from_pairs(&pairs);
    let groups = groups_from_adjacency_list(&adjacency_list);

    let mut group_lens: Vec<usize> = groups.into_iter().map(|group| group.len()).collect();
    group_lens.sort_by(|x, y| y.cmp(x)); // sort descending

    // println!("{:?}", group_lens);

    println!("{}", group_lens.into_iter().take(3).product::<usize>());
}

