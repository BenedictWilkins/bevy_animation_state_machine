use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Debug;

#[derive(Debug)]
pub enum GraphError {
    NoSuchPath(String),
}

pub type Vertex = usize;
pub type Edge = (Vertex, Vertex);

#[derive(Debug, Clone)]
pub struct Graph<V, E> {
    edges: HashMap<Edge, E>,   // Metadata for edges
    verts: HashMap<Vertex, V>, // Metadata for vertices
    adjacency: HashMap<Vertex, HashSet<Vertex>>,
}

impl<V, E> Graph<V, E> {
    pub fn new() -> Self {
        Graph {
            edges: HashMap::new(),
            verts: HashMap::new(),
            adjacency: HashMap::new(),
        }
    }

    pub fn get_vertex_metadata(&self, vertex: Vertex) -> Option<&V> {
        self.verts.get(&vertex)
    }

    pub fn get_edge_metadata(&self, edge: (Vertex, Vertex)) -> Option<&E> {
        self.edges.get(&edge)
    }

    pub fn add_node(&mut self, node: Vertex, metadata: V) {
        self.verts.insert(node, metadata);
        self.adjacency.insert(node, HashSet::new());
    }

    pub fn add_edge(&mut self, edge: (Vertex, Vertex), metadata: E) {
        if !self.verts.contains_key(&edge.0) {
            panic!("Vertex {} not found", edge.0);
        }
        if !self.verts.contains_key(&edge.1) {
            panic!("Vertex {} not found", edge.1);
        }

        self.edges.insert(edge, metadata);
        if let Some(neighbors) = self.adjacency.get_mut(&edge.0) {
            neighbors.insert(edge.1);
        }
    }

    pub fn shortest_path(&self, start: Vertex, end: Vertex) -> Result<Vec<Vertex>, GraphError> {
        if !self.verts.contains_key(&start) || !self.verts.contains_key(&end) {
            return Err(GraphError::NoSuchPath(format!(
                "Either vertex {} or {} doesn't exist.",
                start, end
            )));
        }

        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let mut predecessors: HashMap<Vertex, Vertex> = HashMap::new();

        visited.insert(start);
        queue.push_back(start);

        while let Some(current) = queue.pop_front() {
            if current == end {
                let mut path = vec![end];
                while let Some(&node) = predecessors.get(&path[path.len() - 1]) {
                    path.push(node);
                }
                //path.reverse();
                return Ok(path);
            }

            if let Some(neighbors) = self.adjacency.get(&current) {
                for &neighbor in neighbors {
                    if !visited.contains(&neighbor) {
                        visited.insert(neighbor);
                        queue.push_back(neighbor);
                        predecessors.insert(neighbor, current);
                    }
                }
            }
        }

        Err(GraphError::NoSuchPath(format!(
            "No path exists between vertex {} and {}.",
            start, end
        )))
    }

    pub fn contains_vertex(&self, vertex: Vertex) -> bool {
        return self.adjacency.contains_key(&vertex);
    }

    // Add methods for querying and modifying the graph as needed.
}
