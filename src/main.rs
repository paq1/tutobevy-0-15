use bevy::input::keyboard::Key::Camera;
use bevy::prelude::*;
use bevy::text::cosmic_text::rustybuzz::Direction;

// Dimensions des tuiles
const TILE_SIZE: f32 = 32.0;
const VIEW_RADIUS: i32 = 10; // Nombre de tuiles visibles autour de la caméra

// Composants des entités
#[derive(Component, Debug, Clone)]
struct Player {
    position: Position
}

#[derive(Component, Debug, Clone)]
struct Dog;

#[derive(Component, Debug, Clone)]
struct Name(String);

#[derive(Debug, Clone)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Resource)]
struct GreetTimer(Timer);

#[derive(Resource)]
struct SpritePlayer(Sprite);

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d::default());

    let tile_texture = asset_server.load("player.png");
    let sprite: Sprite = Sprite::from_image(tile_texture);
    // let spriteplayer: SpritePlayer = SpritePlayer(sprite);
    commands.spawn((Player { position: Position { x: 0f32, y: 0f32 }}, sprite, Transform::from_xyz(100.0, 0.0, 0.0)));
}

struct HelloPlugin;
impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)));
        app.add_systems(Startup, setup);
        app.add_systems(Update, draw_player);
        // app.add_systems(Update, (helloworld, (update_people, greet_people).chain()));
    }
}

fn draw_player(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Sprite), With<Player>>,
) {
    for (mut transform, mut sprite) in &mut query {
        transform.translation.x += 150. * time.delta_secs();
        // transform.translation = Vec3::new(100.0, 0.0, 0.0);
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

// fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Player>>) {
//     // update our timer with the time elapsed since the last update
//     // if that caused the timer to finish, we say hello to everyone
//     if timer.0.tick(time.delta()).just_finished() {
//         for name in &query {
//             println!("hello {}!", name.0);
//         }
//     }
// }
//
