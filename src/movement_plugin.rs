use bevy::prelude::*;

use crate::collider_plugin::Collider;

#[derive(Component)]
pub struct Velocity {
    pub velocity: Vec3,
}

#[derive(Component)]
pub struct Acceleration {
    pub acceleration: Vec3,
}

#[derive(Component)]
pub struct Speed(pub f32);


#[derive(Component)]
pub struct IsLinear;

#[derive(Bundle)]
pub struct LinearMovement {
    velocity: Velocity,
    speed: Speed,
    marker: IsLinear,
}

impl LinearMovement {
    pub fn new(velocity: Vec3, speed: f32) -> Self {
        Self {
            velocity: Velocity { velocity: velocity.normalize() },
            speed: Speed(speed),
            marker: IsLinear,
        }
    }
}

#[derive(Bundle)]
pub struct Movement {
    velocity: Velocity,
    accelleration: Acceleration,
}

#[derive(Component)]
pub struct BounceOnBorder;

#[derive(Component)]
pub struct ClampToBorder;

pub struct MovementPlugin;

impl Plugin for MovementPlugin
{
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_linear);
        app.add_systems(Update, (apply_velocity, move_movement, bounce_on_borders).chain());
    }
}

fn move_linear(
    time: Res<Time>,
    objects: Query<(&mut Transform, &Speed, &Velocity), With<IsLinear>>
) {
    for (mut transform, speed, velocity) in objects {
        transform.translation += velocity.velocity * speed.0 * time.delta_secs();
    }
}

fn apply_velocity() {

}

fn move_movement() {

}

fn bounce_on_borders(
    window_query: Query<&Window>,
    objects: Query<(&mut Transform, &mut Velocity, &Collider), With<BounceOnBorder>>
) {
    let window = match window_query.single() {
        Ok(w) => w,
        Err(_) => return,
    };
    for (mut transform, mut velocity, collider) in objects {
        let mut position: Vec3 = transform.translation;
        if window.width() / 2.0 - position.x.abs() - collider.width < 0.0 {
            velocity.velocity.x = -velocity.velocity.x;
            let width = window.width() / 2.0 - collider.width;
            position.x = position.x.clamp(-width, width);
        }
        if window.height() / 2.0 - position.y.abs() - collider.height < 0.0 {
            velocity.velocity.y = -velocity.velocity.y;
            let height = window.height() / 2.0 - collider.height;
            position.y = position.y.clamp(-height, height);
        }
        transform.translation = position;
    }
}