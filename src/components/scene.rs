use bevy::prelude::*;
use gdnative::api::{Node, Node2D, Area2D, PackedScene};
use gdnative::godot_print;

use crate::{GodotRef, GodotRegistry, Transform2D};

#[derive(Component)]
pub struct GodotScene {
    pub path: &'static str,
}

pub fn spawn_godot_scenes_exclusive(world: &mut World) {
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
                godot_print!("cannot instance scene '{}'", scene.path);
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
                let class = instance_tref.get_class().to_string();
                match class.as_str() {
                    "Area2D" => {
                        let node = instance_tref.cast::<Node2D>().expect("expected Area2D to inherit Node2D");
                        crate::insert_godot_ref_entity_mut::<Node>(instance_tref, world.entity_mut(entity));
                        crate::insert_godot_ref_entity_mut::<Node2D>(instance_tref, world.entity_mut(entity));
                        crate::insert_godot_ref_entity_mut::<Area2D>(instance_tref, world.entity_mut(entity));

                        if let Some(transform) = world.get::<Transform2D>(entity) {
                            node.set_position(transform.position);
                            node.set_rotation(transform.rotation);
                            node.set_scale(transform.scale);
                            node.set_z_index(transform.z_index);
                        }
                    }
                    _ => ()
                }
            }
        } else {
            godot_print!("no such scene '{}'", scene.path);
        }
    }
}
