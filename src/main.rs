use bevy::{ecs::bundle, math::ops, prelude::*, time::*, transform};

const BLOCK_HEIGHT: f32 = 50.0;
const BLOCK_WIDTH: f32 = 100.0;
const BLOCK_SPACER_VERTICAL: f32 = 5.0;
const BLOCK_SPACER_HORIZONTAL: f32 = 5.0;
const BLOCK_SPACER_LEFT: f32 = 50.0;
const BLOCK_SPACER_TOP: f32 = 15.0;
const DEFAULT_ROWS_OF_BLOCKS: i8 = 5;
const BLOCK_DELAY: f32 = 15.0;

const PLATFORM_SPEED: f32 = 850.0;
const PLATFORM_WIDTH: f32 = 250.0;
const PLATFORM_HEIGHT: f32 = 30.0;
const PLATFORM_POSITION_Y: f32 = 7.0 / 8.0;

const BALL_SPEED: f32 = 2800.0;
const BALL_SIZE: f32 = 15.0;

#[derive(Component)]
struct Block;

#[derive(Component)]
struct Ball;

#[derive(Component)]
struct Platform;

#[derive(Resource)]
struct BlockAdderTimer(Timer);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Time::<Fixed>::from_hz(144.0))
        .insert_resource(BlockAdderTimer(Timer::from_seconds(
            BLOCK_DELAY,
            TimerMode::Repeating,
        )))
        .add_systems(Startup, setup)
        .add_systems(
            FixedUpdate,
            (
                spawn_initial_blocks,
                spawn_new_blockrow,
                move_platform,
                move_ball,
                check_collisions,
            )
                .chain(),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    window_query: Query<&Window>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    let window = match window_query.single() {
        Ok(w) => w,
        Err(_) => return, // shouldn't happen but bail if no window yet
    };

    let platform_y = -window.height() * PLATFORM_POSITION_Y / 2.0;
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(PLATFORM_WIDTH, PLATFORM_HEIGHT))),
        MeshMaterial2d(materials.add(Color::WHITE)),
        Platform,
        Transform::from_xyz(0.0, platform_y, 0.0),
    ));

    commands.spawn((
        Mesh2d(meshes.add(Circle::new(BALL_SIZE))),
        MeshMaterial2d(materials.add(Color::LinearRgba(LinearRgba::new(0.2, 1.0, 0.2, 1.0)))),
        Ball,
        Transform::from_xyz(0.0, platform_y, 0.0)
            .with_rotation(Quat::from_rotation_z(0.5).normalize()),
    ));
}

fn spawn_block_row(
    window: &Window,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    y: f32,
) -> Vec<(Mesh2d, MeshMaterial2d<ColorMaterial>, Block, Transform)> {
    let number_of_horizontal_bars: u8 =
        ((window.width() - 2.0 * BLOCK_SPACER_LEFT) / BLOCK_WIDTH) as u8;
    let left_position = -window.width() / 2.0 + BLOCK_SPACER_LEFT + BLOCK_WIDTH / 2.0;
    let mut vec: Vec<(Mesh2d, MeshMaterial2d<ColorMaterial>, Block, Transform)> = Vec::new();
    for pos in 0..number_of_horizontal_bars {
        let x = left_position + (BLOCK_WIDTH + BLOCK_SPACER_HORIZONTAL) * pos as f32;
        vec.push(spawn_block(meshes, materials, x, y));
    }
    vec
}

fn spawn_block(
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    x: f32,
    y: f32,
) -> (Mesh2d, MeshMaterial2d<ColorMaterial>, Block, Transform) {
    (
        Mesh2d(meshes.add(Rectangle::new(BLOCK_WIDTH, BLOCK_HEIGHT))),
        MeshMaterial2d(materials.add(Color::LinearRgba(LinearRgba::new(1.0, 0.0, 0.0, 1.0)))),
        Block,
        Transform::from_xyz(x, y, 0.0),
    )
}

fn spawn_initial_blocks(
    mut commands: Commands,
    window_query: Query<&Window>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    blocks: Query<&Block>,
) {
    if blocks.is_empty() {
        let window = match window_query.single() {
            Ok(w) => w,
            Err(_) => return,
        };

        let top_position = window.height() / 2.0;
        for row in 0..DEFAULT_ROWS_OF_BLOCKS {
            let y = top_position
                - (BLOCK_HEIGHT + BLOCK_SPACER_VERTICAL) * row as f32
                - BLOCK_HEIGHT / 2.0
                - BLOCK_SPACER_TOP;
            commands.spawn_batch(spawn_block_row(&window, &mut meshes, &mut materials, y))
        }
    }
}

fn spawn_new_blockrow(
    mut commands: Commands,
    window_query: Query<&Window>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    time: Res<Time>,
    mut timer: ResMut<BlockAdderTimer>,
    mut blocks: Query<&mut Transform, With<Block>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        for mut block in &mut blocks {
            block.translation.y -= BLOCK_HEIGHT + BLOCK_SPACER_VERTICAL;
        }
        let window = match window_query.single() {
            Ok(w) => w,
            Err(_) => return, // shouldn't happen but bail if no window yet
        };
        let top_position = window.height() / 2.0;
        let y = top_position - BLOCK_HEIGHT / 2.0 - BLOCK_SPACER_TOP;
        commands.spawn_batch(spawn_block_row(&window, &mut meshes, &mut materials, y));
    }
}

fn move_platform(
    mut platform_query: Query<&mut Transform, With<Platform>>,
    window_query: Query<&Window>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let mut speed: f32 = 0.0;

    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        speed -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::ArrowRight) {
        speed += 1.0;
    }

    let mut platform = match platform_query.single_mut() {
        Ok(p) => p,
        Err(_) => return, // shouldn't happen but bail if no platform yet
    };
    let window = match window_query.single() {
        Ok(w) => w,
        Err(_) => return, // shouldn't happen but bail if no window yet
    };
    let new_x = platform.translation.x + speed * time.delta_secs() * PLATFORM_SPEED;
    let window_half_width = window.width() / 2.0 - PLATFORM_WIDTH / 2.0;
    let x = new_x.clamp(-window_half_width, window_half_width);
    platform.translation.x = x;
}

fn move_ball(
    time: Res<Time>,
    mut balls: Query<&mut Transform, With<Ball>>,
    window_query: Query<&Window>,
) {
    let window = match window_query.single() {
        Ok(w) => w,
        Err(_) => return,
    };

    let half_width = window.width() / 2.0 - BALL_SIZE;
    let half_height = window.height() / 2.0 - BALL_SIZE;

    for mut ball in &mut balls {
        let movement_direction = ball.rotation * Vec3::Y;
        let movement_distance = BALL_SPEED * time.delta_secs();
        let translation_delta = movement_direction * movement_distance;
        ball.translation += translation_delta;

        let mut bounced = false;
        let mut new_rotation = ball.rotation;

        // Left or right wall
        if ball.translation.x < -half_width {
            ball.translation.x = -half_width;
            new_rotation = Quat::from_rotation_z(-ball.rotation.to_euler(EulerRot::XYZ).2);
            bounced = true;
        } else if ball.translation.x > half_width {
            ball.translation.x = half_width;
            new_rotation = Quat::from_rotation_z(-ball.rotation.to_euler(EulerRot::XYZ).2);
            bounced = true;
        }

        // Top or bottom wall
        if ball.translation.y > half_height {
            ball.translation.y = half_height;
            new_rotation = Quat::from_rotation_z(
                std::f32::consts::PI - ball.rotation.to_euler(EulerRot::XYZ).2,
            );
            bounced = true;
        } else if ball.translation.y < -half_height {
            ball.translation.y = -half_height;
            new_rotation = Quat::from_rotation_z(
                std::f32::consts::PI - ball.rotation.to_euler(EulerRot::XYZ).2,
            );
            bounced = true;
        }

        if bounced {
            ball.rotation = new_rotation.normalize();
        }
    }
}

fn ball_touches(ball: &Transform, object: &Transform, object_width: f32, object_height: f32) -> bool {
    let dx = (ball.translation.x - object.translation.x).abs();
    let dy = (ball.translation.y - object.translation.y).abs();
    dx < (object_width / 2.0 + BALL_SIZE) && dy < (object_height / 2.0 + BALL_SIZE)
}

fn check_collisions(
    mut balls: Query<&mut Transform, (With<Ball>, Without<Platform>, Without<Block>)>,
    platform_query: Query<&Transform, (With<Platform>, Without<Ball>, Without<Block>)>,
    blocks: Query<(Entity, &Transform), (With<Block>, Without<Ball>, Without<Platform>)>,
    mut commands: Commands,
) {
    let Ok(platform_transform) = platform_query.single() else { return };

    for mut ball_transform in &mut balls {
        // 1. Check Platform
        if ball_touches(&ball_transform, platform_transform, PLATFORM_WIDTH, PLATFORM_HEIGHT) {
            let angle = ball_transform.rotation.to_euler(EulerRot::XYZ).2;
            ball_transform.rotation = Quat::from_rotation_z(std::f32::consts::PI - angle).normalize();
        }

        // 2. Check Blocks
        for (entity, block_transform) in &blocks {
            if ball_touches(&ball_transform, block_transform, BLOCK_WIDTH, BLOCK_HEIGHT) {
                let angle = ball_transform.rotation.to_euler(EulerRot::XYZ).2;
                // Bounce vertically (invert Y direction)
                ball_transform.rotation = Quat::from_rotation_z(std::f32::consts::PI - angle).normalize();
                commands.entity(entity).despawn();
                break;
            }
        }
    }
}
