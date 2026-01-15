use crate::resources::{BaseCarStatus, CarStatus, PcStatus};
use crate::states::AppState;
use crate::ui::styles::{
    HOVERED_BUTTON, NORMAL_BUTTON, PRESSED_BUTTON, get_button_text_color, get_button_text_font,
    get_title_text_color, get_title_text_font,
};
use bevy::prelude::*;
use bevy::render::renderer::RenderAdapterInfo;
use sysinfo::System as SysinfoSystem;

// --- Setup Flow Components ---

/// Marker for UI elements in the Course Selection screen.
#[derive(Component)]
struct CourseSelectUi;

/// Marker for UI elements in the Car Selection screen.
#[derive(Component)]
struct CarSelectUi;

/// Marker for UI elements in the Performance Measurement screen.
#[derive(Component)]
struct MeasureUi;

/// Marker for the single "Select" button on the course screen.
#[derive(Component)]
struct CourseButton;

/// Component for car selection buttons, storing the ID of the car type.
#[derive(Component)]
struct CarButton(u32);

/// Timer used to control the duration of the "Measurement" animation/phase.
#[derive(Resource)]
struct MeasureTimer(Timer);

/// Plugin that manages the pre-game flow: Course Select -> Car Select -> Performance Measurement.
pub struct SetupFlowPlugin;

impl Plugin for SetupFlowPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MeasureTimer(Timer::from_seconds(3.0, TimerMode::Once)))
            // Course Select
            .add_systems(OnEnter(AppState::CourseSelect), setup_course_select)
            .add_systems(OnExit(AppState::CourseSelect), cleanup_course_select)
            .add_systems(
                Update,
                interact_course_select.run_if(in_state(AppState::CourseSelect)),
            )
            // Car Select
            .add_systems(OnEnter(AppState::CarSelect), setup_car_select)
            .add_systems(OnExit(AppState::CarSelect), cleanup_car_select)
            .add_systems(
                Update,
                interact_car_select.run_if(in_state(AppState::CarSelect)),
            )
            // Measure Performance
            .add_systems(
                OnEnter(AppState::MeasurePerformance),
                setup_measure_performance,
            )
            .add_systems(
                OnExit(AppState::MeasurePerformance),
                cleanup_measure_performance,
            )
            .add_systems(
                Update,
                update_measure_performance.run_if(in_state(AppState::MeasurePerformance)),
            );
    }
}

// --- Course Select ---

fn setup_course_select(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.05, 0.05, 0.05)),
            CourseSelectUi,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Select Course"),
                get_title_text_font(&asset_server),
                get_title_text_color(),
            ));
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(250.0),
                        height: Val::Px(80.0),
                        margin: UiRect::top(Val::Px(20.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(NORMAL_BUTTON),
                    CourseButton,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Course 1"),
                        get_button_text_font(&asset_server),
                        get_button_text_color(),
                    ));
                });
        });
}

fn cleanup_course_select(mut commands: Commands, query: Query<Entity, With<CourseSelectUi>>) {
    // Despawn UI
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

fn interact_course_select(
    mut query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<CourseButton>),
    >,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for (interaction, mut color) in &mut query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                next_state.set(AppState::CarSelect);
            }
            Interaction::Hovered => *color = HOVERED_BUTTON.into(),
            Interaction::None => *color = NORMAL_BUTTON.into(),
        }
    }
}

// --- Car Select ---

fn setup_car_select(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.05, 0.05, 0.05)),
            CarSelectUi,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Select Car"),
                get_title_text_font(&asset_server),
                get_title_text_color(),
            ));

            let cars = vec![
                ("Balance Type", Color::srgb(0.2, 0.5, 0.8), 0),
                ("Speed Type", Color::srgb(0.8, 0.2, 0.2), 1),
                ("Accel Type", Color::srgb(0.2, 0.8, 0.2), 2),
            ];

            for (name, _, id) in cars {
                parent
                    .spawn((
                        Button,
                        Node {
                            width: Val::Px(250.0),
                            height: Val::Px(60.0),
                            margin: UiRect::top(Val::Px(10.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(NORMAL_BUTTON),
                        CarButton(id),
                    ))
                    .with_children(|parent| {
                        parent.spawn((
                            Text::new(name),
                            get_button_text_font(&asset_server),
                            get_button_text_color(),
                        ));
                    });
            }
        });
}

fn cleanup_car_select(mut commands: Commands, query: Query<Entity, With<CarSelectUi>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

fn interact_car_select(
    mut query: Query<(&Interaction, &mut BackgroundColor, &CarButton), Changed<Interaction>>, // Modified query
    mut next_state: ResMut<NextState<AppState>>,
    mut base_car: ResMut<BaseCarStatus>, // Added BaseCarStatus resource
) {
    for (interaction, mut color, car_btn) in &mut query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();

                // Assign Base Stats based on selection
                match car_btn.0 {
                    0 => {
                        // Balance
                        base_car.cpu_impact = 20.0;
                        base_car.gpu_impact = 20.0;
                        base_car.ram_impact = 20.0;
                        base_car.temp_impact = 20.0;
                        base_car.ssd_impact = 20.0;
                    }
                    1 => {
                        // Speed
                        base_car.cpu_impact = 50.0;
                        base_car.gpu_impact = 10.0;
                        base_car.ram_impact = 10.0;
                        base_car.temp_impact = 15.0;
                        base_car.ssd_impact = 15.0;
                    }
                    2 => {
                        // Accel
                        base_car.cpu_impact = 10.0;
                        base_car.gpu_impact = 50.0;
                        base_car.ram_impact = 10.0;
                        base_car.temp_impact = 15.0;
                        base_car.ssd_impact = 15.0;
                    }
                    _ => {}
                }

                next_state.set(AppState::MeasurePerformance);
            }
            Interaction::Hovered => *color = HOVERED_BUTTON.into(),
            Interaction::None => *color = NORMAL_BUTTON.into(),
        }
    }
}

// --- Measure Performance ---

/// Setup system for the Performance Measurement screen.
/// It performs a one-time scan of the PC's hardware and calculates the car's initial stats.
fn setup_measure_performance(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut timer: ResMut<MeasureTimer>,
    mut pc_status: ResMut<PcStatus>,
    mut car_status: ResMut<CarStatus>,
    base_car: Res<BaseCarStatus>,
    adapter_info: Res<RenderAdapterInfo>, // Provides basic GPU info from Bevy's renderer
) {
    timer.0.reset();

    // Measure System Info
    let mut sys = SysinfoSystem::new_all();
    sys.refresh_all();
    let components = sysinfo::Components::new_with_refreshed_list();

    // CPU Info
    let total_memory = sys.total_memory();
    let used_memory = sys.used_memory();
    let cpu_usage = sys.global_cpu_usage();
    let cpu_name = sys
        .cpus()
        .first()
        .map(|c| c.brand().to_string())
        .unwrap_or("Unknown".into());
    let cpu_freq = sys.cpus().first().map(|c| c.frequency()).unwrap_or(3000);

    let mut cpu_temp_sum: f32 = 0.0;
    let mut cpu_temp_count = 0;
    let mut found_cpu_temp = false;

    for component in &components {
        let label = component.label().to_lowercase();
        if let Some(t) = component.temperature() {
            if t > 0.0
                && (label.contains("cpu") || label.contains("core") || label.contains("package"))
            {
                cpu_temp_sum += t;
                cpu_temp_count += 1;
                found_cpu_temp = true;
            }
        }
    }
    let cpu_temp = if cpu_temp_count > 0 {
        cpu_temp_sum / cpu_temp_count as f32
    } else {
        0.0
    };

    // --- GPU Data Capture (NVIDIA System Management Interface) ---
    // sysinfo often fails to detect GPU temperature on Windows without elevation.
    // nvidia-smi is part of the standard driver and provides a reliable fallback.
    let gpu_name = adapter_info.name.clone();
    let mut gpu_temp = 0.0;
    let mut gpu_clock = 1500.0; // Default fallback if probe fails

    if let Ok(output) = std::process::Command::new("nvidia-smi")
        .args([
            "--query-gpu=temperature.gpu,clocks.current.graphics",
            "--format=csv,noheader,nounits",
        ])
        .output()
    {
        let out_str = String::from_utf8_lossy(&output.stdout);
        let parts: Vec<&str> = out_str.trim().split(',').map(|s| s.trim()).collect();
        if parts.len() >= 2 {
            if let Ok(t) = parts[0].parse::<f32>() {
                gpu_temp = t;
            }
            if let Ok(c) = parts[1].parse::<f32>() {
                gpu_clock = c;
            }
        }
    }

    // Set Sensor Error Flag
    pc_status.sensor_error = !found_cpu_temp && gpu_temp == 0.0;

    println!(
        "DEBUG: Final Measured Temp -> CPU: {}C, GPU: {}C | Found: {}",
        cpu_temp, gpu_temp, found_cpu_temp
    );

    // Rough GPU usage estimation
    let gpu_usage = cpu_usage * 1.1;

    let disks = sysinfo::Disks::new_with_refreshed_list();
    let ssd_available = disks.iter().map(|d| d.available_space()).sum::<u64>();

    // --- Update resources ---
    pc_status.cpu_frequency = cpu_freq as u64;
    pc_status.cpu_usage = cpu_usage;
    pc_status.total_memory = total_memory;
    pc_status.used_memory = used_memory;
    pc_status.gpu_clock = gpu_clock;
    pc_status.gpu_usage = gpu_usage;
    pc_status.gpu_temp = gpu_temp;
    pc_status.cpu_temp = cpu_temp;

    let const_val = 1.0;

    // --- Physics Constants & Scaling (MHz Recalibration) ---
    const CPU_IMPACT_FACTOR: f32 = 0.000001; // Synchronized with game.rs
    const GPU_IMPACT_FACTOR: f32 = 0.00001;
    const RAM_IMPACT_FACTOR: f32 = 0.000000001;
    const TEMP_IMPACT_FACTOR: f32 = 0.001;
    const SSD_IMPACT_FACTOR: f32 = 0.0000000001;

    // Bevy-specific scaling factors
    const BEVY_ACCEL_SCALE: f32 = 50.0;
    const BEVY_HANDLING_SCALE: f32 = 2.0;
    const BEVY_BRAKING_SCALE: f32 = 3000.0;

    // Data values
    let cpu_clock = cpu_freq as f32;
    let ram_used = used_memory as f32;
    let ram_avail = (total_memory - used_memory) as f32;
    let ssd_avail = ssd_available as f32;
    let cpu_u = cpu_usage / 100.0;
    let gpu_u = gpu_usage / 100.0;

    // 1. Max Speed
    // max speed = base max speed * (1 + CPU Impact rate * CPU clock *(1 + CPU usage rate)) * const
    car_status.max_speed = base_car.base_max_speed
        * (1.0 + (base_car.cpu_impact * CPU_IMPACT_FACTOR) * cpu_clock * (1.0 + cpu_u))
        * const_val;

    // 2. Fuel Capacity
    // fuel capacity = base fuel capacity * (1 + RAM Impact rate * RAM used size + SSD Impact rate * SSD available size) * const
    car_status.fuel_capacity = base_car.base_fuel_capacity
        * (1.0
            + (base_car.ram_impact * RAM_IMPACT_FACTOR) * ram_used
            + (base_car.ssd_impact * SSD_IMPACT_FACTOR) * ssd_avail)
        * const_val;

    // 3. Weight
    // weight = base weight * (1 + Remaining fuel) * const
    // Calculation time assumption: Full Fuel (1.0 relative)
    car_status.weight = base_car.base_weight * (1.0 + 1.0) * const_val;

    // 4. Fuel Consumption
    // fuel consumption = base fuel consumption * (1 + temperature Impact rate * (CPU temperature + GPU temperature) * GPU Impact rate * GPU clock) * const
    car_status.fuel_consumption = base_car.base_fuel_consumption
        * (1.0
            + (base_car.temp_impact * TEMP_IMPACT_FACTOR)
                * (cpu_temp + gpu_temp)
                * (base_car.gpu_impact * GPU_IMPACT_FACTOR)
                * gpu_clock)
        * const_val;

    // 5. Acceleration
    // acceleration = base acceleration * ((1 + GPU Impact rate * GPU clock * (1 + GPU usage rate)) * if gear is appropriate then 2.0 else 1.0 / weight) * const
    // Static stat assumes "Gear Appropriate" (factor 2.0)
    let gear_appro_factor = 2.0;
    car_status.acceleration = base_car.base_acceleration
        * ((1.0 + (base_car.gpu_impact * GPU_IMPACT_FACTOR) * gpu_clock * (1.0 + gpu_u))
            * gear_appro_factor
            / car_status.weight)
        * const_val
        * BEVY_ACCEL_SCALE;

    // 6. Grip
    // grip = base grip * (1 + RAM Impact rate * RAM available size) * const
    car_status.grip = base_car.base_grip
        * (1.0 + (base_car.ram_impact * RAM_IMPACT_FACTOR) * ram_avail)
        * const_val;

    // 7. Handling
    // handling = base handling * grip / weight * const
    car_status.handling = base_car.base_handling * car_status.grip / car_status.weight
        * const_val
        * BEVY_HANDLING_SCALE;

    // 8. Aerodynamics
    // aerodynamics = base aerodynamics * (1 + GPU Impact rate * GPU clock * (1 + GPU usage rate)) * const
    car_status.aerodynamics = base_car.base_aerodynamics
        * (1.0 + (base_car.gpu_impact * GPU_IMPACT_FACTOR) * gpu_clock * (1.0 + gpu_u))
        * const_val;

    // 9. DRS
    car_status.drs_acceleration = car_status.acceleration + car_status.aerodynamics * const_val;
    car_status.drs_max_speed = car_status.max_speed + car_status.aerodynamics * const_val;

    // 10. Braking (Negative force agility)
    car_status.braking = base_car.base_braking * car_status.grip / car_status.weight
        * const_val
        * BEVY_BRAKING_SCALE;

    // --- Performance Results UI ---
    // Displays the detected hardware specs and resulting car attributes to the player.
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                row_gap: Val::Px(10.0),
                ..default()
            },
            BackgroundColor(Color::BLACK),
            MeasureUi,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Measuring PC Performance... Press Enter to Continue"),
                get_title_text_font(&asset_server),
                get_title_text_color(),
            ));

            let stat_style = (
                TextFont {
                    font: asset_server.load("fonts/NotoSansJP-Bold.ttf"),
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::srgb(0.8, 0.8, 0.8)),
            );

            parent.spawn((
                Text::new(format!("CPU: {} @ {} MHz", cpu_name, cpu_freq)),
                stat_style.clone().0.clone(),
                stat_style.clone().1,
            ));
            parent.spawn((
                Text::new(format!(
                    "RAM: {} / {} GB",
                    used_memory / 1024 / 1024 / 1024,
                    total_memory / 1024 / 1024 / 1024
                )),
                stat_style.clone().0.clone(),
                stat_style.clone().1,
            ));
            parent.spawn((
                Text::new(format!(
                    "GPU: {} | {}% | {:.0} MHz",
                    gpu_name, gpu_usage, gpu_clock
                )),
                stat_style.clone().0.clone(),
                stat_style.clone().1,
            ));
            parent.spawn((
                Text::new(format!("Temp: CPU {:.0}C + GPU {:.0}C", cpu_temp, gpu_temp)),
                stat_style.clone().0.clone(),
                stat_style.clone().1,
            ));

            parent.spawn((
                Text::new("--- Car Status ---"),
                stat_style.clone().0.clone(),
                stat_style.clone().1,
            ));
            parent.spawn((
                Text::new(format!("Max Speed: {:.1}", car_status.max_speed)),
                stat_style.clone().0.clone(),
                stat_style.clone().1,
            ));
            parent.spawn((
                Text::new(format!("Acceleration: {:.1}", car_status.acceleration)),
                stat_style.clone().0.clone(),
                stat_style.clone().1,
            ));
            parent.spawn((
                Text::new(format!("Handling: {:.1}", car_status.handling)),
                stat_style.clone().0.clone(),
                stat_style.clone().1,
            ));
        });
}

fn cleanup_measure_performance(mut commands: Commands, query: Query<Entity, With<MeasureUi>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

fn update_measure_performance(
    input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if input.just_pressed(KeyCode::Enter) {
        next_state.set(AppState::TimeAttackGame);
    }
}
