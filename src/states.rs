use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    Home,
    ModeSelect,
    CourseSelect,
    CarSelect,
    MeasurePerformance,
    TimeAttackGame,
    Result,
    Settings,
    CalcInfo,
}
