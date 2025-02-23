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

struct HelloPlugin;
impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
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

// // Système d'initialisation
// fn setup(mut commands: Commands) {
//     commands.spawn(Camera2d::default());
// }

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

fn greet_people(query: Query<&Name, With<Player>>) {
    for name in &query {
        println!("hello {}!", name.0);
    }
}

fn helloworld() {
    println!("Hello, world!");
}
