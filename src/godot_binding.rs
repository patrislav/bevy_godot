use bevy::prelude::*;
use gdnative::{api::Node, log::godot_warn};

use crate::{GodotRef, GodotRegistry, insert_node_components, insert_node_components_recursive};

#[derive(Component)]
pub struct GodotBinding {
    path: String,
    recursive: bool,
}

impl GodotBinding {
    pub fn single(path: &str) -> Self {
        Self { path: path.to_string(), recursive: false }
    }

    pub fn recursive(path: &str) -> Self {
        Self { path: path.to_string(), recursive: true }
    }
}

pub fn sync_godot_bindings_system(world: &mut World) {
    let gdr = match world.get_resource::<GodotRegistry>() {
        Some(gdr) => gdr,
        None => return,
    };
    let root_ref = gdr.root_ref.unwrap();
    let root_ref = unsafe { root_ref.assume_safe() };

    let mut bindings_q = world
        .query_filtered::<Entity, (Added<GodotBinding>, Without<GodotRef<Node>>)>();
    let mut entities = Vec::new();
    bindings_q.for_each(world, |entity| entities.push(entity));

    for entity in entities {
        let binding = world.get::<GodotBinding>(entity).unwrap();
        let recursive = binding.recursive;
        let node = match root_ref.get_node(binding.path.clone()) {
            Some(node) => node,
            None => {
                godot_warn!("Unable to find node at path '{}'", binding.path);
                continue;
            }
        };
        let node = unsafe { node.assume_safe() };

        insert_node_components(node, &mut world.entity_mut(entity));
        if recursive {
            let child_entities = insert_node_components_recursive(world, node);
            world.entity_mut(entity).push_children(&child_entities);
        }
    }
}
