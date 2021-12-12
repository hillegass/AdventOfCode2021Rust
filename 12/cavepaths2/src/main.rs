use std::collections::HashMap;
use std::collections::HashSet;
use std::io::{self, BufRead};

fn is_lowercase(s: &str) -> bool {
    s.chars().all(|c| c.is_lowercase())
}

fn all_paths_with_prefix(
    prefix: &Vec<String>,
    graph: &HashMap<String, Vec<String>>,
    free_path: &Option<String>,
) -> Vec<Vec<String>> {
    let mut paths = Vec::new();
    let last_node = prefix.last().unwrap();
    if last_node == "end" {
        paths.push(prefix.clone());
    } else {
        for next_node in graph.get(last_node).unwrap() {
            if is_lowercase(next_node) && prefix.contains(next_node) {
                if let Some(free_path) = free_path {
                    // Can I use my free pass?
                    if *free_path == *next_node {
                        let mut new_prefix = prefix.clone();
                        new_prefix.push(next_node.clone());
                        paths.append(&mut all_paths_with_prefix(
                            &new_prefix,
                            graph,
                            &Option::None,
                        ));
                    }
                };
            } else {
                let mut new_prefix = prefix.clone();
                new_prefix.push(next_node.clone());
                paths.append(&mut all_paths_with_prefix(&new_prefix, graph, free_path));
            }
        }
    }
    paths
}

fn main() {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let linestr = line.unwrap().clone();
        let values = linestr.split("-").collect::<Vec<_>>();
        if values.len() < 2 {
            continue;
        }
        let a = String::from(values[0]);
        let b = String::from(values[1]);

        if a.ne("end") && b.ne("start") {
            println!("{}->{}", a, b);
            if map.contains_key(&a) {
                let v = map.get_mut(&a).unwrap();
                v.push(b.to_string());
            } else {
                let mut v = Vec::new();
                v.push(b.to_string());
                map.insert(a.to_string(), v);
            }
        }

        if b.ne("end") && a.ne("start") {
            println!("{}->{}", b, a);
            if map.contains_key(&b) {
                let v = map.get_mut(&b).unwrap();
                v.push(a.to_string());
            } else {
                let mut v = Vec::new();
                v.push(a.to_string());
                map.insert(b.to_string(), v);
            }
        }
    }
    println!("{:?}", map);

    let mut free_passes = Vec::new();
    for key in map.keys() {
        if is_lowercase(key) && key.ne("start") {
            free_passes.push(key);
        }
    }
    let mut pathset = HashSet::new();
    for pass in free_passes {
        let all_paths = all_paths_with_prefix(
            &vec![String::from("start")],
            &map,
            &Option::Some(pass.clone()),
        );
        for path in all_paths {
            pathset.insert(path);
        }
    }

    //println!("{:?}", all_paths);
    println!("{}", pathset.len());
}
