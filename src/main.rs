use bevy::{ecs::query, prelude::*};

fn main() {
    println!("hello world");
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "PC Race".to_string(),
                resolution: (990, 540).into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, startup)
        .add_systems(Update, move_square)
        .run();
}

#[derive(Component)]
struct Mover;

fn startup(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 0.0, 0.0).looking_at(vec3(1.0, 0.0, 0.0), Vec3::Z),
    ));

}

fn move_square(time: Res<Time>, mut query: Query<&mut Transform, With<Mover>>) {
    for mut transform in &mut query {
        transform.translation.x += 100.0 * time.delta_secs();
    }
}
