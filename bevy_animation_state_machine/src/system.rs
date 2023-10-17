use bevy::prelude::*;

use crate::component::SpriteAnimation;
use crate::{SpriteAnimationGraph, SpriteAnimationState, TransitionBehaviour};

pub fn animation<S: SpriteAnimationState>(
    mut query: Query<(
        &S,
        &mut SpriteAnimation,
        &mut TextureAtlasSprite,
        &mut Handle<TextureAtlas>,
    )>,
    time: Res<Time>,
    animation_graph: Res<SpriteAnimationGraph<S>>,
) {
    for (state, mut animation, mut sprite, mut atlas) in &mut query {
        // TODO rather than always querying with sprite/ atlas as mutable, better may be to use run critera and a system
        // that only mutates them if `animation` requires it. This will free up other system on sprite and atlas to run async.

        let desired_vertex = animation_graph.get_vertex(state);
        // check if the desired vertex changed, update the path to it if so
        // note that the path is in reverse order! desired is at element 0
        if animation.desired_path[0] != desired_vertex {
            animation.desired_path =
                animation_graph.shortest_path(animation.current_vertex, desired_vertex);
            animation.transition(); // this has no effect on current_vertex, but prepares the path stack for later use.
        }

        let mut next_vertex = animation.next_vertex();

        let mut segment_data = animation_graph.get_segment_data(animation.current_vertex);
        let mut transition_data =
            animation_graph.get_transition_data((animation.current_vertex, next_vertex));

        println!(
            "{:?} {:?} {:?}",
            animation_graph.get_state(desired_vertex),
            animation_graph.get_state(animation.current_vertex),
            animation_graph.get_state(next_vertex),
        );

        // check whether to immediately transition to the next animation state.
        while transition_data.transition_behaviour == TransitionBehaviour::Immediate {
            assert!(animation.current_vertex != next_vertex); // this should not be possible due to checks when defining the animation graph.

            // transition immediately!
            animation.transition();

            // TODO these will be done in a seperate system whose run critera is a change in animation vertex.
            *atlas = animation_graph.get_atlas(animation.current_vertex).clone();
            sprite.index =
                segment_data.segment_interval.start + transition_data.transition_to_frame;

            next_vertex = animation.next_vertex();
            segment_data = animation_graph.get_segment_data(animation.current_vertex);
            transition_data =
                animation_graph.get_transition_data((animation.current_vertex, next_vertex));

            println!(
                "immediate {:?} {:?} {:?}",
                animation_graph.get_state(desired_vertex),
                animation_graph.get_state(animation.current_vertex),
                animation_graph.get_state(next_vertex),
            );
        }

        // otherwise, we are waiting for some frames to finish
        if let TransitionBehaviour::Wait(wait_index) = transition_data.transition_behaviour {
            animation.timer.tick(time.delta());
            if animation.timer.just_finished() {
                let segment_start_index = segment_data.segment_interval.start;
                let segment_length = segment_data.segment_interval.length;
                // use the waiting index to determine whether the frame has finished. If the waiting index >= the segment length then take the last frame as the one to wait for.
                let finished_index = segment_start_index + (segment_length - 1).min(wait_index);

                if sprite.index != finished_index {
                    sprite.index = segment_start_index
                        + (sprite.index + 1 - segment_start_index) % segment_length;
                } else {
                    // this segment has finished, transition to the next vertex on the path to desired_vertex
                    animation.transition();
                    // TODO these will be done in a seperate system whose run critera is a change in animation vertex.
                    segment_data = animation_graph.get_segment_data(animation.current_vertex); // get new segment data
                    *atlas = segment_data.texture_atlas_handle.clone();
                    // println!(
                    //     "transition to {:?}{:?} {:?}",
                    //     animation_graph.get_state(animation.current_vertex),
                    //     segment_data.segment_interval.start,
                    //     transition_data.transition_to_frame
                    // );
                    sprite.index =
                        segment_data.segment_interval.start + transition_data.transition_to_frame;
                }
            }
        }
    }
}
