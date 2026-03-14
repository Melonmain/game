use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColliderTypes {
    Circle,
    Square,
    Rectangle
}

#[derive(Component)]
pub struct Collider {
    pub collider_type: ColliderTypes,
    pub width: f32,
    pub height: f32,
}

impl Collider {
    pub fn new(collider_type: ColliderTypes, width: f32, height: f32) -> Self {
        Self {
            collider_type: collider_type,
            width: width,
            height: height,
        }
    }
}

pub struct ColliderPlugin;
impl Plugin for ColliderPlugin
{
    fn build(&self, app: &mut App) {
    }
}