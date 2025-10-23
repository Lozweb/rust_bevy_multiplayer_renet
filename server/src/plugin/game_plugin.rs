use crate::system::game_event::on_game_event;
use bevy::prelude::{App, Plugin, Update};
use game_core::event::game_event::GameEvent;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<GameEvent>();

        app.add_systems(Update, on_game_event);
    }
}
