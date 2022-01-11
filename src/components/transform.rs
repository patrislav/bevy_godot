use bevy::prelude::*;
use crate::GodotRef;
use gdnative::api::Node2D;
use gdnative::core_types::Vector2;

#[derive(Component, Default)]
pub struct Transform2D {
    pub position: Vector2,
    pub rotation: f64,
    pub scale: Vector2,
    pub z_index: i64,
}

pub fn sync_node2d_transform(
    node_q: Query<(&GodotRef<Node2D>, &Transform2D), Changed<Transform2D>>,
) {
    for (gd_ref, transform) in node_q.iter() {
        let node = unsafe { gd_ref.0.assume_safe() };

        node.set_position(transform.position);
        node.set_rotation(transform.rotation);
        node.set_scale(transform.scale);
        node.set_z_index(transform.z_index);
    }
}
