use std::collections::HashMap;
use std::collections::HashSet;
use std::io;
use std::iter::FromIterator;

fn main() {
    println!("Enter your space-separated graph links in the following form a=b:\n");
    let mut graph_input = String::new();
    io::stdin()
        .read_line(&mut graph_input)
        .expect("Failed to read graph input");
    let graph_input = graph_input; //feels right to make this immutable

    let mut vtx_map: HashMap<String, HashSet<String>> = HashMap::new();

    build_graph(graph_input, &mut vtx_map);
    println!("\nFirst-Degree Relationships:\n{:?}", vtx_map);

    loop {
        println!("\nEnter a discontinuity to test for in the form a!=b or 'q' to quit:\n");
        let mut disc_input = String::new();
        io::stdin()
            .read_line(&mut disc_input)
            .expect("Failed to read discontinuity input");
        let disc_input = disc_input;
        match disc_input.trim() {
            "q" => break,
            t => {
                let v: Vec<&str> = t.split("!=").collect();
                if v.len() == 2 {
                    let x = v[0];
                    let y = v[1];
                    if !vtx_map.contains_key(x) {
                        println!("Indeterminate because {} was not found", x);
                    } else if !vtx_map.contains_key(y) {
                        println!("Indeterminate because {} was not found", y);
                    } else {
                        println!("Searching for a connection between {} and {}", x, y);
                        let path: &mut Vec<String> = &mut Vec::new();
                        let is_discontiguous = search_connections(&vtx_map, x, y, path);
                        if is_discontiguous {
                            println!("No connection found");
                        }
                    }
                } else {
                    println!("Ignored term: {}", t)
                }
            }
        }
    }
}

fn build_graph(graph_input: String, vtx_map: &mut HashMap<String, HashSet<String>>) {
    let term_iter = graph_input.split_whitespace();
    for t in term_iter {
        let v: Vec<&str> = t.split('=').collect();
        if v.len() == 2 {
            let a = v[0].to_string();
            let b = v[1].to_string();

            let a_node = vtx_map.entry(a.clone()).or_insert(HashSet::new());
            a_node.insert(b.clone());
            //            println!("{}:{:?}", a, a_node);

            let b_node = vtx_map.entry(b.clone()).or_insert(HashSet::new());
            b_node.insert(a.clone());
        //            println!("{}:{:?}", b, b_node);
        } else {
            println!("Ignored term: {}", t)
        }
    }
}

fn search_connections(
    vtx_map: &HashMap<String, HashSet<String>>,
    x: &str,
    y: &str,
    path: &mut Vec<String>,
) -> bool {
    let start_node = vtx_map
        .get(&x.to_string())
        .cloned()
        .unwrap_or(HashSet::new());
    path.push(x.to_string());
    let path_set = HashSet::from_iter(path.iter().cloned());
    println!("looking for {} in {}", y, x);
    if start_node.contains(y) {
        println!(
            "{} had a connection to {} through the path {:?}",
            x, y, path
        );
        false
    } else {
        start_node
            .difference(&path_set)
            .into_iter()
            .all(|c| search_connections(vtx_map, &c, y, path))
    }
}
