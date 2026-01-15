use bevy::prelude::*;

pub const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

pub fn get_button_text_font(asset_server: &AssetServer) -> TextFont {
    TextFont {
        font: asset_server.load("fonts/NotoSansJP-Bold.ttf"),
        font_size: 40.0,
        ..default()
    }
}

pub fn get_button_text_color() -> TextColor {
    TextColor(Color::srgb(0.9, 0.9, 0.9))
}

pub fn get_title_text_font(asset_server: &AssetServer) -> TextFont {
    TextFont {
        font: asset_server.load("fonts/NotoSansJP-Bold.ttf"),
        font_size: 80.0,
        ..default()
    }
}

pub fn get_title_text_color() -> TextColor {
    TextColor(Color::WHITE)
}

pub struct UiStylesPlugin;

impl Plugin for UiStylesPlugin {
    fn build(&self, _app: &mut App) {}
}
