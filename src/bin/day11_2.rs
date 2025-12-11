use std::collections::HashMap;

const INPUT_FILE: &str = "day11.txt";

 fn count_paths<'a>(from: &'a str, to: &'a str, graph: &'a HashMap<String, Vec<String>>, cache: &mut HashMap<[&'a str; 2], u64>) -> u64 {
    if from == to {
        1
    } else if let Some(paths) = cache.get(&[from, to]) {
        *paths
    } else if graph.contains_key(from) {
            let paths = graph[from].iter().map(|node| count_paths(node, to, graph, cache)).sum();
            cache.insert([from, to], paths);
            paths
    } else {
        0
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

    let mut cache = HashMap::new();

    let total_paths = count_paths("svr", "dac", &graph, &mut cache)
        * count_paths("dac", "fft", &graph, &mut cache)
        * count_paths("fft", "out", &graph, &mut cache)

        + count_paths("svr", "fft", &graph, &mut cache)
        * count_paths("fft", "dac", &graph, &mut cache)
        * count_paths("dac", "out", &graph, &mut cache);

    println!("{}", total_paths);
}

