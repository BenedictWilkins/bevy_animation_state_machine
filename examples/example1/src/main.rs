use bevy::prelude::*;
use bevy_animation_state_machine::*;
use bevy_asset_loader::prelude::*;
use resource::*;

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    #[default]
    Loading,
    Running,
}

mod resource;

#[derive(Component, Debug)]
pub struct Player {
    speed: f32, // used to determine when to play jumping/landing animations.
}

impl Default for Player {
    fn default() -> Self {
        Self { speed: 200.0 }
    }
}

impl Player {
    const MAX_HEIGHT: f32 = 100.; // max jump height
    const MIN_HEIGHT: f32 = 40.; // min jump level
    const GROUND_LEVEL: f32 = 0.;
}

fn main() {
    App::new()
        .add_state::<GameState>()
        //.add_plugins(DefaultPlugins)
        // for pixel art
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        //.add_collection_to_loading_state(LoadingState::new(GameState::Loading))
        .add_loading_state(
            LoadingState::new(GameState::Loading).continue_to_state(GameState::Running),
        )
        .add_collection_to_loading_state::<_, PlayerAtlasResource>(GameState::Loading)
        .add_systems(OnExit(GameState::Loading), create_player_animation_graph)
        .add_systems(OnEnter(GameState::Running), spawn_camera)
        .add_systems(OnEnter(GameState::Running), spawn_player)
        .add_systems(Update, bevy::window::close_on_esc)
        .add_systems(
            Update,
            (animation::<PlayerState>, update_player_animation)
                .run_if(in_state(GameState::Running)),
        )
        .run();
}

fn update_player_animation(
    mut query: Query<(&mut PlayerState, &Player, &mut Transform)>,
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    // change the state of the player depending on user input
    for (mut state, player, mut transform) in &mut query {
        //println!("{:?}", state);
        // if keys.pressed(KeyCode::W) && *state != PlayerState::JumpingDown {
        //     // start jumping!
        //     *state = PlayerState::JumpingUp;
        // }
        // if !keys.pressed(KeyCode::W)
        //     && *state == PlayerState::JumpingUp
        //     && transform.translation[1] > Player::MIN_HEIGHT
        // {
        //     // stop jumping
        //     *state = PlayerState::JumpingDown;
        // }
        //println!("{:?}", state);
        if *state == PlayerState::Idle {
            // transitions from idle -> JumpingUp, Running
            if keys.pressed(KeyCode::W) {
                *state = PlayerState::JumpingUp;
            } else if keys.pressed(KeyCode::D) {
                *state = PlayerState::Running;
            } else {
                *state = PlayerState::Idle;
            }
        } else if *state == PlayerState::JumpingUp {
            // transitions from JumpingUp -> JumpingMax, JumpingDown
            // update player position
            transform.translation[1] += player.speed * time.delta_seconds();

            if keys.pressed(KeyCode::W) && transform.translation[1] < Player::MAX_HEIGHT {
                *state = PlayerState::JumpingUp; // still jumping up
            } else if keys.pressed(KeyCode::W) && transform.translation[1] > Player::MAX_HEIGHT {
                transform.translation[1] = Player::MAX_HEIGHT;
                *state = PlayerState::JumpingDown; // this will go via JumpingMax in the animation graph.
            } else if !keys.pressed(KeyCode::W) && transform.translation[1] > Player::MIN_HEIGHT {
                // player is no longer jumping and has gone passed the min height
                *state = PlayerState::JumpingDown;
            } else {
                // player is no longer jumping but has not gone passed the min height yet, keep going up!
                *state = PlayerState::JumpingUp;
            }
        } else if *state == PlayerState::JumpingDown {
            transform.translation[1] -= 1.5 * player.speed * time.delta_seconds();

            if transform.translation[1] > Player::GROUND_LEVEL {
                *state = PlayerState::JumpingDown;
            } else {
                transform.translation[1] = Player::GROUND_LEVEL;
                // player is at ground level, decide what to do based on current input
                if keys.pressed(KeyCode::W) {
                    *state = PlayerState::JumpingUp;
                } else if keys.pressed(KeyCode::D) {
                    *state = PlayerState::Rolling;
                } else {
                    *state = PlayerState::Landing;
                }
            }
        } else if *state == PlayerState::Running {
            if keys.pressed(KeyCode::W) {
                *state = PlayerState::JumpingUp;
            } else if keys.pressed(KeyCode::D) {
                *state = PlayerState::Running; // keep running
            } else {
                *state = PlayerState::Idle;
            }
        } else if *state == PlayerState::Rolling {
            *state = PlayerState::Running; // keep running
        } else if *state == PlayerState::Landing {
            // the player is currently landing, decide what to do based on current input
            if keys.pressed(KeyCode::W) {
                *state = PlayerState::JumpingUp;
            } else {
                *state = PlayerState::Idle;
            }
        }
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn_player(mut commands: Commands, animation_graph: Res<SpriteAnimationGraph<PlayerState>>) {
    // display sprite to be used. This will be updated automatically as the animation state changes.
    let initial_vertex = animation_graph.get_vertex(&PlayerState::Idle);
    let animation = SpriteAnimation::new(initial_vertex, 1. / 8.);

    let sprite_bundle = SpriteSheetBundle {
        sprite: TextureAtlasSprite {
            custom_size: { Some(Vec2 { x: 128., y: 128. }) },
            ..default()
        },
        texture_atlas: animation_graph.get_atlas(initial_vertex).clone(),
        ..default()
    };

    let animation_bundle = SpriteAnimationBundle {
        animation: animation,
        state: animation_graph.get_state(initial_vertex).clone(),
        sprite_bundle: sprite_bundle,
    };

    commands.spawn((Player::default(), animation_bundle));
}
