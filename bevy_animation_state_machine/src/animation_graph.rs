use core::panic;
use std::collections::HashMap;
use std::default;

use bevy::prelude::*;
use indexmap::IndexMap;

use std::sync::atomic::{AtomicUsize, Ordering};

use crate::graph::{Edge, Graph, Vertex};
use crate::SpriteAnimationState;
use std::hash::{Hash, Hasher};

#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub enum TransitionBehaviour {
    Wait(usize), // transition to the next animation state after the current segment has reached a given frame (defaults to last frame).
    #[default]
    Immediate, // transition immediately to the next animation state without waiting for the current segment to finish.
}

#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub enum SegmentBehaviour {
    #[default]
    Forward,
    Backward,
}

#[derive(Debug, Clone)]
pub struct SegmentInterval {
    pub start: usize,
    pub length: usize,
}

impl SegmentInterval {
    pub fn new(start: usize, length: usize) -> Self {
        return Self {
            start: start,
            length: length,
        };
    }
}

/// a segment is a single "state" in the animation graph.
#[derive(Debug, Clone)]
pub struct SegmentData {
    pub segment_behaviour: SegmentBehaviour,
    // sprite scaling to apply to this segment. This can be used to "flip" a sprite if negative values are provided.
    // TODO pub scale: Vec2,
    pub segment_interval: SegmentInterval,
    pub texture_atlas_handle: Handle<TextureAtlas>,
}

#[derive(Debug, Clone)]
pub struct TransitionData {
    /// transition behaviour
    pub transition_behaviour: TransitionBehaviour,
    /// the starting frame index of the next segment
    pub transition_to_frame: usize,
}

impl TransitionData {
    /// Transitions will wait for the current segment to finish before continuing to the next segment.
    /// This may be used for looping a segment.
    pub fn wait() -> Self {
        TransitionData::wait_for_then_to(usize::MAX, 0)
    }

    /// Transitions will wait for a specific [`frame`] in the current segment before continuing to the start of the next segment.
    pub fn wait_for(frame: usize) -> Self {
        TransitionData::wait_for_then_to(frame, 0)
    }

    /// Transitions will wait for the current segment to finish before continuing to the given [`frame`] in the next segment.
    pub fn wait_to(frame: usize) -> Self {
        TransitionData::wait_for_then_to(usize::MAX, frame)
    }

    /// Transitions will wait for a specific [`wait_frame`] in the current segment before continuing to the given [`to_frame`] in the next segment.
    pub fn wait_for_then_to(wait_frame: usize, to_frame: usize) -> Self {
        Self {
            transition_behaviour: TransitionBehaviour::Wait(wait_frame),
            transition_to_frame: to_frame,
        }
    }

    /// Transition immediately (regardless of the current frame) to the start of the next segment.
    pub fn immediate() -> Self {
        return TransitionData::immediate_to(0);
    }

    /// Transition immediately (regardless of the current frame) to the given [`frame`] in the next segment.
    pub fn immediate_to(frame: usize) -> Self {
        Self {
            transition_behaviour: TransitionBehaviour::Immediate,
            transition_to_frame: frame,
        }
    }
}

impl Default for TransitionData {
    fn default() -> Self {
        Self {
            transition_behaviour: TransitionBehaviour::default(),
            transition_to_frame: 0,
        }
    }
}

#[derive(Resource, Debug)]
pub struct SpriteAnimationGraph<S: SpriteAnimationState> {
    animation_graph: Graph<SegmentData, TransitionData>,
    animation_states: IndexMap<S, Vertex>,
    // this is used to cache shortest path computations
    // _desired_vertex_cache: AtomicUsize,
    // _next_vertex_cache: AtomicUsize,
}
impl<S: SpriteAnimationState> SpriteAnimationGraph<S> {
    pub fn new() -> SpriteAnimationGraph<S> {
        return SpriteAnimationGraph {
            animation_graph: Graph::new(),
            animation_states: IndexMap::new(),
        };
    }

    pub fn add_state(&mut self, state: S, segment_data: SegmentData) -> Vertex {
        let vertex: Vertex = self.animation_states.len();
        self.animation_graph.add_node(vertex, segment_data);
        self.animation_states.insert(state, vertex);
        return vertex;
    }

    pub fn add_transition(&mut self, edge: Edge, transition_data: TransitionData) -> Edge {
        if self.animation_graph.contains_vertex(edge.0)
            && self.animation_graph.contains_vertex(edge.1)
        {
            if edge.0 == edge.1
                && transition_data.transition_behaviour == TransitionBehaviour::Immediate
            {
                panic!(
                    "{:?} cannot be used in a self-transition {:?}.",
                    TransitionBehaviour::Immediate,
                    edge
                )
            }
            // check that the transition_to_index is value
            let vertex1_segment_length = self
                .animation_graph
                .get_vertex_metadata(edge.1)
                .unwrap()
                .segment_interval
                .length;
            if transition_data.transition_to_frame >= vertex1_segment_length {
                panic!("Invalid [`transition_to_index`] for edge {:?}, {:?} segment is not long enough ({:?}).", (self.get_state(edge.0), self.get_state(edge.1)),  self.get_state(edge.1), vertex1_segment_length );
            }
            // everything checked out fine, add the edge.
            self.animation_graph.add_edge(edge, transition_data);
            return edge;
        } else {
            let no_vertex = if self.animation_graph.contains_vertex(edge.0) {
                edge.1
            } else {
                edge.0
            };
            panic!("Failed to add transition {:?} to animation graph, vertex {:?} doesn't exist, did you add the corresponding state with [`add_state`]?", edge, no_vertex);
        }
    }

    pub fn get_vertex(&self, state: &S) -> Vertex {
        if let Some(result) = self.animation_states.get(state) {
            return *result;
        }
        panic!(
            "Animation state {:?} was not part of the animation graph.",
            state
        );
    }

    pub fn get_state(&self, vertex: Vertex) -> &S {
        if let Some(result) = self.animation_states.get_index(vertex) {
            return result.0;
        }
        panic!("Failed to find animation state for vertex {:?}.", vertex);
    }

    pub fn get_atlas(&self, vertex: Vertex) -> &Handle<TextureAtlas> {
        return &self
            .animation_graph
            .get_vertex_metadata(vertex)
            .unwrap()
            .texture_atlas_handle;
    }

    pub fn get_segment_data(&self, vertex: Vertex) -> &SegmentData {
        return &self.animation_graph.get_vertex_metadata(vertex).unwrap();
    }

    pub fn get_transition_data(&self, edge: Edge) -> &TransitionData {
        return match self.animation_graph.get_edge_metadata(edge) {
            Option::Some(data) => data,
            Option::None => panic!(
                "Missing data for transition ({:?},{:?}",
                self.get_state(edge.0),
                self.get_state(edge.1)
            ),
        };
    }

    pub fn shortest_path(&self, current: Vertex, desired: Vertex) -> Vec<Vertex> {
        return match self.animation_graph.shortest_path(current, desired) {
            Ok(path) => path,
            Err(_) => panic!(
                "No path exists between animation states {:?} and {:?}",
                self.get_state(current),
                self.get_state(desired)
            ),
        };
    }

    // pub fn get_transition_data(&self) -> &TransitionData {
    //     return &self
    //         .animation_graph
    //         .get_edge_metadata(self.current_vertex)
    //         .unwrap();
    // }
    // pub fn get_frame_interval(&self) -> (usize, usize) {
    //     return &self
    //         .animation_graph
    //         .get_vertex_metadata(self.current_vertex)
    //         .unwrap()
    //         .frame_interval;
    // }

    // pub fn is_done(&self, frame: usize) -> bool {
    //     return frame >= self.data[self.get_vertex(&self.state)].frame_count;
    // }

    // pub fn get_frame_count(&self) -> usize {
    //     return self.data[self.get_vertex(&self.state)].frame_count;
    // }

    // // pub fn get_loop_index(&self) -> usize {
    // //     let loop_behaviour = self.data[self.get_vertex(&self.state)].loop_behaviour;
    // //     return match loop_behaviour {
    // //         LoopBehaviour::Stop => self.get_frame_count() - 1,
    // //         LoopBehaviour::Repeat => 0,
    // //     };
    // // }

    // pub fn get_next_state(&self, desired: &S) -> &S {
    //     let desired_vertex = self.get_vertex(desired);

    //     if self.update_desired_cache(&desired_vertex) {
    //         // the desired vertex was updated, this means we need to recompute the shortest path.
    //         // this also needs to happen if the underlying graph changes!
    //         let result = self
    //             .animation_graph
    //             ._shortest_path(*self.get_vertex(&self.state), *desired_vertex);
    //         match result {
    //             Ok(path) => {
    //                 let next_vertex = path[1];
    //                 self._next_vertex_cache.swap(next_vertex, Ordering::Relaxed);
    //             }
    //             Err(error) => panic!("Error: {:?}", error),
    //         }
    //     }
    //     return self
    //         .animation_graph
    //         .get_state(self._next_vertex_cache.load(Ordering::Relaxed))
    //         .unwrap();
    // }

    // /// this has a side effect of updateing the desired vertex cache!
    // fn update_desired_cache(&self, desired_vertex: &Vertex) -> bool {
    //     fn _desired_cache(desired: &Vertex, desired_cache: &Vertex) -> Option<Vertex> {
    //         if desired == desired_cache {
    //             return None; // dont update the cache, no need to recompute anything
    //         } else {
    //             return Some(*desired);
    //         }
    //     }
    //     return self
    //         ._desired_vertex_cache
    //         .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |x| {
    //             _desired_cache(desired_vertex, &x)
    //         })
    //         .is_ok();
    // }

    // /// this should only be done if copying from a bevy Resource onto a specific bevy component.
    // pub fn copy_from_resource(&self) -> Self {
    //     let _desired_vertex_cache = self._desired_vertex_cache.load(Ordering::Relaxed);
    //     let _next_vertex_cache = self._next_vertex_cache.load(Ordering::Relaxed);
    //     // TODO give a better error message
    //     assert!(_desired_vertex_cache == usize::MAX); // you are trying to copy this after use, bad!
    //     assert!(_next_vertex_cache == usize::MAX); // you are trying to copy this after use, bad!
    //     Self {
    //         state: self.state.clone(),
    //         animation_graph: self.animation_graph.clone(),
    //         data: self.data.clone(),
    //         _desired_vertex_cache: AtomicUsize::new(usize::MAX),
    //         _next_vertex_cache: AtomicUsize::new(usize::MAX),
    //     }
    //}
}
