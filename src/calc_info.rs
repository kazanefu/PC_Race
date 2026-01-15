use crate::states::AppState;
use crate::ui::styles::{
    HOVERED_BUTTON, NORMAL_BUTTON, PRESSED_BUTTON, get_button_text_color, get_button_text_font,
    get_title_text_color, get_title_text_font,
};
use bevy::prelude::*;

#[derive(Component)]
struct CalcUi;

#[derive(Component)]
struct BackButton;

pub struct CalcInfoPlugin;

impl Plugin for CalcInfoPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::CalcInfo), setup_calc_info)
            .add_systems(OnExit(AppState::CalcInfo), cleanup_calc_info)
            .add_systems(Update, interact_calc.run_if(in_state(AppState::CalcInfo)));
    }
}

fn setup_calc_info(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                row_gap: Val::Px(10.0),
                ..default()
            },
            BackgroundColor(Color::BLACK),
            CalcUi,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Calculation Formula Specification"),
                get_title_text_font(&asset_server),
                get_title_text_color(),
            ));

            let small_font = TextFont {
                font: asset_server.load("fonts/NotoSansJP-Bold.ttf"),
                font_size: 14.0,
                ..default()
            };
            let text_color = TextColor(Color::WHITE);

            let lines = vec![
                "Max Speed = Base * (1 + CPU Impact * CPU Clock * (1 + usage))",
                "Fuel = Base * (1 + RAM Impact * RAM Used)",
                "Acceleration = Base * ((1 + GPU Impact * Clock) / Weight)",
                "Handling = Base * Grip / Weight",
            ];

            for line in lines {
                parent.spawn((Text::new(line), small_font.clone(), text_color));
            }

            // Back Button
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(200.0),
                        height: Val::Px(50.0),
                        margin: UiRect::top(Val::Px(20.0)),
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

fn cleanup_calc_info(mut commands: Commands, query: Query<Entity, With<CalcUi>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

fn interact_calc(
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
