use crate::car::components::*;
use crate::resources::*;
use crate::states::AppState;
use bevy::prelude::*;

/// System that handles user input for gear shifting and DRS.
/// Manual gear shifting is a core requirement for tuning performance.
pub fn car_input_system(input: Res<ButtonInput<KeyCode>>, mut session: ResMut<GameSession>) {
    if session.is_game_over {
        return;
    }

    // Gear Shifting (Manual as per Spec 108)
    if input.just_pressed(KeyCode::ArrowRight) {
        session.current_gear = (session.current_gear + 1).min(6);
    }
    if input.just_pressed(KeyCode::ArrowLeft) {
        session.current_gear = (session.current_gear - 1).max(1);
    }

    // DRS Logic (Arrow Up/Down)
    if input.just_pressed(KeyCode::ArrowUp) || input.just_pressed(KeyCode::KeyE) {
        session.drs_enabled = true;
    }
    if input.just_pressed(KeyCode::ArrowDown) || input.just_pressed(KeyCode::KeyQ) {
        session.drs_enabled = false;
    }
}

/// The core physics engine for the car.
/// This system calculates all car properties (acceleration, grip, etc.) based on
/// actual hardware performance (CPU, GPU, RAM) and applies them to the 3D entity.
pub fn car_physics_system(
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    mut session: ResMut<GameSession>,
    base_car: Res<BaseCarStatus>,
    pc_status: Res<PcStatus>,
    mut car_status: ResMut<CarStatus>,
    mut next_state: ResMut<NextState<AppState>>,
    mut query: Query<(&mut Transform, &mut Velocity), With<PlayerCar>>,
) {
    if session.is_game_over {
        return;
    }

    let dt = time.delta_secs();

    // --- Physics Internal Constants ---
    // These constants are used to translate raw frequency (MHz) and bytes
    // into meaningful impact percentages for the physics formulas.
    const RAM_IMPACT_FACTOR: f32 = 0.000000001;
    const CPU_IMPACT_FACTOR: f32 = 0.000001;
    const GPU_IMPACT_FACTOR: f32 = 0.00001;
    const TEMP_IMPACT_FACTOR: f32 = 0.001;

    // --- Bevy Gameplay Scaling ---
    // These factors ensure the game feels fun and playable.
    const BEVY_ACCEL_SCALE: f32 = 50.0;
    const BEVY_HANDLING_SCALE: f32 = 2.0;
    const BEVY_BRAKING_SCALE: f32 = 3000.0;
    const BEVY_ENGINE_FORCE_SCALE: f32 = 500.0;
    const BEVY_DRAG_SCALE: f32 = 0.4;
    const BEVY_STEERING_SENSITIVITY: f32 = 1.2;
    const BEVY_GRAVITY_SCALE: f32 = 3.0;

    const FUEL_BURN_MULTIPLIER: f32 = 0.5;
    const GROUND_FRICTION: f32 = 2.0;
    const COURSE_OUT_PENALTY_RATE: f32 = 2.0; // Seconds of penalty per actual second off-road

    // The 'const' value mentioned in specification.md for fine-tuning.
    let const_val = 1.0;

    // --- Car Status Dynamic Calculations (Strict Spec Alignment) ---
    // These calculations are performed every frame to reflect hardware state.

    // Calculate Max Speed based on CPU Clock
    let cpu_u = pc_status.cpu_usage / 100.0;
    car_status.max_speed = base_car.base_max_speed
        * (1.0
            + (base_car.cpu_impact * CPU_IMPACT_FACTOR)
                * pc_status.cpu_frequency as f32
                * (1.0 + cpu_u))
        * const_val;

    // Calculate Dynamic Weight based on remaining fuel
    let fuel_ratio = (session.current_fuel / car_status.fuel_capacity).clamp(0.0, 1.0);
    car_status.weight = base_car.base_weight * (1.0 + fuel_ratio) * const_val;

    // Calculate Fuel Consumption based on Temperature and GPU usage
    car_status.fuel_consumption = base_car.base_fuel_consumption
        * (1.0
            + (base_car.temp_impact * TEMP_IMPACT_FACTOR)
                * (pc_status.cpu_temp + pc_status.gpu_temp)
                * (base_car.gpu_impact * GPU_IMPACT_FACTOR)
                * pc_status.gpu_clock)
        * const_val;

    // Calculate Grip based on available RAM
    let ram_avail = (pc_status.total_memory - pc_status.used_memory) as f32;
    car_status.grip = base_car.base_grip
        * (1.0 + (base_car.ram_impact * RAM_IMPACT_FACTOR) * ram_avail)
        * const_val;

    // Calculate Handling (Steering Agility)
    // Formula: handling = base handling * grip / weight
    car_status.handling = base_car.base_handling * car_status.grip / car_status.weight
        * const_val
        * BEVY_HANDLING_SCALE;

    // Calculate Aerodynamics based on GPU performance
    let gpu_u = pc_status.gpu_usage / 100.0;
    car_status.aerodynamics = base_car.base_aerodynamics
        * (1.0 + (base_car.gpu_impact * GPU_IMPACT_FACTOR) * pc_status.gpu_clock * (1.0 + gpu_u))
        * const_val;

    // Gear appropriate logic: acceleration is 2.0x if gear is near ideal for speed ratio
    let gear_limit_ratio = session.current_gear as f32 / 6.0;
    let gear_max_speed = car_status.max_speed * gear_limit_ratio;

    let speed_ratio = if car_status.max_speed > 0.0 {
        session.current_speed / car_status.max_speed
    } else {
        0.0
    };
    let ideal_gear = (speed_ratio * 6.0).ceil().clamp(1.0, 6.0) as i32;
    let gear_factor = if (session.current_gear - ideal_gear).abs() <= 1 {
        2.0
    } else {
        1.0
    };

    // Calculate Acceleration
    car_status.acceleration = base_car.base_acceleration
        * ((1.0 + (base_car.gpu_impact * GPU_IMPACT_FACTOR) * pc_status.gpu_clock * (1.0 + gpu_u))
            * gear_factor
            / car_status.weight)
        * const_val
        * BEVY_ACCEL_SCALE;

    // DRS Boosts
    car_status.drs_acceleration = car_status.acceleration + car_status.aerodynamics * const_val;
    car_status.drs_max_speed = car_status.max_speed + car_status.aerodynamics * const_val;

    // Apply Gear Limit to Max Speed
    let final_max_speed = if session.drs_enabled {
        (car_status.drs_max_speed * gear_limit_ratio).min(car_status.drs_max_speed)
    } else {
        gear_max_speed.min(car_status.max_speed)
    };

    // Calculate Braking performance
    car_status.braking = base_car.base_braking * car_status.grip / car_status.weight
        * const_val
        * BEVY_BRAKING_SCALE;

    if let Some((mut transform, mut velocity)) = query.iter_mut().next() {
        let mut force = Vec3::ZERO;

        // A. Gravity Calculation
        let gravity_base = Vec3::new(0.0, -9.81, 0.0);
        force += gravity_base * BEVY_GRAVITY_SCALE * (car_status.weight / 100.0);

        // B. Engine / Brake Force Determination
        let mut engine_force_mag = 0.0;
        let current_speed_ms = velocity.0.length();
        let current_speed_kmh = current_speed_ms * 3.6;
        session.current_speed = current_speed_kmh;

        if input.pressed(KeyCode::KeyW) {
            // Apply acceleration based on whether DRS is open
            let accel = if session.drs_enabled {
                car_status.drs_acceleration
            } else {
                car_status.acceleration
            };
            engine_force_mag += accel * BEVY_ENGINE_FORCE_SCALE;

            // Consume fuel while accelerating
            let burn_rate = car_status.fuel_consumption * FUEL_BURN_MULTIPLIER * dt;
            session.current_fuel -= burn_rate;
        } else if input.pressed(KeyCode::KeyS) {
            // Apply braking force only if the car is currently moving
            if current_speed_ms > 0.1 {
                engine_force_mag -= car_status.braking;
            } else {
                velocity.0 = Vec3::ZERO; // Come to a complete stop
            }
        }

        // Apply forces in the car's current forward-facing direction
        let forward_dir = transform.forward();
        let forward_flat = Vec3::new(forward_dir.x, 0.0, forward_dir.z).normalize_or_zero();
        force += forward_flat * engine_force_mag;

        // C. Air Resistance (Drag) and Ground Friction
        let drag_coeff = if session.drs_enabled {
            car_status.aerodynamics * BEVY_DRAG_SCALE
        } else {
            car_status.aerodynamics
        };
        force -= velocity.0 * drag_coeff * BEVY_DRAG_SCALE;
        force -= velocity.0 * GROUND_FRICTION;

        // --- Physics Integration ---
        // Basic F=ma and v=u+at implementation
        let mass = car_status.weight;
        let acceleration_vec = force / mass;
        velocity.0 += acceleration_vec * dt;

        // --- Speed Capping Logic ---
        // Ensure the car never exceeds the physical maximums calculated earlier
        let absolute_max_ms = if session.drs_enabled {
            car_status.drs_max_speed
        } else {
            car_status.max_speed
        } / 3.6;

        let hard_cap_ms = (final_max_speed / 3.6).min(absolute_max_ms);

        if velocity.0.length() > hard_cap_ms {
            velocity.0 = velocity.0.normalize() * hard_cap_ms;
        }

        // Update Position based on Velocity
        transform.translation += velocity.0 * dt;
        session.distance_traveled += velocity.0.length() * dt;

        // --- Steering Logic ---
        // Handles horizontal rotation (Yaw) using the Handling attribute
        let handling = car_status.handling * BEVY_STEERING_SENSITIVITY;
        let mut rotation = 0.0;
        if input.pressed(KeyCode::KeyA) {
            rotation = handling * dt;
        }
        if input.pressed(KeyCode::KeyD) {
            rotation = -handling * dt;
        }
        transform.rotate_y(rotation);

        // Align velocity direction with the car's orientation to prevent drifting
        if velocity.0.length() > 0.1 {
            let fwd = *transform.forward();
            velocity.0 = fwd * velocity.0.length();
        }

        // --- Environment Collision & Course-Out Rules ---

        // Ground Height Mapping (Sine wave hills)
        let z_pos_course = -transform.translation.z;
        let ground_y = if z_pos_course >= 0.0 {
            (z_pos_course / 1000.0).sin() * 5.0
        } else {
            0.0
        };

        // Keep the car on the ground surface
        if transform.translation.y < ground_y + 0.5 {
            transform.translation.y = ground_y + 0.5;
            if velocity.0.y < 0.0 {
                velocity.0.y = 0.0;
            }
        }

        // Course-Out Determination (Spec 94)
        let road_limit = 20.0;
        let crash_limit = 35.0;

        if transform.translation.x.abs() > road_limit {
            // Apply time penalty while off-road
            session.play_time += dt * COURSE_OUT_PENALTY_RATE;
        }

        if transform.translation.x.abs() > crash_limit {
            // Irrecoverable crash if too far from the track center
            println!("Game Over: Course Out Crash!");
            session.is_game_over = true;
            session.game_over_cause = GameOverCause::Crash;
            next_state.set(AppState::Result);
        }
    }
}
