use bevy::prelude::*;
use bevy::ecs::world::EntityMut;
use crate::GodotRef;
use gdnative::TRef;
use gdnative::api::Node2D;
use gdnative::core_types::Vector2;

#[derive(Component, Default)]
pub struct Transform2D {
    pub position: Vector2,
    pub rotation: f64,
    pub scale: Vector2,
    pub z_index: i64,
}

pub fn sync_node2d_transform(node: TRef<Node2D>, comp: &Transform2D) {
    node.set_position(comp.position);
    node.set_rotation(comp.rotation);
    node.set_scale(comp.scale);
    node.set_z_index(comp.z_index);
}

pub fn insert_node2d_transform(node: TRef<Node2D>, entity_mut: &mut EntityMut) {
    entity_mut.insert(Transform2D {
        position: node.position(),
        scale: node.scale(),
        rotation: node.rotation(),
        z_index: node.z_index(),
    });
}

pub fn sync_node2d_transform_system(
    node_q: Query<(&GodotRef<Node2D>, &Transform2D), Changed<Transform2D>>,
) {
    for (gd_ref, transform) in node_q.iter() {
        let node = unsafe { gd_ref.0.assume_safe() };
        sync_node2d_transform(node, transform);
    }
}
