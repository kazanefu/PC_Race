use bevy::prelude::*;
use sysinfo::System;

/// Resource that wrapping the 'sysinfo' System struct, providing access to PC hardware scanning.
#[derive(Resource)]
pub struct PcMonitor {
    pub system: System,
}

impl Default for PcMonitor {
    fn default() -> Self {
        Self {
            system: System::new_all(),
        }
    }
}

/// Defines the various states the game can end in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameOverCause {
    None,
    GoalReached, // Victory
    FuelEmpty,   // Defeat
    Crash,       // Defeat
    Overheat,    // Defeat
}

impl Default for GameOverCause {
    fn default() -> Self {
        Self::None
    }
}

/// Resource storing the state of the current racing session.
#[derive(Resource)]
pub struct GameSession {
    pub play_time: f32,         // Total time elapsed (seconds)
    pub current_speed: f32,     // Current speed in km/h
    pub current_fuel: f32,      // Current fuel remaining (absolute units)
    pub current_gear: i32,      // Current manual gear (1-6)
    pub current_temp: f32,      // Accumulated CPU + GPU temperature (Celsius)
    pub drs_enabled: bool,      // Whether the Drag Reduction System (DRS) is active
    pub distance_traveled: f32, // Total distance driven in meters
    pub course_length: f32,     // Target distance to reach the goal
    pub is_game_over: bool,     // Flag to pause logic when game ends
    pub game_over_cause: GameOverCause,
}

impl Default for GameSession {
    fn default() -> Self {
        Self {
            play_time: 0.0,
            current_speed: 0.0,
            current_fuel: 100.0,
            current_gear: 1,
            current_temp: 60.0,
            drs_enabled: false,
            distance_traveled: 0.0,
            course_length: 5000.0,
            is_game_over: false,
            game_over_cause: GameOverCause::None,
        }
    }
}

/// Resource storing raw telemetry data captured from the PC hardware.
#[derive(Resource, Default, Debug, Clone)]
pub struct PcStatus {
    pub total_memory: u64,  // Total RAM (Bytes)
    pub used_memory: u64,   // Currently used RAM (Bytes)
    pub cpu_usage: f32,     // Percentage (0-100)
    pub cpu_frequency: u64, // Real-time CPU Clock (MHz)
    pub gpu_usage: f32,     // GPU Load Percentage (0-100)
    pub gpu_temp: f32,      // GPU Temperature (Celsius)
    pub gpu_clock: f32,     // GPU Core Clock (MHz)
    pub cpu_temp: f32,      // CPU Temperature (Celsius)
    pub sensor_error: bool, // True if hardware sensors were not detected

    // Meta-data for logging (marked as allowed dead code for now)
    #[allow(dead_code)]
    pub cpu_cores: usize,
    #[allow(dead_code)]
    pub cpu_name: String,
    #[allow(dead_code)]
    pub gpu_name: String,
    #[allow(dead_code)]
    pub ssd_available: u64,
}

/// Resource storing the dynamic attributes of the player car.
/// These values are re-calculated every frame based on PcStatus.
#[derive(Resource, Default, Debug, Clone)]
pub struct CarStatus {
    pub max_speed: f32,        // Top speed limit (km/h)
    pub fuel_capacity: f32,    // Fuel tank size
    pub weight: f32,           // Total mass (kg) including fuel
    pub fuel_consumption: f32, // Rate of fuel use
    pub acceleration: f32,     // Base engine power
    pub braking: f32,          // Reverse/Decceleration force
    pub grip: f32,             // Traction for steering
    pub handling: f32,         // Turning speed
    pub aerodynamics: f32,     // Drag reduction / DRS strength
    pub drs_acceleration: f32, // Enhanced acceleration when DRS is ON
    pub drs_max_speed: f32,    // Enhanced top speed when DRS is ON
}

/// Resource storing the base design specs of the car.
/// The formulas in specification.md use these as the reference points.
#[derive(Resource, Debug, Clone)]
pub struct BaseCarStatus {
    // Impact Rates (Sensitivity to hardware stats, sum to 100 as per Spec)
    pub cpu_impact: f32,
    pub gpu_impact: f32,
    pub ram_impact: f32,
    pub temp_impact: f32,
    pub ssd_impact: f32,

    // Base performance values (Hardware stats scale from these)
    pub base_max_speed: f32,
    pub base_weight: f32,
    pub base_fuel_consumption: f32,
    pub base_handling: f32,
    pub base_acceleration: f32,
    pub base_braking: f32,
    pub base_grip: f32,
    pub base_aerodynamics: f32,
    pub base_fuel_capacity: f32,
}

impl Default for BaseCarStatus {
    fn default() -> Self {
        Self {
            cpu_impact: 20.0,
            gpu_impact: 20.0,
            ram_impact: 20.0,
            temp_impact: 20.0,
            ssd_impact: 20.0,
            base_max_speed: 300.0,
            base_weight: 1000.0,
            base_fuel_consumption: 0.5,
            base_handling: 2.0,
            base_acceleration: 50.0,
            base_braking: 50.0,
            base_grip: 2.0,
            base_aerodynamics: 1.0,
            base_fuel_capacity: 60.0,
        }
    }
}
