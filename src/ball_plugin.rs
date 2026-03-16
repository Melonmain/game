use bevy::prelude::*;
use avian2d::prelude::*;

use crate::block_plugin::IsBlock;

const BALL_SPEED: f32 = 800.0;
const BALL_SIZE: f32 = 15.0;

#[derive(Component)]
struct IsBall;

#[derive(Bundle)]
struct Ball {
    is_ball: IsBall,
    rigid_body: RigidBody,
    movement: LinearVelocity,
    transform: Transform,
    collider: Collider,
    restitution: Restitution,
    collision_events_enabled: CollisionEventsEnabled,
}

impl Ball {
    pub fn new(position: [f32; 3]) -> Self {
        Self {
            is_ball: IsBall,
            rigid_body: RigidBody::Dynamic,
            movement: LinearVelocity(Vec2::new(1.0, 5.0).normalize() * BALL_SPEED),
            transform: Transform::from_xyz(position[0], position[1], position[2]),
            collider: Collider::circle(BALL_SIZE),
            restitution: Restitution::new(1.0).with_combine_rule(CoefficientCombine::Max),
            collision_events_enabled: CollisionEventsEnabled,
        }
    }
}

pub struct BallPlugin;

impl Plugin for BallPlugin
{
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
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(BALL_SIZE))),
        MeshMaterial2d(materials.add(Color::LinearRgba(LinearRgba::new(0.2, 1.0, 0.2, 1.0)))),
        Ball::new([0.0, platform_y, 1.0]),
    )).observe(delete_block_on_collision);
}

fn delete_block_on_collision(
    event: On<CollisionStart>, 
    mut commands: Commands,
    // We check for the presence of IsBlock on any entity
    block_query: Query<&IsBlock>,
) {
    // CollisionStart contains the two entities involved: .entity and .other
    // In an observer, trigger.entity() is the entity the observer is attached to.
    let entity1 = event.collider1;
    let entity2 = event.collider2; // The 'other' entity in the collision

    // Check if either entity is a block and despawn it
    if block_query.get(entity1).is_ok() {
        commands.entity(entity1).despawn();
    } else if block_query.get(entity2).is_ok() {
        commands.entity(entity2).despawn();
    }
}