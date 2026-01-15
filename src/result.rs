use crate::resources::{GameOverCause, GameSession};
use crate::states::AppState;
use crate::ui::styles::{
    HOVERED_BUTTON, NORMAL_BUTTON, PRESSED_BUTTON, get_button_text_color, get_button_text_font,
    get_title_text_font,
};
use bevy::prelude::*;

#[derive(Component)]
struct ResultUi;

#[derive(Component)]
struct HomeButton;

pub struct ResultPlugin;

impl Plugin for ResultPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Result), setup_result)
            .add_systems(OnExit(AppState::Result), cleanup_result)
            .add_systems(
                Update,
                interact_home_button.run_if(in_state(AppState::Result)),
            );
    }
}

fn setup_result(mut commands: Commands, asset_server: Res<AssetServer>, session: Res<GameSession>) {
    let result_text = match session.game_over_cause {
        GameOverCause::GoalReached => {
            format!("Example! Time: {:.2}s", session.play_time)
        }
        GameOverCause::FuelEmpty => "Game Over: Out of Fuel".to_string(),
        GameOverCause::Crash => "Game Over: Crashed (Course Out)".to_string(),
        GameOverCause::Overheat => "Game Over: Engine Meltdown".to_string(),
        GameOverCause::None => "Game Over: Unknown".to_string(),
    };

    let color = if session.game_over_cause == GameOverCause::GoalReached {
        Color::srgb(0.2, 0.8, 0.2)
    } else {
        Color::srgb(0.8, 0.2, 0.2)
    };

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.05, 0.05, 0.05)),
            ResultUi,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(result_text),
                get_title_text_font(&asset_server),
                TextColor(color),
            ));

            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(250.0),
                        height: Val::Px(80.0),
                        margin: UiRect::top(Val::Px(20.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(NORMAL_BUTTON),
                    HomeButton,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Back to Home"),
                        get_button_text_font(&asset_server),
                        get_button_text_color(),
                    ));
                });
        });
}

fn cleanup_result(mut commands: Commands, query: Query<Entity, With<ResultUi>>) {
    // Despawn UI
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

fn interact_home_button(
    mut query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<HomeButton>),
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
