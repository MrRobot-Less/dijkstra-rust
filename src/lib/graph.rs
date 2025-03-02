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
    pub weight: i32
}

#[derive(Clone)]
#[derive(Debug)]
pub struct Graph {
    pub head: Option<Vertex>,
    pub hash: HashMap<String, Vertex>,
    pub cost: HashMap<String, Cost>
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Cost {
    pub previous: String,
    pub total: i32
}

impl Vertex {
    pub fn new(name: String) -> Vertex {
        Vertex { name, edges: Box::new(Vec::new()) }
    }

    pub fn add_edge(&mut self, to: &Vertex, weight: i32) {
        self.edges.push(Edge::new(self.name.clone(), to.name.clone(), weight));
    }

    pub fn get_edges_orderby_cost(&self) -> Box<Vec<Edge>> {
        let mut edges: Box<Vec<Edge>> = self.edges.clone();
        edges.sort_by(|a,b| { a.weight.cmp(&b.weight) });
        edges
    }
}

impl Edge {
    fn new(from: String, to: String, weight: i32) -> Edge {
        Edge { from, to, weight }
    }

    pub fn increment_cost(&mut self, weight: i32) {
        self.weight += weight;
    }
}

impl Graph {
    pub fn new(vertices: &Vec<Vertex>) -> Graph {
        let first = vertices.first();
        let mut hashmap_vertex: HashMap<String, Vertex> = HashMap::new();
        let mut hashmap_cost: HashMap<String, Cost> = HashMap::new();
        if let Some(head) = first {
            for vertex in vertices.iter() {
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

    pub fn update_cost(&mut self, name: String, previous: String, new_value: &i32) {
        if let Some(mut cost) = self.get_cost(name.clone()) {
            cost.total = new_value.clone();
            cost.previous = previous.clone();
            self.cost.insert(name, cost);
        }
    }
}

impl Default for Cost {
    fn default() -> Self {
        Cost {
            previous: "".to_string(),
            total: INFINITY as i32
        } 
    }
}