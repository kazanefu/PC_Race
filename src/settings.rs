use crate::states::AppState;
use crate::ui::styles::{
    HOVERED_BUTTON, NORMAL_BUTTON, PRESSED_BUTTON, get_button_text_color, get_button_text_font,
    get_title_text_color, get_title_text_font,
};
use bevy::prelude::*;

#[derive(Component)]
struct SettingsUi;

#[derive(Component)]
struct BackButton;

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Settings), setup_settings)
            .add_systems(OnExit(AppState::Settings), cleanup_settings)
            .add_systems(
                Update,
                interact_settings.run_if(in_state(AppState::Settings)),
            );
    }
}

fn setup_settings(mut commands: Commands, asset_server: Res<AssetServer>) {
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
            BackgroundColor(Color::srgb(0.05, 0.05, 0.05)),
            SettingsUi,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Settings"),
                get_title_text_font(&asset_server),
                get_title_text_color(),
            ));
            parent.spawn((
                Text::new("(Settings Not Imeplented Yet - Stub)"),
                get_button_text_font(&asset_server),
                TextColor(Color::WHITE),
            ));

            // Back Button
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(200.0),
                        height: Val::Px(60.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(NORMAL_BUTTON),
                    BackButton,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Back"),
                        get_button_text_font(&asset_server),
                        get_button_text_color(),
                    ));
                });
        });
}

fn cleanup_settings(mut commands: Commands, query: Query<Entity, With<SettingsUi>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

fn interact_settings(
    mut query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<BackButton>),
    >,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for (interaction, mut color) in &mut query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                next_state.set(AppState::Home);
            }
            Interaction::Hovered => *color = HOVERED_BUTTON.into(),
            Interaction::None => *color = NORMAL_BUTTON.into(),
        }
    }
}
