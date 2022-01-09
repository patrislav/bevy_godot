use bevy::prelude::*;
use gdnative::api::{Node, PackedScene};
use gdnative::godot_print;

use crate::{GodotRef, GodotRegistry};

#[derive(Component)]
pub struct GodotScene {
    pub path: &'static str,
}

pub fn spawn_godot_scenes(
    mut commands: Commands,
    gdr: Res<GodotRegistry>,
    scene_q: Query<(&GodotScene, &Parent), (Added<GodotScene>, Without<GodotRef<Node>>)>,
    parents_q: Query<&GodotRef<Node>, With<Children>>,
) {
    let loader = gdnative::api::ResourceLoader::godot_singleton();
    for (scene, parent) in scene_q.iter() {
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
            if let Ok(parent_ref) = parents_q.get(parent.0) {
                let parent_tref = unsafe { parent_ref.0.assume_safe() };
                parent_tref.add_child(instance, false);
                instance_tref.set_owner(parent_ref.0);

                // Bind to Bevy entities
                gdr.bind_recursive(&mut commands, &instance_tref.get_path().to_string());
            }
        } else {
            godot_print!("no such scene '{}'", scene.path);
        }
    }
}
