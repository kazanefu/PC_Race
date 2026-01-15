use crate::car::player;
use bevy::{ecs::system::command, prelude::*};

#[derive(Copy, Clone)]
pub enum CharacterKind {
    Player,
    Opponent,
}
impl CharacterKind {
    pub fn spawn_character(&self, pos: Vec3, command: &mut Commands) {
        match self {
            CharacterKind::Player =>{ command.spawn((
                player::Player::default(),
                Transform {
                    translation: pos,
                    ..default()
                },
            ));},
            _ => unimplemented!()
        }
    }
}

#[derive(Resource)]
pub struct CharacterSpawnList {
    list: Vec<(CharacterKind, Vec3)>,
}

impl CharacterSpawnList {
    pub fn spawn_all(&self, command: &mut Commands) {
        self.list
            .iter()
            .for_each(|(kind, pos)| kind.spawn_character(*pos, command));
    }
}
