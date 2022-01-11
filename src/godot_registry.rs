use bevy::ecs::system::EntityCommands;
use bevy::ecs::world::EntityMut;
use bevy::prelude::{
    Commands,
    Component,
    Entity,
    Quat,
    Transform,
    Vec3,
    BuildChildren,
};
use gdnative::api::{
    AnimatedSprite,
    Area2D,
    Node,
    Node2D,
    Sprite,
    Path2D,
    PathFollow2D,
};
use gdnative::prelude::{
    GodotObject,
    Ref,
    SubClass,
    TRef,
};
use crate::components;
use crate::godot_ref::*;

#[derive(Default)]
pub struct GodotRegistry {
    pub root_ref: Option<Ref<Node>>,
}

impl GodotRegistry {
    pub fn bind(&self, commands: &mut Commands, path: &str) -> Option<Entity> {
        self.bind_by_path(commands, path).map(|(e, _)| e)
    }

    pub fn bind_recursive(&self, commands: &mut Commands, path: &str) -> Option<Entity> {
        if let Some(root_ref) = self.root_ref {
            let root_ref = unsafe { root_ref.assume_safe() };
            let node = root_ref.get_node(path).expect("Expected node to be found!");
            let node = unsafe { node.assume_safe() };
            self.bind_recursive_node(commands, node, None)
        } else {
            None
        }
    }

    fn bind_recursive_node(&self, commands: &mut Commands, node: TRef<Node>, parent: Option<Entity>) -> Option<Entity> {
        self.bind_node(commands, node, parent).map(|entity| {
            let child_count = node.get_child_count();
            for i in 0..child_count {
                let child = node
                    .get_child(i)
                    .unwrap_or_else(|| panic!("expected to find child at position {}", i));
                let child = unsafe { child.assume_safe() };
                self.bind_recursive_node(commands, child, Some(entity));
            }
            entity
        })
    }

    fn bind_by_path(&self, commands: &mut Commands, path: &str) -> Option<(Entity, TRef<Node>)> {
        if let Some(root_ref) = self.root_ref {
            let root_ref = unsafe { root_ref.assume_safe() };
            let node = root_ref.get_node(path).expect("Expected node to be found!");
            let node = unsafe { node.assume_safe() };
            self.bind_node(commands, node, None).map(|e| (e, node))
        } else {
            None
        }
    }

    fn bind_node(&self, commands: &mut Commands, node: TRef<Node>, parent: Option<Entity>) -> Option<Entity> {
        let mut ecmd = commands.spawn();
        insert_godot_ref::<Node>(node, &mut ecmd);

        let class = node.get_class().to_string();
        match class.as_str() {
            "Area2D" => {
                insert_node2d(node, &mut ecmd);
                insert_godot_ref::<Node2D>(node, &mut ecmd);
                insert_godot_ref::<Area2D>(node, &mut ecmd);
            }
            "Sprite" => {
                insert_node2d(node, &mut ecmd);
                insert_godot_ref::<Node2D>(node, &mut ecmd);
                insert_godot_ref::<Sprite>(node, &mut ecmd);
            }
            "AnimatedSprite" => {
                insert_node2d(node, &mut ecmd);
                insert_animated_sprite(node, &mut ecmd);
                insert_godot_ref::<Node2D>(node, &mut ecmd);
                insert_godot_ref::<AnimatedSprite>(node, &mut ecmd);
            }
            "Path2D" => {
                insert_godot_ref::<Node2D>(node, &mut ecmd);
                insert_godot_ref::<Path2D>(node, &mut ecmd);
            }
            "PathFollow2D" => {
                insert_godot_ref::<Node2D>(node, &mut ecmd);
                insert_godot_ref::<PathFollow2D>(node, &mut ecmd);
            }
            _ => ()
        };

        let entity = ecmd.id();
        if let Some(parent) = parent {
            commands.entity(parent).add_child(entity);
        }

        Some(entity)
    }
}

fn insert_godot_ref<T: 'static + SubClass<Node>>(node: TRef<Node>, ecmd: &mut EntityCommands) {
    let node = node.cast::<T>().expect("Expected node to be castable into <T>"); // TODO: improve message, what <T>?
    ecmd.insert(GodotRef(node.claim()));
}

pub fn insert_godot_ref_entity_mut<T: 'static + SubClass<Node>>(node: TRef<Node>, mut entity_mut: EntityMut) {
    let node = node.cast::<T>().expect("Expected node to be castable into <T>"); // TODO: improve message, what <T>?
    entity_mut.insert(GodotRef(node.claim()));
}

fn insert_node2d(node: TRef<Node>, ecmd: &mut EntityCommands) {
    let node = node.cast::<Node2D>().expect("Expected node to be of type Node2D");

    ecmd.insert(components::Transform2D {
        position: node.position(),
        scale: node.scale(),
        rotation: node.rotation(),
        z_index: node.z_index(),
    });
}

fn insert_animated_sprite(node: TRef<Node>, ecmd: &mut EntityCommands) {
    let node = node.cast::<AnimatedSprite>().expect("Expected node to be of type AnimatedSprite");

    ecmd.insert(components::AnimatedSprite {
        animation: node.animation().to_string().clone(),
        playing: node.is_playing(),
        flip_h: node.is_flipped_h(),
        flip_v: node.is_flipped_v(),
    });
}

