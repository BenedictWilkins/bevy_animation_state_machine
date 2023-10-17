use std::ops::Deref;

use bevy::prelude::*;

use crate::animation_graph::SpriteAnimationGraph;

use crate::graph::Vertex;
use crate::SpriteAnimationState;

#[derive(Component)]
pub struct SpriteAnimation {
    pub timer: Timer,
    /// the vertex of the current animation state
    pub(crate) current_vertex: Vertex,
    /// the path to the currently desired vertex. This will be updated whenever the state changes.
    /// note that this is in reverse order, the desired vertex is at position 0 (the path is a stack).
    /// The last element in the path is the next vertex to visit.
    pub(crate) desired_path: Vec<Vertex>,
}

impl SpriteAnimation {
    pub fn new(initial_vertex: Vertex, frame_duration: f32) -> Self {
        return SpriteAnimation {
            current_vertex: initial_vertex,
            desired_path: vec![initial_vertex],
            timer: Timer::from_seconds(frame_duration, TimerMode::Repeating),
        };
    }

    pub fn next_vertex(&self) -> Vertex {
        return self.desired_path[self.desired_path.len() - 1];
    }

    pub fn transition(&mut self) {
        if self.desired_path.len() > 1 {
            self.current_vertex = self.desired_path.pop().unwrap();
        } else {
            // we are the the desired node, no change needs to be made
            self.current_vertex = self.desired_path[0];
        }
    }
}

#[derive(Bundle)]
pub struct SpriteAnimationBundle<S: SpriteAnimationState> {
    pub state: S,
    pub animation: SpriteAnimation,
    #[bundle()]
    pub sprite_bundle: SpriteSheetBundle,
}
