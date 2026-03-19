use bevy::{platform, prelude::*};
use avian2d::prelude::*;

use crate::{ball_plugin::{IsBall, spawn_ball}, platform_plugin::IsPlatform};

const POWERUP_SPEED: f32 = 500.0;
const POWERUP_SIZE: f32 = 12.0;

#[derive(Component)]
pub struct IsPowerup;

#[derive(Bundle)]
pub struct Powerup {
    is_powerup: IsPowerup,
    transform: Transform,
    rigid_body: RigidBody,
    collider: Collider,
    movement: LinearVelocity,
    collision_events_enabled: CollisionEventsEnabled,
    collission_layer: CollisionLayers,
}

impl Powerup {
    pub fn new(position: Vec3) -> Self {
        Self {
            is_powerup: IsPowerup,
            transform: Transform::from_xyz(position.x, position.y, position.z),
            rigid_body: RigidBody::Kinematic,
            collider: Collider::circle(POWERUP_SIZE),
            movement: LinearVelocity(Vec2::new(0.0, -1.0) * POWERUP_SPEED),
            collision_events_enabled: CollisionEventsEnabled::default(),
            collission_layer: CollisionLayers::new(0b0100, 0b0101),
        }
    }
}

pub fn spawn_powerup(
    position: Vec3,
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
) {
    commands.spawn((
        Powerup::new(position),
        Mesh2d(meshes.add(Circle::new(POWERUP_SIZE))),
        MeshMaterial2d(materials.add(Color::LinearRgba(LinearRgba::new(0.2, 0.1, 1.0, 1.0))))
    )).observe(spawn_new_balls_powerup_on_collision);
    
}

pub fn spawn_new_balls_powerup_on_collision(event: On<CollisionStart>, mut commands: Commands, platform_query: Query<(&IsPlatform)>, ball_query: Query<(&IsBall, &Transform)>, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
     let powerup_entity = event.collider2;
     commands.entity(event.collider1).despawn();
     let is_powerup_collision = platform_query.get(powerup_entity).is_ok();

     if is_powerup_collision {
         for (_, ball) in ball_query{
             let ball_transform = ball.translation;
             spawn_ball(Vec3::new(ball_transform.x, ball_transform.y, 0.0), &mut commands, &mut meshes, &mut materials);
         }
     }
}