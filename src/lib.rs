#![cfg_attr(not(feature = "std"), no_std)]

use crate::embedded_graphics::Drawable;
use bevy_app::{Last, Plugin};
use bevy_ecs::{
    component::Component,
    query::With,
    system::{NonSendMut, Query, ResMut, Resource},
};
use embedded_graphics::{
    pixelcolor::Rgb555,
    prelude::{Dimensions, DrawTarget, Point, Primitive, RgbColor, Size},
    primitives::{PrimitiveStyle, Rectangle},
};
use embedded_graphics_simulator::SimulatorDisplay;
use std_env::{Simulator, SimulatorPlugin};

extern crate embedded_graphics;

pub mod std_env;

pub struct BevyRenderPlugin;

impl Plugin for BevyRenderPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        app.set_runner(|mut app| loop {
            app.update();
            if let Some(exit) = app.should_exit() {
                return exit;
            }
        });
        app.add_plugins(SimulatorPlugin);
        app.add_systems(Last, render);
    }
}

#[derive(Component)]
pub struct DrawableEntity;

#[derive(Component)]
pub struct Position {
    x: i32,
    y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    pub fn set_xy(&mut self, x: i32, y: i32) {
        *self = Position { x, y };
    }
}
#[derive(Resource)]
pub struct Display<T>
where
    T: DrawTarget,
{
    display: T,
}

impl<T> Display<T>
where
    T: DrawTarget,
{
    pub fn new(display: T) -> Self {
        Self { display }
    }
}

fn render(
    display_parameters: Option<ResMut<Display<SimulatorDisplay<Rgb555>>>>,
    entities: Query<&Position, With<DrawableEntity>>,
    window_parameters: Option<NonSendMut<Simulator>>,
) {
    let mut display_parameters = display_parameters
        .unwrap_or_else(|| panic!("BEVY_NOSTD_RENDER>> Panic! Please init DisplayParameters!"));
    let mut window_parameters = window_parameters.unwrap_or_else(|| {
        panic!("BEVY_NOSTD_RENDER>> Panic! Please init SimulatorParameters.window!")
    });
    let dimensions = display_parameters.display.bounding_box();
    let (width, heigh) = (dimensions.size.width as i32, dimensions.size.height as i32);

    display_parameters
        .display
        .clear(Rgb555::BLUE)
        .expect("Unable to clear the screen");

    for entity in entities.iter() {
        if entity.x < 0 || entity.x > width || entity.y < 0 || entity.y > heigh {
            #[cfg(feature = "std")]
            eprintln!(
                "Entity (x: {}, y: {}) exceeds display size!",
                entity.x, entity.y
            );
            continue;
        }

        // Draw a rectangle.
        let style = PrimitiveStyle::with_fill(Rgb555::BLACK);
        let status = Rectangle::new(Point::new(entity.x, entity.y), Size::new(2, 2))
            .into_styled(style)
            .draw(&mut display_parameters.display);

        if status.is_ok() {
            #[cfg(feature = "std")]
            println!(
                "Drawn a rectangle for entity (x: {}, y: {})!",
                entity.x, entity.y
            );
        } else {
            #[cfg(feature = "std")]
            eprintln!(
                "Unable to draw a rectangle for entity (x: {}, y: {})!",
                entity.x, entity.y
            );
        }
    }
    window_parameters.window.update(&display_parameters.display);
}

