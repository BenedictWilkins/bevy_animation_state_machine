mod animation_graph;
mod component;
mod graph;
mod system;

mod graph2;

pub use animation_graph::{LoopBehaviour, SpriteAnimationGraph, SpriteAnimationNode};
pub use component::SpriteAnimation;
pub use graph::Graph;

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
