use crate::spawner;
use bevy::prelude::*;

pub trait Machine {
    fn cal_speed(&self) -> i64;
    fn cal_weight(&self) -> i64;
    fn fuel_remain(&self) -> i64;
}

#[derive(Component, Default)]
pub struct PlayerMachineStatus {
    cpu_temperature: i64,
    gpu_temperature: i64,
    cpu_clock: i64,
    gpu_clock: i64,
    memory_used: i64,
    memory_max: i64,
    strage_used: i64,
    strage_max: i64,
    fuel_remain: i64,
}
impl PlayerMachineStatus {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn reset_fuel(&mut self) {
        self.fuel_remain = self.memory_used + self.strage_used;
    }
}

impl Machine for PlayerMachineStatus {
    fn cal_speed(&self) -> i64 {
        self.cpu_clock
    }
    fn cal_weight(&self) -> i64 {
        self.fuel_remain
    }
    fn fuel_remain(&self) -> i64 {
        self.fuel_remain
    }
}

#[derive(Component,Clone,Default)]
pub struct Player {
    id: i32,
}
