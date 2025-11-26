mod main;
mod pause;
mod settings;

use bevy::app::App;
use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::{
    in_state, AppExtStates, IntoScheduleConfigs, IntoSystem, KeyCode, NextState, OnEnter, ResMut,
    States, SystemCondition, Update,
};

pub(super) fn plugin(app: &mut App) {
    app.init_state::<Menu>();
    app.add_plugins((main::plugin, pause::plugin, settings::plugin));
}

#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub enum Menu {
    #[default]
    None,
    Main,
    Settings,
    Pause,
}

fn setup_menu<Marker>(
    app: &mut App,
    menu_state: Menu,
    spawn_fn: impl IntoSystem<(), (), Marker> + Send + Sync + 'static,
) {
    app.add_systems(OnEnter(menu_state), spawn_fn);
    app.add_systems(
        Update,
        (move |mut next_menu: ResMut<NextState<Menu>>| {
            next_menu.set(Menu::None);
        })
        .run_if(in_state(menu_state).and(input_just_pressed(KeyCode::Escape))),
    );
}
