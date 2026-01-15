use crate::states::AppState;
use crate::ui::styles::{
    HOVERED_BUTTON, NORMAL_BUTTON, PRESSED_BUTTON, get_button_text_color, get_button_text_font,
    get_title_text_color, get_title_text_font,
};
use bevy::prelude::*;

#[derive(Component)]
struct HomeUi;

#[derive(Component)]
struct StartButton;

#[derive(Component)]
struct SettingsButton;

#[derive(Component)]
struct CalcButton;

#[derive(Component)]
struct ExitButton;

pub struct HomePlugin;

impl Plugin for HomePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Home), setup_home)
            .add_systems(OnExit(AppState::Home), cleanup_home)
            .add_systems(
                Update,
                interact_home_buttons.run_if(in_state(AppState::Home)),
            );
    }
}

fn setup_home(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                row_gap: Val::Px(20.0),
                ..default()
            },
            BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
            HomeUi,
        ))
        .with_children(|parent| {
            // Title
            parent.spawn((
                Text::new("PC Racing Game"),
                get_title_text_font(&asset_server),
                get_title_text_color(),
            ));

            // Start Game Button
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(220.0),
                        height: Val::Px(60.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(NORMAL_BUTTON),
                    StartButton,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Start Game"),
                        get_button_text_font(&asset_server),
                        get_button_text_color(),
                    ));
                });

            // Settings Button
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(220.0),
                        height: Val::Px(60.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(NORMAL_BUTTON),
                    SettingsButton,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Settings"),
                        get_button_text_font(&asset_server),
                        get_button_text_color(),
                    ));
                });

            // Calculation Method Button
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(220.0),
                        height: Val::Px(60.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(NORMAL_BUTTON),
                    CalcButton,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Calculations"),
                        get_button_text_font(&asset_server),
                        get_button_text_color(),
                    ));
                });

            // Exit Button
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(220.0),
                        height: Val::Px(60.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(NORMAL_BUTTON),
                    ExitButton,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Exit (Esc)"),
                        get_button_text_font(&asset_server),
                        get_button_text_color(),
                    ));
                });
        });
}

fn cleanup_home(mut commands: Commands, query: Query<Entity, With<HomeUi>>) {
    // Despawn UI
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

#[allow(clippy::type_complexity)]
fn interact_home_buttons(
    mut next_state: ResMut<NextState<AppState>>,
    mut queries: ParamSet<(
        Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<StartButton>)>,
        Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<SettingsButton>)>,
        Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<CalcButton>)>,
        Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<ExitButton>)>,
    )>,
) {
    // Start
    for (interaction, mut color) in queries.p0().iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                next_state.set(AppState::ModeSelect);
            }
            Interaction::Hovered => *color = HOVERED_BUTTON.into(),
            Interaction::None => *color = NORMAL_BUTTON.into(),
        }
    }
    // Settings
    for (interaction, mut color) in queries.p1().iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                println!("Go to Settings");
                next_state.set(AppState::Settings);
            }
            Interaction::Hovered => *color = HOVERED_BUTTON.into(),
            Interaction::None => *color = NORMAL_BUTTON.into(),
        }
    }
    // Calc
    for (interaction, mut color) in queries.p2().iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                println!("Go to Calculations");
                next_state.set(AppState::CalcInfo);
            }
            Interaction::Hovered => *color = HOVERED_BUTTON.into(),
            Interaction::None => *color = NORMAL_BUTTON.into(),
        }
    }
    // Exit
    for (interaction, mut color) in queries.p3().iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                // exit.send(AppExit::Success);
                println!("Exit Pressed (Logic disabled for build fix)");
            }
            Interaction::Hovered => *color = HOVERED_BUTTON.into(),
            Interaction::None => *color = NORMAL_BUTTON.into(),
        }
    }
}
