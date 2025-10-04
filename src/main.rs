use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<GameState>()
        .add_systems(Startup, setup)
        .add_systems(OnEnter(GameState::Loading), setup_loading_screen)
        .add_systems(Update, loading_system.run_if(in_state(GameState::Loading)))
        .add_systems(OnExit(GameState::Loading), cleanup_loading_screen)
        .add_systems(OnEnter(GameState::Playing), setup_game)
        .run();
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum GameState {
    #[default]
    Loading,
    Playing,
}

#[derive(Component)]
struct LoadingScreen;

#[derive(Resource)]
struct LoadingTimer {
    timer: Timer,
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    
    // Initialize loading timer (simulating asset loading)
    commands.insert_resource(LoadingTimer {
        timer: Timer::from_seconds(3.0, TimerMode::Once),
    });
}

fn setup_loading_screen(
    mut commands: Commands,
) {
    // Root UI node
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
            LoadingScreen,
        ))
        .with_children(|parent| {
            // Loading text
            parent.spawn((
                Text::new("Loading..."),
                TextFont {
                    font_size: 60.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
            
            // Spinner container
            parent.spawn((
                Node {
                    width: Val::Px(50.0),
                    height: Val::Px(50.0),
                    margin: UiRect::top(Val::Px(30.0)),
                    ..default()
                },
            )).with_children(|parent| {
                // Simple loading spinner
                parent.spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        border: UiRect::all(Val::Px(5.0)),
                        ..default()
                    },
                    // BorderColor { 0: Color::WHITE },
                    BorderRadius::all(Val::Percent(50.0)),
                    LoadingSpinner,
                ));
            });
        });
}

#[derive(Component)]
struct LoadingSpinner;

fn loading_system(
    time: Res<Time>,
    mut timer: ResMut<LoadingTimer>,
    mut next_state: ResMut<NextState<GameState>>,
    mut spinner_query: Query<&mut Transform, With<LoadingSpinner>>,
) {
    // Rotate spinner
    for mut transform in &mut spinner_query {
        transform.rotate_z(-2.0 * time.delta_secs());
    }
    
    // Tick the timer
    timer.timer.tick(time.delta());
    
    // When loading is complete, transition to Playing state
    if timer.timer.is_finished() {
        next_state.set(GameState::Playing);
    }
}

fn cleanup_loading_screen(
    mut commands: Commands,
    query: Query<Entity, With<LoadingScreen>>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

fn setup_game(mut commands: Commands) {
    // Spawn your actual game entities here
    commands.spawn((
        Text::new("Game Started!"),
        TextFont {
            font_size: 40.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(50.0),
            left: Val::Px(50.0),
            ..default()
        },
    ));
}