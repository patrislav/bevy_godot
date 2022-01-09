use bevy::prelude::*;
use crate::GodotRef;
use gdnative::api::Node2D;
use gdnative::core_types::Vector2;

pub fn sync_node2d_transform(
    node_q: Query<(&GodotRef<Node2D>, &Transform), Changed<Transform>>,
) {
    for (gd_ref, transform) in node_q.iter() {
        let node = unsafe { gd_ref.0.assume_safe() };

        node.set_position(Vector2::new(transform.translation.x, transform.translation.y));
    }
}
