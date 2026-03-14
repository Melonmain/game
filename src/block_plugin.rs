use bevy::prelude::*;

const BLOCK_HEIGHT: f32 = 50.0;
const BLOCK_WIDTH: f32 = 100.0;
const BLOCK_SPACER_VERTICAL: f32 = 5.0;
const BLOCK_SPACER_HORIZONTAL: f32 = 5.0;
const BLOCK_SPACER_LEFT: f32 = 50.0;
const BLOCK_SPACER_TOP: f32 = 15.0;
const DEFAULT_ROWS_OF_BLOCKS: i8 = 5;
const BLOCK_DELAY: f32 = 15.0;

#[derive(Component)]
struct Block;

#[derive(Resource)]
struct BlockAdderTimer(Timer);

pub struct BlockPlugin;

impl Plugin for BlockPlugin
{
    fn build(&self, app: &mut App) {
        app.insert_resource(BlockAdderTimer(Timer::from_seconds(
            BLOCK_DELAY,
            TimerMode::Repeating,
        )));
        app.add_systems(PostStartup, setup);
        app.add_systems(Update, spawn_new_blockrow);
    }
}

fn setup(
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