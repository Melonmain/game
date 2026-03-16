#![allow(unused)]

use bevy::{prelude::*};
use avian2d::prelude::*;

mod ball_plugin;
mod block_plugin;
mod platform_plugin;

use ball_plugin::BallPlugin;
use block_plugin::BlockPlugin;
use platform_plugin::PlatformPlugin;


fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PhysicsPlugins::default()))
        .add_plugins(BallPlugin)
        .add_plugins(BlockPlugin)
        .add_plugins(PlatformPlugin)
        .insert_resource(Time::<Fixed>::from_hz(144.0))
        .add_systems(Startup, setup)
        .add_systems(Startup, spawn_screen_walls)
        .run();
}

fn setup(
    mut commands: Commands,
) {
    commands.spawn(Camera2d);
}

fn spawn_screen_walls(
    mut commands: Commands,
    window_query: Query<&Window>,
) {
    let window = match window_query.single() {
        Ok(w) => w,
        Err(_) => return,
    };
    let width = window.width();
    let height = window.height();
    let half_width = width / 2.0;
    let half_height = height / 2.0;

    // A shared bundle for all walls
    // Static bodies do not move and have infinite mass
    let wall_physics = (
        RigidBody::Static,
        Restitution::PERFECTLY_ELASTIC, // Bounce without losing energy
        Friction::ZERO,                // Prevent the ball from "grabbing" the wall
    );

    // Top Wall
    commands.spawn((
        wall_physics,
        Collider::rectangle(width, 10.0),
        Transform::from_xyz(0.0, half_height, 0.0),
    ));

    // Bottom Wall (You might want to skip this if the ball should fall out)
    commands.spawn((
        wall_physics,
        Collider::rectangle(width, 10.0),
        Transform::from_xyz(0.0, -half_height, 0.0),
    ));

    // Left Wall
    commands.spawn((
        wall_physics,
        Collider::rectangle(10.0, height),
        Transform::from_xyz(-half_width, 0.0, 0.0),
    ));

    // Right Wall
    commands.spawn((
        wall_physics,
        Collider::rectangle(10.0, height),
        Transform::from_xyz(half_width, 0.0, 0.0),
    ));
}
