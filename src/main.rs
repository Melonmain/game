use bevy::{
    asset::RenderAssetUsages, mesh::{Indices, PrimitiveTopology}, prelude::*
};
use std::f32::consts::PI;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_octagon)
        .run();
}

fn spawn_octagon(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    let mut octagon = Mesh::new(
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
    octagon.insert_attribute(Mesh::ATTRIBUTE_POSITION, v_pos);

    // 2. Assign Colors
    // Standard Bevy vertex colors use an array of 4 floats [R, G, B, A]
    let mut v_color = vec![[0.0, 0.0, 0.0, 0.0]]; // Transparent blue center
    v_color.extend_from_slice(&[[0.0, 0.0, 1.0, 1.0]; 8]); // Solid blue edges
    
    // Using the built-in ATTRIBUTE_COLOR automatically tells Bevy's default pipeline how to read it
    octagon.insert_attribute(Mesh::ATTRIBUTE_COLOR, v_color);

    // 3. Define Triangles (Counter-clockwise winding)
    let mut indices = Vec::new();
    for i in 1..8 {
        indices.extend_from_slice(&[0, i, i + 1]);
    }
    indices.extend_from_slice(&[0, 8, 1]);
    octagon.insert_indices(Indices::U32(indices));

    // 4. Spawn using Bevy's standard 2D rendering components
    commands.spawn((
        Mesh2d(meshes.add(octagon)),
        // A default ColorMaterial multiplies its base color (white) by the vertex colors
        MeshMaterial2d(materials.add(ColorMaterial::default())),
    ));
}