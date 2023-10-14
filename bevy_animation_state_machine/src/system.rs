use bevy::prelude::*;

use crate::component::SpriteAnimation;
use crate::SpriteAnimationState;

pub fn animation<S: SpriteAnimationState>(
    mut query: Query<(
        &S,
        &mut SpriteAnimation<S>,
        &mut TextureAtlasSprite,
        &mut Handle<TextureAtlas>,
    )>,
    time: Res<Time>,
) {
    for (state, mut animation, mut sprite, mut atlas) in &mut query {
        println!("animate {:?} {:?}", state, sprite.index);
        animation.timer.tick(time.delta());
        if animation.timer.just_finished() {
            animation.timer.reset();
            // next frame, or transition

            sprite.index = sprite.index + 1;
            if animation.animation_graph.is_done(sprite.index) {
                println!("animation done");
                if *state == animation.animation_graph.state {
                    sprite.index = 0; //animation.animation_graph.get_loop_index();
                } else if *state != animation.animation_graph.state {
                    sprite.index = 0; // TODO this should be set based on the next animation

                    // a new state! compute path to new state
                    animation.animation_graph.state = *state;
                    *atlas = animation.animation_graph.get_atlas().clone();
                }
                // the current state has finished animating, trigger a transition
                // if *state == animation.animation_graph.state {
                //     // restart the current animation.
                //     sprite.index = 0;
                // } else {
                //     // a new state! compute path to new state
                //     animation.animation_graph.state = *state;
                //     //*atlas = animation.animation_graph.get_atlas().clone(); // set the new atlas
                // }
            }
        }
    }
}
