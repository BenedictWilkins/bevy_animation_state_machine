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

#[derive(Component)]
pub struct Player;

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

fn update_player_animation(mut query: Query<&mut PlayerState>, keys: Res<Input<KeyCode>>) {
    // change the state of the player depending on user input
    for mut state in &mut query {
        if keys.just_pressed(KeyCode::Space) {
            // start jumping!
            *state = PlayerState::Jumping;
        }
        if keys.just_released(KeyCode::Space) {
            // stop jumping
        }
        if keys.just_pressed(KeyCode::D) {
            // this will take us through walking in the animation graph
            *state = PlayerState::Running;
        }
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn_player(mut commands: Commands, animation_data: Res<PlayerAnimationGraphResource>) {
    // display sprite to be used. This will be updated automatically as the animation state changes.
    let sprite_bundle = SpriteSheetBundle {
        sprite: TextureAtlasSprite {
            custom_size: { Some(Vec2 { x: 128., y: 128. }) },
            ..default()
        },
        texture_atlas: animation_data.animation_graph.get_atlas().clone(),
        ..default()
    };

    let animation = SpriteAnimation {
        timer: Timer::from_seconds(1. / 20., TimerMode::Once),
        animation_graph: animation_data.animation_graph.copy_from_resource(),
    };
    commands.spawn((
        Player,
        animation_data.animation_graph.state,
        sprite_bundle,
        animation,
    ));
}
