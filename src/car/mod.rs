// Car Module Definition
// This module handles everything related to the player's car, including
// physics, input, and component definitions.
pub mod components;
pub mod systems;

use crate::states::AppState;
use bevy::prelude::*;
use systems::*;

/// Plugin that handles the car's behavior during the game.
/// It registers the input and physics systems to run only when the game is active.
pub struct CarPlugin;

impl Plugin for CarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (car_input_system, car_physics_system).run_if(in_state(AppState::TimeAttackGame)),
        );
    }
}
