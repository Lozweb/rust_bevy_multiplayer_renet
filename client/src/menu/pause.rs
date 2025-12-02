use crate::menu::{quit_to_title, Menu};
use crate::theme::widget;
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    crate::menu::setup_menu(app, Menu::Pause, spawn_pause_menu);
}

fn spawn_pause_menu(mut commands: Commands) {
    commands.spawn((
        widget::ui_root("Pause Menu"),
        GlobalZIndex(2),
        DespawnOnExit(Menu::Pause),
        children![
            widget::header("Game paused"),
            widget::button("Continue", close_menu),
            widget::button("Settings", open_settings_menu),
            widget::button("Exit", quit_to_title),
        ],
    ));
}
fn open_settings_menu(_: On<Pointer<Click>>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Settings);
}

fn close_menu(_: On<Pointer<Click>>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::None);
}
