mod gameplay;
mod loading;
mod splash;
mod title;

use crate::client;
use bevy::app::App;
use bevy::prelude::{AppExtStates, States};

pub(super) fn plugin(app: &mut App) {
    app.init_state::<Screen>();
    app.add_plugins((
        gameplay::plugin,
        client::plugin,
        loading::plugin,
        splash::plugin,
        title::plugin,
    ));
}

#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub enum Screen {
    #[default]
    Splash,
    Title,
    Loading,
    Gameplay,
}
