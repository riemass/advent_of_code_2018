use std::io::{self, BufRead};

#[derive(Default, Debug)]
struct Node {
    child_ids: Vec<usize>,
    metadata: Vec<i32>,
}

fn parse_nodes(input: &Vec<i32>, nodes: &mut Vec<Node>, next: &mut usize) -> usize {
    let mut new_node = Node::default();
    let num_of_children = input[*next];
    *next += 1;
    let num_of_metadata = input[*next];
    *next += 1;
    for _i in 0..num_of_children {
        new_node.child_ids.push(parse_nodes(input, nodes, next));
    }
    for i in 0..num_of_metadata {
        new_node.metadata.push(input[*next + i as usize]);
    }
    *next += num_of_metadata as usize;
    // println!("{:?}", new_node);
    nodes.push(new_node);
    (nodes.len() - 1)
}

fn compute_value(id: usize, nodes: &Vec<Node>, lookup: &mut Vec<Option<i32>>) -> i32 {

    if lookup[id].is_some() {
        println!("for node {} unwrapping", id);
        return lookup[id].unwrap();
    }

    let mut hash = 0;
    if nodes[id].child_ids.len() == 0 {
        println!("For node: {}, in metadata", id);
        hash = nodes[id].metadata.iter().sum();
    } else {
        hash = nodes[id]
            .metadata
            .iter()
            .filter(|&child_id| ((*child_id - 1) as usize) < nodes[id].child_ids.len())
            .map(|&child_id| {
                let node_id = nodes[id].child_ids[(child_id - 1) as usize];
                println!("calling for {} from {}", node_id, id);
                compute_value(node_id, nodes, lookup)
            })
            .sum();
    }
    println!("Setting lookup[{}] = Some({})", id, hash);
    lookup[id] = Some(hash);
    hash
}

fn main() {
    let mut input = String::new();
    io::stdin().lock().read_line(&mut input).unwrap();
    let code = input
        .trim()
        .split_whitespace()
        .filter_map(|num| num.parse::<i32>().ok())
        .collect::<Vec<i32>>();

    let mut nodes = Vec::new();
    let mut i = 0;
    let root_node_it = parse_nodes(&code, &mut nodes, &mut i);

    let hashcode: i32 = nodes
        .iter()
        .map(|node| node.metadata.iter().sum::<i32>())
        .sum();

    let mut lookup: Vec<Option<i32>> = Vec::new();
    lookup.resize(nodes.len(), None);

    compute_value(root_node_it, &nodes, &mut lookup);

    // println!("{:?}", hashcode);
    println!("{:?}", nodes);
    // println!("root hash: {:?}", lookup[root_node_it]);
    println!("root values: {:?}", lookup);
}
