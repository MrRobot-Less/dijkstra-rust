mod lib;

use crate::lib::node_edge::{Vertex, Edge, Graph};
use std::collections::{HashSet, VecDeque};

fn setup() -> Graph {

    let mut head = Vertex::new("Start".to_string());
    let mut node1 = Vertex::new("a".to_string());
    let mut node2 = Vertex::new("b".to_string());
    let end = Vertex::new("End".to_string());

    head.add_edge(&node2, 5);
    head.add_edge(&node1, 10);

    node2.add_edge(&node1, 3);

    node1.add_edge(&end, 5);
    node2.add_edge(&end, 5);
    
    Graph::new(&vec![head, node1, node2, end])
}

fn main() {
    let mut graph = setup();
    let mut processed : HashSet<String> = HashSet::new();

    let start = graph.head.as_ref().unwrap();
    let mut queue: VecDeque<Edge> = VecDeque::from(*start.get_edges_orderby_cost());
    while !queue.is_empty() {
    
        if let Some(edge) = queue.pop_front() {
            let vertex_from = edge.from.clone();
            let vertex_to = edge.to.clone();

            if processed.get(&vertex_from).is_some() { continue; }
            processed.insert(vertex_from.clone());
    
            if let Some(cost_record) = graph.get_cost(vertex_from.clone()) {
                if edge.cost < cost_record.total {
                    graph.update_cost(vertex_from, vertex_to.clone(), edge.cost);
                }
            }

            if let Some(vertex) = graph.get_vertex(vertex_to.clone()) {
                let mut new_edges_to_enqueue = vertex.get_edges_orderby_cost().clone();
                for new_edge in new_edges_to_enqueue.iter_mut() {
                    new_edge.increment_cost(edge.cost);
                    queue.push_back(new_edge.clone());
                }
            }
        }
    }
    
    if let Some(mut from) = graph.get_cost("Start".to_string()) {
        println!("{} - {}", "Start", &from.next);
        let mut total = 0;
        while let Some(to) = graph.get_cost(from.next.clone()) {
            if to.next.is_empty() { break; }
            println!("{} - {}", from.next, &to.next);
            from = to.clone();
            total = to.total as i32;
        }

        println!("Total: {}", total);
    }
}