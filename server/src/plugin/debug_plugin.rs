use game_core::debug_state::{DebugMode, Log};

use crate::system::debug_camera::{setup_debug_camera, update_debug_camera};
use crate::ui::console::{
    auto_scroll_console, handle_console_input, on_scroll_handler, send_scroll_events,
    setup_console_ui, update_console_display, ConsoleScrollState,
};
use crate::ui::menu::{handle_mode_switch, setup_menu};
use bevy::prelude::{in_state, App, AppExtStates, IntoScheduleConfigs, Plugin, Startup, Update};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<DebugMode>()
            .init_resource::<ConsoleScrollState>()
            .insert_resource(Log::new(1000))
            .add_systems(Startup, (setup_menu, setup_console_ui, setup_debug_camera))
            .add_systems(
                Update,
                (
                    handle_mode_switch,
                    send_scroll_events.run_if(in_state(DebugMode::Console)),
                    update_console_display.run_if(in_state(DebugMode::Console)),
                    auto_scroll_console.run_if(in_state(DebugMode::Console)),
                    handle_console_input.run_if(in_state(DebugMode::Console)),
                    update_debug_camera.run_if(in_state(DebugMode::Camera)),
                ),
            )
            .add_observer(on_scroll_handler);
    }
}
