use bevy::prelude::*;
use bevy::ecs::world::EntityMut;
use crate::{GodotRef, GodotRegistry, insert_godot_ref};
use gdnative::prelude::{TRef, Node};
use gdnative::api::Node2D;
use gdnative::core_types::Vector2;

#[derive(Component, Default)]
pub struct Transform2D {
    pub position: Vector2,
    pub rotation: f64,
    pub scale: Vector2,
    pub z_index: i64,
}

#[derive(Default)]
pub struct Node2DPlugin;

impl Plugin for Node2DPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(register)
            .add_system(sync_node2d_transform_system)
        ;
    }
}

fn register(mut registry: ResMut<GodotRegistry>) {
    registry.register_class("Node2D", "CanvasItem", Some(insert_components));
}

fn insert_components(node: TRef<Node>, entity_mut: &mut EntityMut) {
    let node = insert_godot_ref::<Node2D>(node, entity_mut);
    if let Some(comp) = entity_mut.get::<Transform2D>() {
        sync_node2d_transform(node, comp);
    } else {
        insert_node2d_transform(node, entity_mut);
    }
}

fn sync_node2d_transform(node: TRef<Node2D>, comp: &Transform2D) {
    node.set_position(comp.position);
    node.set_rotation(comp.rotation);
    node.set_scale(comp.scale);
    node.set_z_index(comp.z_index);
}

fn insert_node2d_transform(node: TRef<Node2D>, entity_mut: &mut EntityMut) {
    entity_mut.insert(Transform2D {
        position: node.position(),
        scale: node.scale(),
        rotation: node.rotation(),
        z_index: node.z_index(),
    });
}

fn sync_node2d_transform_system(
    node_q: Query<(&GodotRef<Node2D>, &Transform2D), Changed<Transform2D>>,
) {
    for (gd_ref, transform) in node_q.iter() {
        let node = unsafe { gd_ref.0.assume_safe() };
        sync_node2d_transform(node, transform);
    }
}
