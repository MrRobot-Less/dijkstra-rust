use std::{collections::HashMap, f32::INFINITY};

#[derive(Clone)]
#[derive(Debug)]
pub struct Vertex {
    pub name: String,
    pub edges: Box<Vec<Edge>>
}

#[derive(Clone)]
#[derive(Debug)]
pub struct Edge {
    pub from: String,
    pub to: String,
    pub cost: i32
}

#[derive(Clone)]
#[derive(Debug)]
pub struct Graph {
    pub head: Option<Vertex>,
    hash: HashMap<String, Vertex>,
    pub cost: HashMap<String, Cost>
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Cost {
    pub next: String,
    pub total: i32
}

impl Vertex {
    pub fn new(name: String) -> Vertex {
        Vertex { name, edges: Box::new(Vec::new()) }
    }

    pub fn add_edge(&mut self, to: &Vertex, cost: i32) {
        self.edges.push(Edge::new(self.name.clone(), to.name.clone(), cost));
    }

    pub fn get_edges_orderby_cost(&self) -> Box<Vec<Edge>> {
        let mut edges: Box<Vec<Edge>> = self.edges.clone();
        edges.sort_by(|a,b| { a.cost.cmp(&b.cost) });
        edges
    }
}

impl Edge {
    fn new(from: String, to: String, cost: i32) -> Edge {
        Edge { from, to, cost }
    }

    pub fn increment_cost(&mut self, cost: i32) {
        self.cost += cost;
    }
}

impl Graph {
    pub fn new(vertexs: &Vec<Vertex>) -> Graph {
        let first = vertexs.first();
        let mut hashmap_vertex: HashMap<String, Vertex> = HashMap::new();
        let mut hashmap_cost: HashMap<String, Cost> = HashMap::new();
        if let Some(head) = first {
            for vertex in vertexs.iter() {
                hashmap_vertex.insert(vertex.name.clone(), vertex.clone());
                hashmap_cost.insert(vertex.name.clone(), Cost::default());
            }
            return Graph { head: Some(head.clone()), hash: hashmap_vertex, cost: hashmap_cost };
        }
        Graph { head: None, hash: hashmap_vertex, cost: hashmap_cost }
    }

    pub fn get_vertex(&self, name: String) -> Option<Vertex> {
        self.hash.get(&name).cloned()
    }

    pub fn get_cost(&self, name: String) -> Option<Cost> {
        self.cost.get(&name).cloned()
    }

    pub fn update_cost(&mut self, name: String, next: String, new_value: i32) {
        if let Some(mut cost) = self.get_cost(name.clone()) {
            cost.total = new_value;
            cost.next = next;
            self.cost.insert(name, cost);
        }
    }
}

impl Default for Cost {
    fn default() -> Self {
        Cost {
            next: "".to_string(),
            total: INFINITY as i32
        } 
    }
}