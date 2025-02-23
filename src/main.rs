use bevy::prelude::*;


// Dimensions des tuiles
const TILE_SIZE: f32 = 32.0;
const VIEW_RADIUS: i32 = 10; // Nombre de tuiles visibles autour de la caméra

// Composants des entités
#[derive(Component, Debug, Clone)]
struct Player;

#[derive(Component, Debug, Clone)]
struct Name(String);

#[derive(Debug, Clone)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Resource)]
struct GreetTimer(Timer);

struct HelloPlugin;
impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)));
        app.add_systems(Startup, add_people);
        app.add_systems(Update, (helloworld, (update_people, greet_people).chain()));
    }
}


// Plugin principal
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(HelloPlugin)
        // .add_startup_system(setup)
        .run();
}

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Player>>) {
    // update our timer with the time elapsed since the last update
    // if that caused the timer to finish, we say hello to everyone
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("hello {}!", name.0);
        }
    }
}

fn update_people(mut query: Query<&mut Name, With<Player>>) {
    for mut name in &mut query {
        if name.0 == "A" {
            name.0 = "Elaina Hume".to_string();
            break; // We don't need to change any other names.
        }
    }
}

fn add_people(mut commands: Commands) {
    commands.spawn((Player, Name("A".to_string())));
    commands.spawn((Player, Name("B".to_string())));
    commands.spawn((Player, Name("C".to_string())));
}

fn helloworld() {
    println!("Hello, world!");
}
