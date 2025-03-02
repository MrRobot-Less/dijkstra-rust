mod lib;

use crate::lib::graph::{Vertex, Edge, Graph};
use std::collections::{HashSet, VecDeque};

fn setup() -> Graph {
    let mut head = Vertex::new("Start".to_string());
    let mut node1 = Vertex::new("a".to_string());
    let mut node2 = Vertex::new("b".to_string());
    let end = Vertex::new("End".to_string());

    head.add_edge(&node2, 5);
    head.add_edge(&node1, 10);
    node2.add_edge(&node1, 7);
    node1.add_edge(&end, 1);
    node2.add_edge(&end, 10);
    
    Graph::new(&vec![head, node1, node2, end])
}

fn main() {
    let mut graph = setup();
    let mut _processed : HashSet<String> = HashSet::new();

    let start = graph.head.clone().unwrap();
    let mut queue: VecDeque<Edge> = VecDeque::from(*start.get_edges_orderby_cost());

    while !queue.is_empty() {
        if let Some(edge) = queue.pop_front() {
            let vertex_from = &edge.from.clone();
            let vertex_to = &edge.to.clone();

            if let Some(cost_record) = &graph.get_cost(vertex_to.clone()) {
                if &edge.weight < &cost_record.total {
                    graph.update_cost(vertex_to.clone(), vertex_from.clone(), &edge.weight);
                }
            }

            if let Some(vertex) = graph.get_vertex(vertex_to.clone()) {
                let mut new_edges_to_enqueue = vertex.get_edges_orderby_cost().clone();
                for new_edge in new_edges_to_enqueue.iter_mut() {
                    new_edge.increment_cost(edge.weight);
                    queue.push_back(new_edge.clone());
                }
            }
        }
    }

    if let Some(mut from) = graph.get_cost("End".to_string()) {
        println!("[FROM]: {}\n[TO]: {}\n", "End".to_string(), &from.previous);
        while let Some(to) = graph.get_cost(from.previous.clone()) {
            if to.previous.is_empty() { break; }
            println!("[FROM]: {}\n[TO]: {}\n", from.previous, &to.previous);
            from = to.clone();
        }
        println!("Total: {}", graph.get_cost("End".to_string()).unwrap().total);
    }
}