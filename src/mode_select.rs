use crate::states::AppState;
use crate::ui::styles::{
    HOVERED_BUTTON, NORMAL_BUTTON, PRESSED_BUTTON, get_button_text_color, get_button_text_font,
    get_title_text_color, get_title_text_font,
};
use bevy::prelude::*;

#[derive(Component)]
struct ModeSelectUi;

#[derive(Component)]
enum ModeButton {
    TimeAttack,
    Race,
    Back,
}

pub struct ModeSelectPlugin;

impl Plugin for ModeSelectPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::ModeSelect), setup_mode_select)
            .add_systems(OnExit(AppState::ModeSelect), cleanup_mode_select)
            .add_systems(
                Update,
                interact_mode_buttons.run_if(in_state(AppState::ModeSelect)),
            );
    }
}

fn setup_mode_select(mut commands: Commands, asset_server: Res<AssetServer>) {
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
            ModeSelectUi,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Select Mode"),
                get_title_text_font(&asset_server),
                get_title_text_color(),
            ));

            let button_node = Node {
                width: Val::Px(250.0),
                height: Val::Px(80.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::all(Val::Px(10.0)),
                ..default()
            };

            // Time Attack Button
            parent
                .spawn((
                    Button,
                    button_node.clone(),
                    BackgroundColor(NORMAL_BUTTON),
                    ModeButton::TimeAttack,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Time Attack"),
                        get_button_text_font(&asset_server),
                        get_button_text_color(),
                    ));
                });

            // Race Mode Button
            parent
                .spawn((
                    Button,
                    button_node.clone(),
                    BackgroundColor(NORMAL_BUTTON),
                    ModeButton::Race,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Race Mode"),
                        get_button_text_font(&asset_server),
                        get_button_text_color(),
                    ));
                });

            // Back Button
            parent
                .spawn((
                    Button,
                    button_node,
                    BackgroundColor(NORMAL_BUTTON),
                    ModeButton::Back,
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

fn cleanup_mode_select(mut commands: Commands, query: Query<Entity, With<ModeSelectUi>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

fn interact_mode_buttons(
    mut query: Query<
        (&Interaction, &mut BackgroundColor, &ModeButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for (interaction, mut color, button_type) in &mut query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                match button_type {
                    ModeButton::TimeAttack => {
                        println!("Time Attack Selected");
                        next_state.set(AppState::CourseSelect);
                    }
                    ModeButton::Race => {
                        println!("Race Mode Selected (Not Implemented)");
                    }
                    ModeButton::Back => {
                        next_state.set(AppState::Home);
                    }
                }
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}
