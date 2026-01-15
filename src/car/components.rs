use bevy::prelude::*;

/// Marker component for the player's car.
#[derive(Component)]
pub struct PlayerCar;

/// Component to store the current velocity of an entity.
/// In this game, velocity is primarily use for forward movement and gravity.
#[derive(Component, Default, Debug)]
pub struct Velocity(pub Vec3);
