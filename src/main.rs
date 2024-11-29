use bevy_app::{App, First};
use bevy_ecs::system::{Commands, Local};
use bevy_nostd_render::{
    std_env::WindowTitle, BevyRenderPlugin, Display, DrawableEntity, Position,
};
use embedded_graphics::{pixelcolor::Rgb555, prelude::Size};
use embedded_graphics_simulator::SimulatorDisplay;

fn main() {
    App::new()
        .add_plugins(BevyRenderPlugin)
        .add_systems(First, spawn_entity)
        .insert_resource(WindowTitle {
            title: "This is my window",
        })
        .insert_resource(Display::new(SimulatorDisplay::<Rgb555>::new(Size::new(
            1920, 1080,
        ))))
        .run();
}

fn spawn_entity(
    mut commands: Commands, 
    mut x: Local<u16>,
    mut y: Local<u16>
) {
    commands.spawn((
        Position::new(*x as i32, *y as i32),
        DrawableEntity
    ));
    if *x < 1980 {
        *x += 1;
    }
    else if *y < 1080 {
        *y += 1;
    }
    else {
        panic!("The program finished!");
    }
}
