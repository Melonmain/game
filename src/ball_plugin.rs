use bevy::prelude::*;

use crate::{collider_plugin::{Collider, ColliderTypes}, movement_plugin::{BounceOnBorder, LinearMovement}};

const BALL_SPEED: f32 = 800.0;
const BALL_SIZE: f32 = 15.0;

#[derive(Component)]
struct IsBall;

#[derive(Bundle)]
struct Ball {
    is_ball: IsBall,
    bounce_on_order: BounceOnBorder,
    movement: LinearMovement,
    transform: Transform,
    collider: Collider
}

impl Ball {
    pub fn new(position: [f32; 3]) -> Self {
        Self {
            is_ball: IsBall,
            bounce_on_order: BounceOnBorder,
            movement: LinearMovement::new(Vec3::from_array([0.5, 1.5, 0.]), BALL_SPEED),
            transform: Transform::from_xyz(position[0], position[1], position[2]),
            collider: Collider::new(ColliderTypes::Circle, BALL_SIZE, BALL_SIZE),
        }
    }
}

pub struct BallPlugin;

impl Plugin for BallPlugin
{
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, setup);
        app.add_systems(Update, check_collissions);
    }
}

fn setup(
    mut commands: Commands,
    window_query: Query<&Window>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let window = match window_query.single() {
        Ok(w) => w,
        Err(_) => return,
    };

    let platform_y = -window.height() / 2.5;
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(BALL_SIZE))),
        MeshMaterial2d(materials.add(Color::LinearRgba(LinearRgba::new(0.2, 1.0, 0.2, 1.0)))),
        Ball::new([0.0, platform_y, 1.0]),
    ));
}

fn check_collissions() {

}