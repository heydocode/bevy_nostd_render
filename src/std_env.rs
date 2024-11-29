use bevy_app::Plugin;
use bevy_ecs::system::Resource;
use embedded_graphics_simulator::{OutputSettings, Window};

pub struct SimulatorPlugin;

impl Plugin for SimulatorPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        let window_title = app
            .world()
            .get_resource::<WindowTitle>()
            .unwrap_or_else(|| &WindowTitle {
                title: "DEFAULT // CONFIGURE BY SETTING WindowTitle RESOURCE",
            })
            .title;
        let window = Window::new(window_title, &OutputSettings::default());
        app.insert_non_send_resource(Simulator { window });
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
