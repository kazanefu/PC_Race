use bevy::prelude::*;

mod calc_info;
mod car;
mod game;
mod home;
mod mode_select;
mod resources;
mod result;
mod settings;
mod setup_flow;
mod states;
mod ui;

use calc_info::CalcInfoPlugin;
use game::GamePlugin;
use home::HomePlugin;
use mode_select::ModeSelectPlugin;
use resources::{BaseCarStatus, CarStatus, PcMonitor, PcStatus};
use result::ResultPlugin;
use settings::SettingsPlugin;
use setup_flow::SetupFlowPlugin;
use states::AppState;
use ui::styles::UiStylesPlugin;

fn main() {
    App::new()
        // 1. Core Engine Plugins
        .add_plugins(DefaultPlugins)
        // 2. State & Resource Initialization
        // Initializing the application state machine
        .init_state::<AppState>()
        // Persistent hardware monitoring and car status resources
        .init_resource::<PcMonitor>()
        .init_resource::<PcStatus>()
        .init_resource::<CarStatus>()
        .init_resource::<BaseCarStatus>()
        // 3. User Interface & Screen Plugins
        .add_plugins(UiStylesPlugin)
        .add_plugins(HomePlugin)
        .add_plugins(ModeSelectPlugin)
        .add_plugins(SetupFlowPlugin)
        .add_plugins(ResultPlugin)
        .add_plugins(SettingsPlugin)
        .add_plugins(CalcInfoPlugin)
        // 4. Gameplay Logic Plugins
        .add_plugins(GamePlugin)
        .add_plugins(car::CarPlugin)
        // 5. Global Systems
        .add_systems(Startup, setup_camera)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 10.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}
