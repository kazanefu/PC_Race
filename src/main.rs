use bevy::{ecs::query, prelude::*,light::CascadeShadowConfigBuilder};
use std::f32::consts::PI;

mod character;
mod car;
mod spawner;

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
    // directional 'sun' light
    commands.spawn((
        DirectionalLight {
            illuminance: light_consts::lux::OVERCAST_DAY,
            shadows_enabled: true,
            ..default()
        },
        Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        // The default cascade config is designed to handle large scenes.
        // As this example has a much smaller world, we can tighten the shadow
        // bounds for better visual quality.
        CascadeShadowConfigBuilder {
            first_cascade_far_bound: 4.0,
            maximum_distance: 10.0,
            ..default()
        }
        .build(),
    ));

}

fn move_square(time: Res<Time>, mut query: Query<&mut Transform, With<Mover>>) {
    for mut transform in &mut query {
        transform.translation.x += 100.0 * time.delta_secs();
    }
}
