use bevy_app::{Last, Plugin};
use bevy_ecs::{schedule::Schedule, system::{NonSendMut, Res, Resource}};
use embedded_graphics::pixelcolor::Rgb555;
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window};

use crate::Display;

pub struct SimulatorPlugin;

impl Plugin for SimulatorPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        let window_title = app
            .world()
            .get_resource::<WindowTitle>()
            .unwrap_or(&WindowTitle {
                title: "DEFAULT // CONFIGURE BY SETTING WindowTitle RESOURCE",
            })
            .title;
        let output_settings = OutputSettingsBuilder::new().max_fps(2).build();
        let window = Window::new(window_title, &output_settings);
        app.insert_non_send_resource(Simulator { window });
        // TODO! USE EMBASSY TO ASYNC THIS FUNCTION! 
        // THIS FUNCTION IS SLOWING DOWN THE ENTIRE PROGRAM!
        app.add_systems(Last, update_window);
    }
}

/// If you set the wrong size of your monitor, there will be lags & bugs (e.g. unrendered objects) :(
pub struct Simulator {
    pub window: Window,
}

#[derive(Resource)]
pub struct WindowTitle {
    pub title: &'static str,
}

fn update_window(
    window_parameters: Option<NonSendMut<Simulator>>,
    display_parameters: Option<Res<Display<SimulatorDisplay<Rgb555>>>>,
) {
    let mut window_parameters = window_parameters.unwrap_or_else(|| {
        panic!("BEVY_NOSTD_RENDER>> Panic! Please init SimulatorParameters.window!")
    });
    let display_parameters = display_parameters
        .unwrap_or_else(|| panic!("BEVY_NOSTD_RENDER>> Panic! Please init DisplayParameters!"));

    let display = &display_parameters.display;

    window_parameters.window.update(display);
}
