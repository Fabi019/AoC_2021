use std::collections::{HashMap, HashSet};

fn main() {
    let data = std::fs::read_to_string("assets/day12.txt").unwrap();
    let mut graph = Graph::new(&data);
    let paths = find_paths("start", "end", &mut graph, Vec::new());
    println!("{:#?}", paths);
    print!("Number of paths: {}", paths.len());
}

fn find_paths<'a>(
    start: &'a str,
    end: &'a str,
    g: &mut Graph,
    mut path: Vec<String>,
) -> Vec<Vec<String>> {
    path.push(start.to_string());
    if start == end {
        return vec![path];
    }
    let mut paths = Vec::new();
    let nodes = g.map.get(start).unwrap().clone();
    for node in nodes {
        let large_cave = node.chars().next().unwrap().is_uppercase();

        // Start part two
        let small_caves: Vec<&String> = path
            .iter()
            .filter(|n| n.chars().next().unwrap().is_lowercase())
            .collect();

        let unique = small_caves.iter().collect::<HashSet<_>>().len();
        let no_dup_small =
            !large_cave && small_caves.len() == unique && node != "start" && node != "end";
        // End part two

        if !path.contains(&node) || large_cave || no_dup_small {
            paths.extend(find_paths(&node, end, g, path.clone()));
        }
    }
    paths
}

struct Graph {
    map: HashMap<String, Vec<String>>,
}

impl Graph {
    fn new(edges: &str) -> Graph {
        let map = HashMap::new();
        let mut graph = Graph { map };
        edges.lines().for_each(|l| {
            let mut from_to = l.split('-');
            graph.add_edge(from_to.next().unwrap(), from_to.next().unwrap());
        });
        graph
    }

    fn add_edge(&mut self, from: &str, to: &str) {
        self.map
            .entry(from.to_string())
            .or_insert_with(Vec::new)
            .push(to.to_string());
        self.map
            .entry(to.to_string())
            .or_insert_with(Vec::new)
            .push(from.to_string());
    }
}
