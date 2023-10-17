mod animation_graph;
mod component;
mod graph;
mod system;

pub use animation_graph::{
    SegmentBehaviour, SegmentData, SegmentInterval, SpriteAnimationGraph, TransitionBehaviour,
    TransitionData,
};
pub use component::{SpriteAnimation, SpriteAnimationBundle};

pub use system::animation;

pub trait SpriteAnimationState:
    Eq + PartialEq + std::hash::Hash + Copy + std::fmt::Debug + bevy::prelude::Component
{
}

// Provide a blanket implementation for any type `T` that satisfies the required trait bounds.
impl<T: Eq + PartialEq + std::hash::Hash + Copy + std::fmt::Debug + bevy::prelude::Component>
    SpriteAnimationState for T
{
}
