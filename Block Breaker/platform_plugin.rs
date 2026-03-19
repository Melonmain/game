use bevy::prelude::*;
use avian2d::prelude::*;

const PLATFORM_SPEED: f32 = 850.0;
const PLATFORM_WIDTH: f32 = 250.0;
const PLATFORM_HEIGHT: f32 = 30.0;
const PLATFORM_POSITION_Y: f32 = 7.0 / 8.0;

#[derive(Component)]
pub struct IsPlatform;

#[derive(Bundle)]
struct Platform {
    is_platform: IsPlatform,
    transform: Transform,
    rigid_body: RigidBody,
    collider: Collider,
}

impl Platform {
    pub fn new(position: [f32; 3]) -> Self {
        Self {
            is_platform: IsPlatform,
            transform: Transform::from_xyz(position[0], position[1], position[2]),
            rigid_body: RigidBody::Kinematic,
            collider: Collider::rectangle(PLATFORM_WIDTH, PLATFORM_HEIGHT),
        }
    }
}

pub struct PlatformPlugin;

impl Plugin for PlatformPlugin
{
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, setup);
        app.add_systems(Update, move_platform);
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

    let platform_y = -window.height() * PLATFORM_POSITION_Y / 2.0;
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(PLATFORM_WIDTH, PLATFORM_HEIGHT))),
        MeshMaterial2d(materials.add(Color::WHITE)),
        Platform::new([0.0, platform_y, 0.0]),
    ));
}

fn move_platform(
    mut platform_query: Query<&mut Transform, With<IsPlatform>>,
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