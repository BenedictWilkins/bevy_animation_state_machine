use indexmap::map::IndexMap;
use std::any::Any;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Debug;
use std::hash::Hash;

#[derive(Debug)]
pub enum GraphError {
    NoSuchPath(String),
}

pub type Vertex = usize;

#[derive(Debug, Clone)]
pub struct Graph<S: Hash + Eq + PartialEq + Debug> {
    pub(crate) edges: HashMap<Vertex, HashSet<Vertex>>,
    pub(crate) verticies: IndexMap<S, Vertex>,
}

impl<S: Hash + Eq + PartialEq + Debug> Graph<S> {
    pub fn new() -> Self {
        Graph {
            edges: HashMap::new(),
            verticies: IndexMap::new(),
        }
    }

    pub(crate) fn from_data(
        verts: IndexMap<S, Vertex>,
        edges: HashMap<Vertex, HashSet<Vertex>>,
    ) -> Graph<S> {
        // assume that verts and edges have matching vertex data...
        return Graph {
            edges: edges,
            verticies: verts,
        };
    }

    pub fn get_vertex(&self, state: &S) -> Option<&Vertex> {
        return self.verticies.get(state);
    }

    pub fn get_state(&self, vertex: Vertex) -> Option<&S> {
        let entry = self.verticies.get_index(vertex)?;
        return Some(entry.0);
    }

    pub fn get_states(&self, path: &Vec<Vertex>) -> Vec<&S> {
        return path.iter().map(|x| self.get_state(*x).unwrap()).collect();
    }

    pub fn add_vertex(&mut self, state: S) -> Vertex {
        if !self.verticies.contains_key(&state) {
            let index = self.verticies.len();
            let (actual_index, _) = self.verticies.insert_full(state, index);
            assert!(index == actual_index); // sanity check...
            return index;
        } else {
            return *self.verticies.get(&state).unwrap();
        }
    }

    pub fn add_edge(&mut self, state1: &S, state2: &S) -> (Vertex, Vertex) {
        let v1 = *self.verticies.get(state1).unwrap();
        let v2 = *self.verticies.get(state2).unwrap();
        self.add_vertex_edge(v1, v2);
        return (v1, v2);
    }

    pub fn add_vertex_edge(&mut self, v1: Vertex, v2: Vertex) {
        self.edges.entry(v1).or_default().insert(v2);
    }

    pub fn shortest_path(&self, start: &S, end: &S) -> Result<Vec<&S>, GraphError> {
        let v1 = *self.verticies.get(start).unwrap();
        let v2 = *self.verticies.get(end).unwrap();
        let path = self._shortest_path(v1, v2)?;
        return Ok(self.get_states(&path));
    }

    pub(crate) fn _shortest_path(&self, v1: Vertex, v2: Vertex) -> Result<Vec<Vertex>, GraphError> {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let mut predecessors = HashMap::new();

        visited.insert(v1);
        queue.push_back(v1);

        while let Some(current) = queue.pop_front() {
            if current == v2 {
                let mut path = vec![v2];
                while let Some(&node) = predecessors.get(&path[path.len() - 1]) {
                    path.push(node);
                }
                path.reverse();
                return Ok(path);
            }

            if let Some(neighbors) = self.edges.get(&current) {
                for &neighbor in neighbors {
                    if !visited.contains(&neighbor) {
                        visited.insert(neighbor);
                        queue.push_back(neighbor);
                        predecessors.insert(neighbor, current);
                    }
                }
            }
        }

        return Err(GraphError::NoSuchPath(format!(
            "No path exists between states {:?} and {:?} in animation graph.",
            self.get_state(v1),
            self.get_state(v2)
        )));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graph_basic() {
        let mut graph: Graph<&str> = Graph::new();

        assert_eq!(graph.add_vertex("A"), 0);
        assert_eq!(graph.add_vertex("B"), 1);
        assert_eq!(graph.add_vertex("C"), 2);
        assert_eq!(graph.add_vertex("C"), 2);
        assert_eq!(graph.add_edge(&"A", &"B"), (0, 1));
        assert_eq!(graph.add_edge(&"B", &"C"), (1, 2));

        // Test vertex retrieval
        assert_eq!(graph.get_vertex(&"A"), Some(&0));
        assert_eq!(graph.get_vertex(&"B"), Some(&1));
        assert_eq!(graph.get_vertex(&"C"), Some(&2));

        // Test state retrieval
        assert_eq!(graph.get_state(0), Some(&"A"));
        assert_eq!(graph.get_state(1), Some(&"B"));
        assert_eq!(graph.get_state(2), Some(&"C"));

        // Test shortest_path
        let path_ab = graph.shortest_path(&"A", &"B").unwrap();
        assert_eq!(path_ab, vec![&"A", &"B"]);

        let path_ac = graph.shortest_path(&"A", &"C").unwrap();
        assert_eq!(path_ac, vec![&"A", &"B", &"C"]);

        let path_ca = graph.shortest_path(&"C", &"A");
        assert!(path_ca.is_err());
    }
}
