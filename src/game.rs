use crate::car::components::*;
use crate::resources::*;
use crate::states::AppState;
use bevy::prelude::*;
use std::process::Command;

/// Component used to mark entities that belong to the game world (level geometry, lights, etc.)
/// for easy cleanup when leaving the game state.
#[derive(Component)]
pub struct GameWorld;

/// Marker for the 3D entity displaying the play time.
#[derive(Component)]
struct GameTimerText;

/// Marker for the 3D entity displaying car telemetry and hardware stats.
#[derive(Component)]
struct HudText;

/// Plugin that manages the main Time Attack game loop and course generation.
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameSession>()
            .add_systems(OnEnter(AppState::TimeAttackGame), setup_game)
            .add_systems(OnExit(AppState::TimeAttackGame), cleanup_game)
            .add_systems(
                Update,
                (game_logic_system, hud_update_system, update_temps)
                    .run_if(in_state(AppState::TimeAttackGame)),
            )
            .add_systems(
                PostUpdate,
                camera_follow.run_if(in_state(AppState::TimeAttackGame)),
            );
    }
}

/// System that updates PC hardware temperatures and usage every 1 second.
/// It uses a combination of 'sysinfo' for general data and 'nvidia-smi' as a fallback for GPU data.
fn update_temps(
    time: Res<Time>,
    mut timer: Local<f32>,
    mut monitor: ResMut<PcMonitor>,
    mut pc_status: ResMut<PcStatus>,
    mut session: ResMut<GameSession>,
) {
    if session.is_game_over {
        return;
    }
    *timer += time.delta_secs();
    if *timer >= 1.0 {
        *timer -= 1.0;

        // --- CPU & RAM (sysinfo) ---
        monitor.system.refresh_all();
        let components = sysinfo::Components::new_with_refreshed_list();

        let mut cpu_temp_sum: f32 = 0.0;
        let mut cpu_count = 0;
        let mut found_cpu_temp = false;

        for component in &components {
            let label = component.label().to_lowercase();
            if let Some(t) = component.temperature() {
                if t > 0.0
                    && (label.contains("cpu")
                        || label.contains("core")
                        || label.contains("package"))
                {
                    cpu_temp_sum += t;
                    cpu_count += 1;
                    found_cpu_temp = true;
                }
            }
        }

        let cpu_t = if cpu_count > 0 {
            cpu_temp_sum / cpu_count as f32
        } else {
            0.0
        };
        pc_status.cpu_temp = cpu_t;
        pc_status.cpu_usage = monitor.system.global_cpu_usage();
        pc_status.cpu_frequency = monitor
            .system
            .cpus()
            .first()
            .map(|c| c.frequency())
            .unwrap_or(0);
        pc_status.used_memory = monitor.system.used_memory();
        pc_status.total_memory = monitor.system.total_memory();

        // --- GPU (nvidia-smi Fallback) ---
        // sysinfo often fails for GPU on Windows. Using nvidia-smi as a robust probe.
        if let Ok(output) = Command::new("nvidia-smi")
            .args([
                "--query-gpu=temperature.gpu,clocks.current.graphics",
                "--format=csv,noheader,nounits",
            ])
            .output()
        {
            let out_str = String::from_utf8_lossy(&output.stdout);
            let parts: Vec<&str> = out_str.trim().split(',').map(|s| s.trim()).collect();
            if parts.len() >= 2 {
                if let Ok(temp) = parts[0].parse::<f32>() {
                    pc_status.gpu_temp = temp;
                }
                if let Ok(clock) = parts[1].parse::<f32>() {
                    pc_status.gpu_clock = clock;
                }
            }
        }

        pc_status.sensor_error = !found_cpu_temp && pc_status.gpu_temp == 0.0;
        session.current_temp = pc_status.cpu_temp + pc_status.gpu_temp;
    }
}

// Redefining build to be cleaner

/// Startup system that initializes the Time Attack course, player car, and UI.
fn setup_game(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    mut session: ResMut<GameSession>,
    car_status: Res<CarStatus>,
) {
    // Reset Session state for a new run
    session.play_time = 0.0;
    session.current_speed = 0.0;
    session.current_fuel = car_status.fuel_capacity; // Start with full tank
    session.current_gear = 1;
    session.current_temp = 60.0;
    session.drs_enabled = false;
    session.distance_traveled = 0.0;
    session.is_game_over = false;

    // --- Procedural Course Generation ---
    // The course is made of repeated segments to simulate a long track.
    let segment_length = 10.0;
    let total_course_visual = 10000.0;
    let num_segments = (total_course_visual / segment_length) as i32;
    let road_width = 40.0;

    let road_material = materials.add(Color::srgb(0.2, 0.2, 0.25));
    let border_material = materials.add(Color::srgb(0.8, 0.8, 0.8));

    for i in 0..num_segments {
        let z_pos = -(i as f32) * segment_length;
        // Simple height variation using sine wave
        // With segment_length=10, distance = i * 10.
        // We want distance/1000. So (i * 10) / 1000 = i * 0.01
        let y_pos = (i as f32 * 0.01).sin() * 5.0; // Hills synchronized with car/systems.rs

        // Road Segment
        commands.spawn((
            Mesh3d(meshes.add(Cuboid::new(road_width, 1.0, segment_length))),
            MeshMaterial3d(road_material.clone()),
            Transform::from_xyz(0.0, y_pos - 0.5, z_pos), // -0.5 to keep surface at y_pos
            GameWorld,
        ));

        // Side barriers
        commands.spawn((
            Mesh3d(meshes.add(Cuboid::new(2.0, 1.5, segment_length))),
            MeshMaterial3d(border_material.clone()),
            Transform::from_xyz(road_width / 2.0 + 1.0, y_pos + 0.25, z_pos),
            GameWorld,
        ));
        commands.spawn((
            Mesh3d(meshes.add(Cuboid::new(2.0, 1.5, segment_length))),
            MeshMaterial3d(border_material.clone()),
            Transform::from_xyz(-(road_width / 2.0 + 1.0), y_pos + 0.25, z_pos),
            GameWorld,
        ));
    }

    // --- Finish Line ---
    let finish_z = -session.course_length;
    let goal_material = materials.add(Color::srgba(1.0, 0.2, 0.2, 0.6)); // Translucent red
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(road_width, 10.0, 1.0))),
        MeshMaterial3d(goal_material),
        Transform::from_xyz(0.0, 5.0, finish_z),
        GameWorld,
    ));
    // Additional Goal post decorations
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(2.0, 20.0, 2.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.5, 0.5, 0.5))),
        Transform::from_xyz(road_width / 2.0 + 2.0, 10.0, finish_z),
        GameWorld,
    ));
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(2.0, 20.0, 2.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.5, 0.5, 0.5))),
        Transform::from_xyz(-(road_width / 2.0 + 2.0), 10.0, finish_z),
        GameWorld,
    ));

    // Light
    commands.spawn((
        DirectionalLight::default(),
        Transform::from_xyz(50.0, 100.0, 50.0).looking_at(Vec3::ZERO, Vec3::Y),
        GameWorld,
    ));

    // Player Car (Cube)
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(2.0, 1.0, 4.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.9, 0.1, 0.1))), // Red sporty car
        Transform::from_xyz(0.0, 1.0, 0.0),                        // Start slightly up
        Velocity::default(),
        PlayerCar,
        GameWorld,
    ));

    // 4. Create UI Overlay (HUD)
    setup_hud(&mut commands, &asset_server);
}

fn setup_hud(commands: &mut Commands, asset_server: &AssetServer) {
    let font = asset_server.load("fonts/NotoSansJP-Bold.ttf");

    // Top Left - Time
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(10.0),
                left: Val::Px(10.0),
                ..default()
            },
            GameWorld,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Time: 0.00"),
                TextFont {
                    font: font.clone(),
                    font_size: 40.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                GameTimerText,
            ));
        });

    // Top Right - Stats & Telemetry
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(10.0),
                right: Val::Px(10.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::FlexEnd,
                ..default()
            },
            GameWorld,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("HUD Info"),
                TextFont {
                    font: font.clone(),
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                HudText,
            ));
        });
}

fn cleanup_game(mut commands: Commands, query: Query<Entity, With<GameWorld>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

// --- Systems ---

// input_system moved to car/systems.rs

// physics_system and input_system moved to car/systems.rs

/// Core game rule checker: Handes Victory (Distance), Failure (Fuel/Overheat), and Bounds.
fn game_logic_system(
    time: Res<Time>,
    mut session: ResMut<GameSession>,
    mut next_state: ResMut<NextState<AppState>>,
    query: Query<&Transform, With<PlayerCar>>,
) {
    if session.is_game_over {
        return;
    }

    // Advance total game time
    session.play_time += time.delta_secs();

    // Condition 1: Victory - Reached the end of the course
    if session.distance_traveled >= session.course_length {
        println!("Goal Reached!");
        session.is_game_over = true;
        session.game_over_cause = GameOverCause::GoalReached;
        next_state.set(AppState::Result);
        return;
    }

    // Condition 2: Failure - Out of Fuel
    if session.current_fuel <= 0.0 {
        println!("Game Over: Out of Fuel!");
        session.is_game_over = true;
        session.game_over_cause = GameOverCause::FuelEmpty;
        next_state.set(AppState::Result);
        return;
    }

    // Condition 3: Failure - Engine Overheat (Specification rule 98)
    if session.current_temp >= 255.0 {
        println!("Game Over: Engine Overheat!");
        session.is_game_over = true;
        session.game_over_cause = GameOverCause::Overheat;
        next_state.set(AppState::Result);
        return;
    }

    // Condition 4: Failure - Crash / Extreme Course Out (Specification rule 94)
    if let Some(transform) = query.iter().next() {
        // Road width 40.0 -> +/- 20.0 from center, plus safety
        if transform.translation.x.abs() > 25.0 {
            println!("Game Over: Course Out (Crash)!");
            session.is_game_over = true;
            session.game_over_cause = GameOverCause::Crash;
            next_state.set(AppState::Result);
            return;
        }
    }
}

fn hud_update_system(
    session: Res<GameSession>,
    car_status: Res<CarStatus>,
    pc_status: Res<PcStatus>,
    mut timer_text: Query<&mut Text, (With<GameTimerText>, Without<HudText>)>,
    mut hud_text: Query<&mut Text, (With<HudText>, Without<GameTimerText>)>,
) {
    if let Some(mut text) = timer_text.iter_mut().next() {
        text.0 = format!("Time: {:.2}", session.play_time);
    }

    if let Some(mut text) = hud_text.iter_mut().next() {
        let sensor_msg = if pc_status.sensor_error {
            "\n[HARDWARE SENSORS NOT FOUND]"
        } else {
            ""
        };

        text.0 = format!(
            "Speed: {:.1} km/h\nGear: {}\nFuel: {:.1} / {:.1}\nTemp: {:.1} C\nDRS: {}{}\n\n[PC STATUS]\nCPU: {:.1} MHz | {:.1}%\nGPU: {:.1} MHz\nRAM: {:.1} GB",
            session.current_speed,
            session.current_gear,
            session.current_fuel,
            car_status.fuel_capacity,
            session.current_temp,
            if session.drs_enabled { "ON" } else { "OFF" },
            sensor_msg,
            pc_status.cpu_frequency as f32,
            pc_status.cpu_usage,
            pc_status.gpu_clock,
            pc_status.used_memory as f32 / 1024.0 / 1024.0 / 1024.0
        );
    }
}

/// Smoothly interpolates the camera to follow the car's position and orientation.
/// This system runs in PostUpdate to minimize jitter.
fn camera_follow(
    mut camera_query: Query<&mut Transform, (With<Camera3d>, Without<PlayerCar>)>,
    car_query: Query<&Transform, With<PlayerCar>>,
    time: Res<Time>,
) {
    if let (Some(mut cam_transform), Some(car_transform)) =
        (camera_query.iter_mut().next(), car_query.iter().next())
    {
        let dt = time.delta_secs();

        // 1. Position Setup (Third-person view behind the car)
        let offset = Vec3::new(0.0, 5.0, 15.0);
        let target_pos = car_transform.translation + car_transform.rotation * offset;

        // 2. Stable Interplation (Lerp)
        // We use an exponential decay formula for frame-rate independent smoothness.
        let lerp_rate = 8.0;
        let lerp_factor = 1.0 - (-lerp_rate * dt).exp();
        cam_transform.translation = cam_transform.translation.lerp(target_pos, lerp_factor);

        // 3. Aiming (Keep looking at the car)
        let look_target = car_transform.translation + Vec3::Y * 1.0;
        cam_transform.look_at(look_target, Vec3::Y);
    }
}
