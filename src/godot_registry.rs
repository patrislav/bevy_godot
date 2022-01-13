use bevy::ecs::world::{World, EntityMut};
use bevy::prelude::{BuildWorldChildren, Entity};
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
    Ref,
    TRef,
    SubClass,
};
use crate::components;
use crate::godot_ref::*;

#[derive(Default)]
pub struct GodotRegistry {
    pub root_ref: Option<Ref<Node>>,
}

pub fn insert_godot_ref<'a, T: 'static + SubClass<Node>>(node: TRef<'a, Node>, entity_mut: &mut EntityMut) -> TRef<'a, T> {
    let node = node.cast::<T>().expect("Expected node to be castable into <T>"); // TODO: improve message, what <T>?
    entity_mut.insert(GodotRef(node.claim()));
    node
}

pub fn get_node_super_class(class: &str) -> Option<&str> {
    // TODO: this can surely be automatically generated.
    match class {
        "CanvasItem" => Some("Node"),
        "Node2D" => Some("CanvasItem"),
        "Sprite" => Some("Node2D"),
        "AnimatedSprite" => Some("Node2D"),
        "Path2D" => Some("Node2D"),
        "PathFollow2D" => Some("Node2D"),
        "CollisionObject2D" => Some("Node2D"),
        "Area2D" => Some("CollisionObject2D"),
        _ => None,
    }
}

pub fn insert_node_components(node: TRef<Node>, entity_mut: &mut EntityMut) {
    let mut class_container = Some(node.get_class().to_string());
    while let Some(class) = class_container {
        let class = class.as_str();
        insert_single_type_components(class, node, entity_mut);
        class_container = get_node_super_class(class).map(|s| String::from(s));
    }
}

pub fn insert_node_components_recursive(world: &mut World, node: TRef<Node>) -> Vec<Entity> {
    let mut child_entities = Vec::new();
    let child_count = node.get_child_count();
    for i in 0..child_count {
        let child = node
            .get_child(i)
            .unwrap_or_else(|| panic!("expected to find child at position {}", i));
        let child = unsafe { child.assume_safe() };
        let mut entity_mut = world.spawn();
        let entity = entity_mut.id();
        child_entities.push(entity);
        insert_node_components(child, &mut entity_mut);

        let children = insert_node_components_recursive(world, child);
        if !children.is_empty() {
            world.entity_mut(entity).push_children(&children);
        }
    }
    child_entities
}

fn insert_single_type_components(class: &str, node: TRef<Node>, entity_mut: &mut EntityMut) {
    match class {
        "Node" => { insert_godot_ref::<Node>(node, entity_mut); }
        "Node2D" => {
            let node = insert_godot_ref::<Node2D>(node, entity_mut);
            if let Some(comp) = entity_mut.get::<components::Transform2D>() {
                components::sync_node2d_transform(node, comp);
            } else {
                components::insert_node2d_transform(node, entity_mut);
            }
        }
        "AnimatedSprite" => {
            let node = insert_godot_ref::<AnimatedSprite>(node, entity_mut);
            if let Some(comp) = entity_mut.get::<components::AnimatedSprite>() {
                components::sync_animated_sprite(node, comp);
            } else {
                components::insert_animated_sprite(node, entity_mut);
            }
        }
        "Sprite" => { insert_godot_ref::<Sprite>(node, entity_mut); }
        "Area2D" => { insert_godot_ref::<Area2D>(node, entity_mut); }
        "Path2D" => { insert_godot_ref::<Path2D>(node, entity_mut); }
        "PathFollow2D" => { insert_godot_ref::<PathFollow2D>(node, entity_mut); }
        _ => ()
    }
}

