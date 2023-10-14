use bevy::prelude::*;

use crate::animation_graph::{SpriteAnimationGraph, SpriteAnimationNode};

use crate::graph::Graph;
use crate::SpriteAnimationState;

#[derive(Component)]
pub struct SpriteAnimation<S: SpriteAnimationState> {
    pub timer: Timer,
    // there are no borrowed references inside Animation graph so 'static should be fine here. probably... what about the life time of AnimationState objects?
    pub animation_graph: SpriteAnimationGraph<S>,
}

impl<S: SpriteAnimationState> SpriteAnimation<S> {
    // pub fn new(
    //     initial_state: S,
    //     graph: Graph<SpriteAnimationNode<S>>,
    //     frame_duration: f32,
    // ) -> Self {
    //     return SpriteAnimation {
    //         timer: Timer::from_seconds(frame_duration, TimerMode::Once), // this will be reset manually
    //         animation_graph: SpriteAnimationGraph::new(initial_state, graph),
    //     };
    // }
}
