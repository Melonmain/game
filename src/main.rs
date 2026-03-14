#![allow(unused)]

use bevy::{prelude::*};

mod ball_plugin;
mod movement_plugin;
mod block_plugin;
mod platform_plugin;
mod collider_plugin;

use ball_plugin::BallPlugin;
use movement_plugin::MovementPlugin;
use block_plugin::BlockPlugin;
use platform_plugin::PlatformPlugin;
use collider_plugin::ColliderPlugin;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(BallPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(BlockPlugin)
        .add_plugins(PlatformPlugin)
        .add_plugins(ColliderPlugin)
        .insert_resource(Time::<Fixed>::from_hz(144.0))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
) {
    commands.spawn(Camera2d);
}