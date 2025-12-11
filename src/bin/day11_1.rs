use std::collections::HashMap;

const INPUT_FILE: &str = "day11.txt";

fn count_paths(from: &str, to: &str, graph: &HashMap<String, Vec<String>>) -> u32 {
    if from == to {
        1
    } else {
        graph[from].iter().map(|node| count_paths(node, to, graph)).sum()
    }
}

fn main() {
    let mut graph: HashMap<String, Vec<String>> = HashMap::new(); // adjacency list
    for line in std::fs::read_to_string(INPUT_FILE)
        .expect("Error reading input file")
        .lines() {

        let tokens: Vec<String> = line.split_whitespace().map(|s| s.to_string()).collect();

        let key = &tokens[0][..tokens[0].len()-1]; // cut off trailing colon
        graph.insert(key.to_string(), tokens[1..].to_vec());
    }

    println!("{}", count_paths("you", "out", &graph));
}

