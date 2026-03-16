use avian2d::prelude::*;
use bevy::{ecs::spawn, prelude::*};

use crate::{
    block_plugin::IsBlock,
    powerup_plugin::{Powerup, spawn_new_balls_powerup_on_collision, spawn_powerup},
};
use rand::Rng;

const BALL_SPEED: f32 = 800.0;
const BALL_SIZE: f32 = 15.0;

#[derive(Component)]
pub struct IsBall;

#[derive(Bundle)]
pub struct Ball {
    is_ball: IsBall,
    rigid_body: RigidBody,
    movement: LinearVelocity,
    transform: Transform,
    collider: Collider,
    restitution: Restitution,
    collision_events_enabled: CollisionEventsEnabled,
    friction: Friction,
    damping: LinearDamping,
    collission_layer: CollisionLayers,
}

impl Ball {
    pub fn new(position: [f32; 3], velocity: Vec2) -> Self {
        Self {
            is_ball: IsBall,
            rigid_body: RigidBody::Dynamic,
            movement: LinearVelocity(velocity.normalize() * BALL_SPEED),
            transform: Transform::from_xyz(position[0], position[1], position[2]),
            collider: Collider::circle(BALL_SIZE),
            restitution: Restitution::new(1.0).with_combine_rule(CoefficientCombine::Max),
            collision_events_enabled: CollisionEventsEnabled,
            friction: Friction::ZERO,
            damping: LinearDamping(0.0),
            collission_layer: CollisionLayers::new(0b0010, 0b0011),
        }
    }
}

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, setup);
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
    spawn_ball(Vec3::new(0.0, 0.0, 0.0), &mut commands, &mut meshes, &mut materials);
}

pub fn spawn_ball(
    position: Vec3,
    mut commands: &mut Commands,
    mut meshes: &mut Assets<Mesh>,
    mut materials: &mut Assets<ColorMaterial>,
) {
    let mut rng = rand::thread_rng();
    let velocity: Vec2 = Vec2::new(rng.gen_range(-1.0..=1.0), rng.gen_range(-1.0..=1.0));

    commands.spawn((
        Mesh2d(meshes.add(Circle::new(BALL_SIZE))),
        MeshMaterial2d(materials.add(Color::LinearRgba(LinearRgba::new(0.2, 1.0, 0.2, 1.0)))),
        Ball::new([position.x, position.y, position.z], velocity),
    )).observe(delete_block_on_collision);
}

fn delete_block_on_collision(
    event: On<CollisionStart>,
    mut commands: Commands,
    block_query: Query<(&IsBlock, &Transform)>, // Query for both components
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) 
{
    let entity2 = event.collider2;
    if let Ok((_block, transform)) = block_query.get(entity2) {
        let mut rng = rand::thread_rng();
        let n: u32 = rng.gen_range(0..=100);

        if n < 10 {
            let pos = transform.translation;
            spawn_powerup(pos, &mut commands, &mut meshes, &mut materials);
        }

        commands.entity(entity2).despawn();
    }
}
