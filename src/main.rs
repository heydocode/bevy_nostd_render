use bevy_app::{App, Startup};
use bevy_ecs::system::Commands;
use bevy_nostd_render::{std_env::WindowTitle, BevyRenderPlugin, Display, DrawableEntity, Position};
use embedded_graphics::{pixelcolor::Rgb555, prelude::Size};
use embedded_graphics_simulator::SimulatorDisplay;

fn main() {
    App::new()
        .add_plugins(BevyRenderPlugin)
        .add_systems(Startup, spawn_entity)
        .insert_resource(WindowTitle {
            title: "This is my window",
        })
        .insert_resource(Display::new(
            SimulatorDisplay::<Rgb555>::new(Size::new(1920, 1080))
        ))
        .run();
}

fn spawn_entity(mut commands: Commands) {
    commands.spawn((Position::new(12, 92), DrawableEntity));

    commands.spawn((Position::new(900, 192), DrawableEntity));
}
