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
    JumpingUp,
    JumpingMax,
    JumpingDown,
    Landing,
    Rolling,
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
        segment_interval: SegmentInterval::new(0, 10),
        segment_behaviour: SegmentBehaviour::Forward,
        texture_atlas_handle: atlases.idle.clone(),
    };
    let walking_data = SegmentData {
        segment_interval: SegmentInterval::new(0, 8),
        segment_behaviour: SegmentBehaviour::Forward,
        texture_atlas_handle: atlases.walking.clone(),
    };

    let rolling_data = SegmentData {
        segment_interval: SegmentInterval::new(0, 7),
        segment_behaviour: SegmentBehaviour::Forward,
        texture_atlas_handle: atlases.rolling.clone(),
    };

    let running_data = SegmentData {
        segment_interval: SegmentInterval::new(0, 8),
        segment_behaviour: SegmentBehaviour::Forward,
        texture_atlas_handle: atlases.running.clone(),
    };
    // chunk out most of the landing frames
    let landing_data = SegmentData {
        segment_interval: SegmentInterval::new(0, 3),
        segment_behaviour: SegmentBehaviour::Forward,
        texture_atlas_handle: atlases.landing.clone(),
    };

    let jumping_up_data = SegmentData {
        segment_interval: SegmentInterval::new(0, 1),
        segment_behaviour: SegmentBehaviour::Forward,
        texture_atlas_handle: atlases.jumping.clone(),
    };

    let jumping_max_data = SegmentData {
        segment_interval: SegmentInterval::new(1, 1),
        segment_behaviour: SegmentBehaviour::Forward,
        texture_atlas_handle: atlases.jumping.clone(),
    };

    let jumping_down_data = SegmentData {
        segment_interval: SegmentInterval::new(2, 1),
        segment_behaviour: SegmentBehaviour::Forward,
        texture_atlas_handle: atlases.jumping.clone(),
    };

    let mut animation_graph = SpriteAnimationGraph::new();

    let v_idle = animation_graph.add_state(PlayerState::Idle, idle_data);
    let v_running = animation_graph.add_state(PlayerState::Running, running_data);
    let v_landing = animation_graph.add_state(PlayerState::Landing, landing_data);

    let v_jumping_up = animation_graph.add_state(PlayerState::JumpingUp, jumping_up_data);
    let v_jumping_max = animation_graph.add_state(PlayerState::JumpingMax, jumping_max_data);
    let v_jumping_down = animation_graph.add_state(PlayerState::JumpingDown, jumping_down_data);

    let v_rolling = animation_graph.add_state(PlayerState::Rolling, rolling_data);

    // idle transitions
    // a self-transition is included that specifies how to loop on the same segment.
    animation_graph.add_transition((v_idle, v_idle), TransitionData::wait());

    // running transitions
    animation_graph.add_transition((v_running, v_running), TransitionData::wait());
    animation_graph.add_transition((v_running, v_idle), TransitionData::wait_for(3));
    animation_graph.add_transition((v_idle, v_running), TransitionData::immediate_to(3));
    animation_graph.add_transition((v_idle, v_idle), TransitionData::wait());

    // jumping up transitions
    // immediately transitioning to the next state to the first frame of the segment.
    animation_graph.add_transition((v_idle, v_jumping_up), TransitionData::immediate());
    animation_graph.add_transition((v_running, v_jumping_up), TransitionData::immediate());

    animation_graph.add_transition((v_jumping_up, v_jumping_up), TransitionData::wait());
    animation_graph.add_transition((v_jumping_up, v_jumping_max), TransitionData::immediate());

    // jumping max transitions
    animation_graph.add_transition((v_jumping_max, v_jumping_max), TransitionData::wait());
    animation_graph.add_transition((v_jumping_max, v_jumping_down), TransitionData::wait());

    // jumping down transitions
    animation_graph.add_transition((v_jumping_down, v_jumping_down), TransitionData::wait());
    animation_graph.add_transition((v_jumping_down, v_rolling), TransitionData::immediate());
    animation_graph.add_transition((v_jumping_down, v_landing), TransitionData::immediate());

    // landing transitions
    animation_graph.add_transition((v_landing, v_landing), TransitionData::wait());
    animation_graph.add_transition((v_landing, v_jumping_up), TransitionData::immediate());
    animation_graph.add_transition((v_landing, v_idle), TransitionData::wait());

    animation_graph.add_transition((v_rolling, v_rolling), TransitionData::wait());
    animation_graph.add_transition((v_rolling, v_running), TransitionData::wait_to(3));

    // animation_graph.add_transition(
    //     (v_landing, v_running),
    //     TransitionData {
    //         transition_behaviour: TransitionBehaviour::Immediate,
    //         transition_to_index: 4,
    //     },
    // );
    //
    // walking transitions
    // animation_graph.add_transition((v_walking, v_walking), TransitionData::wait());
    // animation_graph.add_transition((v_walking, v_idle), TransitionData::wait());
    // animation_graph.add_transition((v_walking, v_running), TransitionData::wait());
    // animation_graph.add_transition((v_walking, v_jumping_up), TransitionData::immediate());

    // // walking transitions
    // animation_graph.add_transition((v_walking, v_idle), TransitionData::wait());
    // animation_graph.add_transition((v_walking, v_walking), TransitionData::wait());
    // animation_graph.add_transition((v_walking, v_running), TransitionData::wait());
    // animation_graph.add_transition((v_walking, v_jumping_up), TransitionData::immediate());

    // // running transitions
    // animation_graph.add_transition((v_running, v_running), TransitionData::wait());
    // animation_graph.add_transition((v_running, v_jumping_up), TransitionData::immediate());
    // animation_graph.add_transition((v_running, v_walking), TransitionData::wait());

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

    #[asset(texture_atlas(tile_size_x = 48., tile_size_y = 48., columns = 7, rows = 1,))]
    #[asset(path = "player_rolling.png")]
    pub rolling: Handle<TextureAtlas>,

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
