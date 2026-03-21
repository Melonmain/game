use bevy::{
    asset::RenderAssetUsages,
    mesh::{Indices, PrimitiveTopology},
    prelude::*,
};
use std::f32::consts::PI;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_octagon)
        .add_systems(Update, (check_drag, drag_vertex, update_octagon).chain())
        .run();
}

#[derive(Component)]
struct IsDragged;

#[derive(Component)]
struct Vertex {
    position: Transform,
}

#[derive(Component)]
struct Octagon {
    pub mesh: Handle<Mesh>,
    pub positions: [Entity; 9],
}

fn drag_vertex(
    mut query: Query<&mut Vertex, With<IsDragged>>,
    camera_query: Single<(&Camera, &GlobalTransform)>,
    window: Single<&Window>,

) {
    // Drag logic here
    let (camera, camera_transform) = *camera_query;
        if let Some(cursor_position) = window.cursor_position()
            && let Ok(cursor_world_pos) =
                camera.viewport_to_world_2d(camera_transform, cursor_position)
        {
            for mut vertex in query.iter_mut() {
                vertex.position.translation = Vec3::new(cursor_world_pos.x, cursor_world_pos.y, 0.0);
            }
        }
}

fn check_drag(
    mut commands: Commands,
    query: Query<(Entity, &Vertex)>,
    camera_query: Single<(&Camera, &GlobalTransform)>,
    buttons: Res<ButtonInput<MouseButton>>,
    window: Single<&Window>,
) {
    // Check for mouse input and add IsDragged component to the vertex being dragged
    // Add isDragged modifier if neccessary

    if buttons.just_released(MouseButton::Left) {
        for (entity, _) in query.iter() {
            commands.entity(entity).remove::<IsDragged>();
        }
        return;
    }
    if buttons.just_pressed(MouseButton::Left) {
        let (camera, camera_transform) = *camera_query;
        if let Some(cursor_position) = window.cursor_position()
            && let Ok(cursor_world_pos) =
                camera.viewport_to_world_2d(camera_transform, cursor_position)
        {
            for (entity, vertex) in query.iter() {
                let vertex_pos = vertex.position.translation.truncate();
                if cursor_world_pos.distance(vertex_pos) < 10.0 {
                    commands.entity(entity).insert(IsDragged);
                    println!("Dragging vertex: {:?}", entity);
                    break; // Only drag one vertex at a time
                }
            }
        }
    }
}

fn update_octagon(
    octagon_query: Query<&Octagon>, 
    vertex_query: Query<&Vertex>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for octagon in octagon_query.iter() {
        if let Some(mesh) = meshes.get_mut(&octagon.mesh) {
            let mut new_positions: Vec<[f32; 3]> = Vec::with_capacity(9);

            for &entity in octagon.positions.iter() {
                if let Ok(vertex) = vertex_query.get(entity) {
                    let pos = vertex.position.translation;
                    new_positions.push([pos.x, pos.y, pos.z]);
                } else {
                    new_positions.push([0.0, 0.0, 0.0]);
                }
            }

            // 4. Overwrite the old mesh positions with the newly built array
            if new_positions.len() == 9 {
                mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, new_positions);
            }
        }
    }
}

fn spawn_octagon(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    let mut octagon_mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );

    // 1. Calculate Vertices
    let mut v_pos = vec![[0.0, 0.0, 0.0]];
    for i in 0..8 {
        let a = i as f32 * (PI / 4.0);
        let radius = 200.0;
        v_pos.push([radius * f32::cos(a), radius * f32::sin(a), 0.0]);
    }
    octagon_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, v_pos.clone());

    // Spawn the 9 vertex entities and collect their Entity IDs
    let mut vertex_entities = Vec::new();
    for pos in v_pos.iter() {
        // Spawn each vertex as its own entity in the world
        let id = commands
            .spawn(Vertex {
                // Convert your [f32; 3] array into a Bevy Vec3 for the Transform
                position: Transform::from_xyz(pos[0], pos[1], pos[2]),
            })
            .id();

        vertex_entities.push(id);
    }

    // Convert the Vec into a fixed-size array of 9 entities
    let vectors: [Entity; 9] = vertex_entities.try_into().unwrap();

    // 2. Assign Colors
    let mut v_color = vec![[0.0, 0.0, 0.0, 0.0]]; // Transparent blue center
    v_color.extend_from_slice(&[[0.0, 0.0, 1.0, 1.0]; 8]); // Solid blue edges
    octagon_mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, v_color);

    // 3. Define Triangles
    let mut indices = Vec::new();
    for i in 1..8 {
        indices.extend_from_slice(&[0, i, i + 1]);
    }
    indices.extend_from_slice(&[0, 8, 1]);
    octagon_mesh.insert_indices(Indices::U32(indices));

    // Create the handle ONCE
    let mesh_handle = meshes.add(octagon_mesh);

    // 4. Spawn the actual Octagon entity
    commands.spawn((
        Mesh2d(mesh_handle.clone()), // Use the clone of the handle
        MeshMaterial2d(materials.add(ColorMaterial::default())),
        // Attach your custom component to hold the mesh handle and vertex IDs
        Octagon {
            mesh: mesh_handle,
            positions: vectors,
        },
    ));
}
