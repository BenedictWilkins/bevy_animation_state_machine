use bevy::prelude::*;
use bevy_animation_state_machine::*;
use bevy_asset_loader::prelude::*;

/// the possible states that the player can be in, these are used by the animation but might be used more generally to determine player system logic.
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, Default, Component)]
pub enum PlayerState {
    #[default]
    Idle,
    Walking,
    Running,
    Jumping,
    Landing,
    // Shooting,
    // Stabbing,
}

/// this is used to insert the animation graph. It is an exclusiive system that runs when the game enters the GameState::Running state.
/// In fact, it only need run once, but must do so AFTER the PlayerAtlasResource has been loaded.
/// In later versions of this package we will create a macro that should handle most of what is in this file.
pub fn create_player_animation_graph(world: &mut World) {
    world.insert_resource(PlayerAnimationGraphResource::from_world(world));
}

#[derive(Resource, Debug)]
pub struct PlayerAnimationGraphResource {
    // idle: SpriteAnimationNode<PlayerState>,
    // blinking: SpriteAnimationNode<PlayerState>,
    // head_turn: SpriteAnimationNode<PlayerState>,
    // walking: SpriteAnimationNode<PlayerState>,
    // running: SpriteAnimationNode<PlayerState>,
    // shooting: SpriteAnimationNode<PlayerState>,
    // stabbing: SpriteAnimationNode<PlayerState>,
    pub animation_graph: SpriteAnimationGraph<PlayerState>,
}

impl PlayerAnimationGraphResource {
    // this is not the same as impl FromWorld, we dont need a mutable ref to World.
    // Because this depends on PlayerAtlasResource which is loaded by bevy_assert_loader,
    // this should be called when the game enters GameState::Running (or exits GameState::Loading) to ensure PlayerAtlasResource exists.
    // See [`create_player_animation_graph`] above.
    fn from_world(world: &World) -> Self {
        let atlases = world.resource::<PlayerAtlasResource>();

        // frame_count can be used if the Atlas does not contain sprites for every tile. i.e. frame_count <= columns * rows
        let idle = SpriteAnimationNode {
            state: PlayerState::Idle,
            loop_behaviour: LoopBehaviour::Repeat,
            frame_count: 10,
            atlas: atlases.idle.clone(),
        };
        let walking = SpriteAnimationNode {
            state: PlayerState::Walking,
            loop_behaviour: LoopBehaviour::Repeat,
            frame_count: 8,
            atlas: atlases.walking.clone(),
        };
        let running = SpriteAnimationNode {
            state: PlayerState::Running,
            loop_behaviour: LoopBehaviour::Repeat,
            frame_count: 8,
            atlas: atlases.running.clone(),
        };
        let landing = SpriteAnimationNode {
            state: PlayerState::Landing,
            loop_behaviour: LoopBehaviour::Repeat,
            frame_count: 9,
            atlas: atlases.landing.clone(),
        };
        let jumping = SpriteAnimationNode {
            state: PlayerState::Jumping,
            loop_behaviour: LoopBehaviour::Stop, // this means the animation will stop at the last frame.
            frame_count: 3,
            atlas: atlases.jumping.clone(),
        };

        let mut animation_graph = SpriteAnimationGraph::new(&idle.state);

        let v_idle = animation_graph.add_animation_node(idle);
        let v_walking = animation_graph.add_animation_node(walking);
        let v_running = animation_graph.add_animation_node(running);
        let v_landing = animation_graph.add_animation_node(landing);
        let v_jumping = animation_graph.add_animation_node(jumping);

        // animation_graph.add_edge(v_idle, v_blinking);
        // animation_graph.add_edge(v_blinking, v_idle);
        // animation_graph.add_edge(v_idle, v_head_turn);
        // animation_graph.add_edge(v_head_turn, v_idle);

        // TODO lets do the others

        return PlayerAnimationGraphResource {
            animation_graph: animation_graph,
        };
    }
}

/// this atlas resource will be used to intialise the player animation graph
#[derive(AssetCollection, Resource)]
pub struct PlayerAtlasResource {
    #[asset(texture_atlas(tile_size_x = 48., tile_size_y = 48., columns = 10, rows = 1,))]
    #[asset(path = "player_idle.png")]
    pub idle: Handle<TextureAtlas>,

    #[asset(texture_atlas(tile_size_x = 48., tile_size_y = 48., columns = 8, rows = 1,))]
    #[asset(path = "player_walking.png")]
    pub walking: Handle<TextureAtlas>,

    #[asset(texture_atlas(tile_size_x = 48., tile_size_y = 48., columns = 8, rows = 1,))]
    #[asset(path = "player_running.png")]
    pub running: Handle<TextureAtlas>,

    #[asset(texture_atlas(tile_size_x = 48., tile_size_y = 48., columns = 3, rows = 1,))]
    #[asset(path = "player_jumping.png")]
    pub jumping: Handle<TextureAtlas>,

    #[asset(texture_atlas(tile_size_x = 48., tile_size_y = 48., columns = 9, rows = 1,))]
    #[asset(path = "player_landing.png")]
    pub landing: Handle<TextureAtlas>,
}
