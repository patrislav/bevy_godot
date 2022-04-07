use bevy::prelude::*;
use gdnative::api::{Node, PackedScene};
use gdnative::log::godot_warn;

use crate::{GodotRef, insert_node_components, insert_node_components_recursive};

#[derive(Component)]
pub struct GodotScene {
    pub path: &'static str,
}

pub fn spawn_godot_scenes_system(world: &mut World) {
    let loader = gdnative::api::ResourceLoader::godot_singleton();

    let mut scene_q = world.query_filtered::<
        Entity,
        (Added<GodotScene>, With<Parent>, Without<GodotRef<Node>>)
    >();
    let mut entities = Vec::new();
    scene_q.for_each(world, |entity| entities.push(entity));

    for entity in entities {
        let scene = world.get::<GodotScene>(entity).unwrap();
        let parent = world.get::<Parent>(entity).unwrap();

        // Load the resource as "PackedScene"
        if let Some(res) = loader.load(scene.path, "PackedScene", false) {
            let res = res.cast::<PackedScene>().expect("expected resource to be a PackedScene");
            let res = unsafe { res.assume_safe() };
            if !res.can_instance() {
                godot_warn!("cannot instance scene '{}'", scene.path);
                continue;
            }

            let instance = res.instance(0).expect("expected to be able to instantiate scene");
            let instance_tref = unsafe { instance.assume_safe() };

            // Attach it to a parent node
            if let Some(parent_ref) = world.get::<GodotRef<Node>>(parent.0) {
                let parent_tref = unsafe { parent_ref.0.assume_safe() };
                parent_tref.add_child(instance, false);
                instance_tref.set_owner(parent_ref.0);

                // Bind to Bevy entities
                insert_node_components(instance_tref, &mut world.entity_mut(entity));
                let child_entities = insert_node_components_recursive(world, instance_tref);
                if !child_entities.is_empty() {
                    world.entity_mut(entity).push_children(&child_entities);
                }
            }
        } else {
            godot_warn!("no such scene '{}'", scene.path);
        }
    }
}
