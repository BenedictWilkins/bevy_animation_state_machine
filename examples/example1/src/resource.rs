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
    let atlases = world.resource::<PlayerAtlasResource>();

    // frame_count can be used if the Atlas does not contain sprites for every tile. i.e. frame_count <= columns * rows
    let idle_data = SegmentData {
        frame_interval: (0, 9),
        segment_behaviour: SegmentBehaviour::Forward,
        texture_atlas_handle: atlases.idle.clone(),
    };
    let walking_data = SegmentData {
        frame_interval: (0, 7),
        segment_behaviour: SegmentBehaviour::Forward,
        texture_atlas_handle: atlases.walking.clone(),
    };
    let running_data = SegmentData {
        frame_interval: (0, 7),
        segment_behaviour: SegmentBehaviour::Forward,
        texture_atlas_handle: atlases.running.clone(),
    };

    let landing_data = SegmentData {
        frame_interval: (0, 8),
        segment_behaviour: SegmentBehaviour::Forward,
        texture_atlas_handle: atlases.landing.clone(),
    };

    let jumping_data = SegmentData {
        frame_interval: (0, 2),
        segment_behaviour: SegmentBehaviour::Forward,
        texture_atlas_handle: atlases.jumping.clone(),
    };

    let mut animation_graph = SpriteAnimationGraph::new();

    let v_idle = animation_graph.add_state(PlayerState::Idle, idle_data);
    let v_walking = animation_graph.add_state(PlayerState::Walking, walking_data);
    let v_running = animation_graph.add_state(PlayerState::Running, running_data);
    let v_landing = animation_graph.add_state(PlayerState::Landing, landing_data);
    let v_jumping = animation_graph.add_state(PlayerState::Jumping, jumping_data);

    // transition data defaults to immediately transitioning to the next state and jumping to the first frame of the segment.
    // this is what we want when transitioning from the idle state.
    animation_graph.add_transition((v_idle, v_jumping), TransitionData::default());
    animation_graph.add_transition((v_idle, v_walking), TransitionData::default());
    // a self-transition is included that defines how to loop on the same segment. Below will just loop normally
    // note that default should not be used here, otherwise the transition will be immediate and it will seem
    // as though the animation is not running (it will always be immediately returning to the first frame)
    animation_graph.add_transition((v_idle, v_idle), TransitionData::looping());

    animation_graph.add_transition((v_walking, v_idle), TransitionData::wait());
    animation_graph.add_transition((v_walking, v_walking), TransitionData::looping());

    animation_graph.add_transition((v_walking, v_running), TransitionData::wait());

    animation_graph.add_transition((v_running, v_running), TransitionData::looping());
    animation_graph.add_transition((v_running, v_walking), TransitionData::wait());

    world.insert_resource(animation_graph);
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
