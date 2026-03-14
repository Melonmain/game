use bevy::prelude::*;

use crate::{collider_plugin::{Collider, ColliderTypes}, movement_plugin::{BounceOnBorder, LinearMovement, Velocity}};

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
        app.add_systems(Update, check_collisions);
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

fn check_collisions(
    mut commands: Commands,
    mut balls: Query<(&Transform, &Collider, &mut Velocity), With<IsBall>>,
    objects: Query<(Entity, &Transform, &Collider), Without<IsBall>>,
) {
    for (ball_transform, ball_collider, mut ball_velocity_obj) in &mut balls {
        let ball_pos = ball_transform.translation.truncate(); // Use Vec2 for 2D math
        let radius = ball_collider.width; // "width" is our radius
        let mut ball_velocity = ball_velocity_obj.velocity;

        for (object_entity, object_transform, object_collider) in &objects {
            let obj_pos = object_transform.translation.truncate();

            if object_collider.collider_type == ColliderTypes::Circle {
                // --- Circle vs Circle ---
                let distance = ball_pos.distance(obj_pos);
                if distance < radius + object_collider.width {
                    // Simple reflection: get the direction from object to ball
                    let normal = (ball_pos - obj_pos).normalize_or_zero();
                    
                    // Reflect velocity based on the collision normal
                    // v = v - 2 * (v.dot(n)) * n
                    let dot = ball_velocity.x * normal.x + ball_velocity.y * normal.y;
                    ball_velocity.x -= 2.0 * dot * normal.x;
                    ball_velocity.y -= 2.0 * dot * normal.y;

                    ball_velocity_obj.velocity = ball_velocity.normalize();
                    commands.entity(object_entity).despawn();
                    break;
                }
            } else if object_collider.collider_type == ColliderTypes::Rectangle {
                // --- Circle vs Rectangle ---
                let half_width = object_collider.width / 2.0;
                let half_height = object_collider.height / 2.0;

                // 1. Find the closest point on the rectangle to the ball
                let closest_point = Vec2::new(
                    ball_pos.x.clamp(obj_pos.x - half_width, obj_pos.x + half_width),
                    ball_pos.y.clamp(obj_pos.y - half_height, obj_pos.y + half_height),
                );

                // 2. Check distance between ball center and that closest point
                let collision_vector = ball_pos - closest_point;
                let distance_sq = collision_vector.length_squared();

                if distance_sq < radius.powi(2) {
                    // 3. Determine which axis to bounce
                    // We compare the difference on each axis from the closest point
                    // If the x-difference is greater, we hit a side. If y is greater, we hit top/bottom.
                    let diff = (ball_pos - closest_point).abs();

                    if diff.x > diff.y {
                        ball_velocity.x *= -1.0;
                    } else {
                        ball_velocity.y *= -1.0;
                    }
                    ball_velocity_obj.velocity = ball_velocity.normalize();
                    commands.entity(object_entity).despawn();
                    break;
                }
            }
        }
    }
}