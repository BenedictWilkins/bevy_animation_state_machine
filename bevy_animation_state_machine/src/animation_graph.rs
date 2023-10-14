use std::collections::HashMap;

use bevy::prelude::*;

use std::sync::atomic::{AtomicUsize, Ordering};

use crate::graph::{Graph, Vertex};
use crate::SpriteAnimationState;
use std::hash::{Hash, Hasher};

#[derive(Debug, Copy, Clone)]
pub enum LoopBehaviour {
    Stop,
    Repeat,
}

// internal struct to keep track of graph nodes, avoids a copy of state
#[derive(Debug, Clone)]
pub(crate) struct SpriteAnimationData {
    pub frame_count: usize,
    pub atlas: Handle<TextureAtlas>,
}

#[derive(Debug)]
pub struct SpriteAnimationNode<S: SpriteAnimationState> {
    pub state: S,
    pub frame_count: usize,
    pub atlas: Handle<TextureAtlas>,
}

impl<S: SpriteAnimationState> PartialEq for SpriteAnimationNode<S> {
    fn eq(&self, other: &Self) -> bool {
        self.state == other.state
    }
}

impl<S: SpriteAnimationState> Eq for SpriteAnimationNode<S> {}

impl<S: SpriteAnimationState> Hash for SpriteAnimationNode<S> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state.hash(state);
    }
}
#[derive(Component, Debug)]
pub struct SpriteAnimationGraph<S: SpriteAnimationState> {
    pub state: S,
    graph: Graph<S>,
    data: HashMap<Vertex, SpriteAnimationData>,

    // this is used to cache shortest path computations
    _desired_vertex_cache: AtomicUsize,
    _next_vertex_cache: AtomicUsize,
}
impl<'a, S: SpriteAnimationState> SpriteAnimationGraph<S> {
    pub fn add_animation_node(&mut self, node: SpriteAnimationNode<S>) -> Vertex {
        let vertex = self.graph.add_vertex(node.state);
        self.data.insert(
            vertex,
            SpriteAnimationData {
                frame_count: node.frame_count,
                atlas: node.atlas,
            },
        );
        return vertex;
    }

    pub fn add_edge(&mut self, vertex1: Vertex, vertex2: Vertex) {
        self.graph.add_vertex_edge(vertex1, vertex2);
    }

    pub fn new(initial_state: &S) -> SpriteAnimationGraph<S> {
        return SpriteAnimationGraph {
            state: initial_state.clone(),
            graph: Graph::new(),
            data: HashMap::new(),
            _desired_vertex_cache: AtomicUsize::new(usize::MAX),
            _next_vertex_cache: AtomicUsize::new(usize::MAX),
        };
    }

    // fn new(initial_state: S, graph: Graph<SpriteAnimationNode<S>>) -> Self {
    //     // create state graph from data
    //     let mut data: HashMap<Vertex, SpriteAnimationData> = HashMap::new();
    //     let mut verts: IndexMap<S, Vertex> = IndexMap::new();
    //     // unpack given data, take ownership of it.
    //     for (node, vertex) in graph.verticies {
    //         data.insert(
    //             vertex,
    //             SpriteAnimationData {
    //                 frame_count: node.frame_count,
    //                 atlas: node.atlas,
    //             },
    //         );
    //         verts.insert(node.state, vertex);
    //     }
    //     // usize:MAX  will never be used as a graph Vertex...? this is important to ensure that the cache works correctly, usize::MAX is a flag "not set".
    //     assert!(!data.contains_key(&usize::MAX));
    //     let animation_graph = Graph::from_data(verts, graph.edges);
    //     let animation_graph = SpriteAnimationGraph {
    //         state: initial_state,
    //         graph: animation_graph,
    //         data: data,
    //         _desired_vertex_cache: AtomicUsize::new(usize::MAX),
    //         _next_vertex_cache: AtomicUsize::new(usize::MAX),
    //     };

    //     return animation_graph;
    // }

    #[inline(always)]
    fn get_vertex(&self, state: &S) -> &Vertex {
        return self.graph.get_vertex(state).unwrap();
    }

    pub fn is_done(&self, frame: usize) -> bool {
        return frame >= self.data[self.get_vertex(&self.state)].frame_count;
    }

    pub fn get_atlas(&self) -> &Handle<TextureAtlas> {
        return &self.data[self.get_vertex(&self.state)].atlas;
    }

    pub fn get_frame_count(&self) -> usize {
        return self.data[self.get_vertex(&self.state)].frame_count;
    }

    // pub fn get_loop_index(&self) -> usize {
    //     let loop_behaviour = self.data[self.get_vertex(&self.state)].loop_behaviour;
    //     return match loop_behaviour {
    //         LoopBehaviour::Stop => self.get_frame_count() - 1,
    //         LoopBehaviour::Repeat => 0,
    //     };
    // }

    pub fn get_next_state(&self, desired: &S) -> &S {
        let desired_vertex = self.get_vertex(desired);

        if self.update_desired_cache(&desired_vertex) {
            // the desired vertex was updated, this means we need to recompute the shortest path.
            // this also needs to happen if the underlying graph changes!
            let result = self
                .graph
                ._shortest_path(*self.get_vertex(&self.state), *desired_vertex);
            match result {
                Ok(path) => {
                    let next_vertex = path[1];
                    self._next_vertex_cache.swap(next_vertex, Ordering::Relaxed);
                }
                Err(error) => panic!("Error: {:?}", error),
            }
        }
        return self
            .graph
            .get_state(self._next_vertex_cache.load(Ordering::Relaxed))
            .unwrap();
    }

    /// this has a side effect of updateing the desired vertex cache!
    fn update_desired_cache(&self, desired_vertex: &Vertex) -> bool {
        fn _desired_cache(desired: &Vertex, desired_cache: &Vertex) -> Option<Vertex> {
            if desired == desired_cache {
                return None; // dont update the cache, no need to recompute anything
            } else {
                return Some(*desired);
            }
        }
        return self
            ._desired_vertex_cache
            .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |x| {
                _desired_cache(desired_vertex, &x)
            })
            .is_ok();
    }

    /// this should only be done if copying from a bevy Resource onto a specific bevy component.
    pub fn copy_from_resource(&self) -> Self {
        let _desired_vertex_cache = self._desired_vertex_cache.load(Ordering::Relaxed);
        let _next_vertex_cache = self._next_vertex_cache.load(Ordering::Relaxed);
        // TODO give a better error message
        assert!(_desired_vertex_cache == usize::MAX); // you are trying to copy this after use, bad!
        assert!(_next_vertex_cache == usize::MAX); // you are trying to copy this after use, bad!
        Self {
            state: self.state.clone(),
            graph: self.graph.clone(),
            data: self.data.clone(),
            _desired_vertex_cache: AtomicUsize::new(usize::MAX),
            _next_vertex_cache: AtomicUsize::new(usize::MAX),
        }
    }
}
