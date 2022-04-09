use std::collections::HashMap;
use bevy::ecs::world::{World, EntityMut};
use bevy::prelude::{BuildWorldChildren, Entity};
use gdnative::prelude::{
    Ref,
    TRef,
    Node,
    SubClass,
};
use crate::godot_ref::*;

type InsertFunction = fn(TRef<Node>, &mut EntityMut);

#[derive(Clone)]
struct RegClass(String, Option<InsertFunction>);

#[derive(Default, Clone)]
pub struct GodotRegistry {
    pub root_ref: Option<Ref<Node>>,
    reg_classes: HashMap<String, RegClass>,
}

impl GodotRegistry {
    pub fn register_class<S: AsRef<str>>(&mut self, name: S, super_class: S, insert_func: Option<InsertFunction>) {
        self.reg_classes.insert(name.as_ref().to_string(), RegClass(super_class.as_ref().to_string(), insert_func));
    }

    pub fn register_fallback_class<T: 'static + SubClass<Node>, S: AsRef<str>>(&mut self, name: S, super_class: S) {
        let name_string = name.as_ref().to_string();
        if !self.reg_classes.contains_key(&name_string) {
            self.reg_classes.insert(name_string, RegClass(super_class.as_ref().to_string(), Some(insert_fallback_components::<T>)));
        }
    }

    pub fn insert_node_components(&self, node: TRef<Node>, entity_mut: &mut EntityMut) {
        let mut class_container = Some(node.get_class().to_string());
        while let Some(class) = class_container {
            let class = class.as_str();
            self.insert_single_type_components(class, node, entity_mut);
            class_container = self.get_node_super_class(class).map(|s| String::from(s));
        }
    }

    pub fn insert_node_components_recursive(&self, world: &mut World, node: TRef<Node>) -> Vec<Entity> {
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
            self.insert_node_components(child, &mut entity_mut);

            let children = self.insert_node_components_recursive(world, child);
            if !children.is_empty() {
                world.entity_mut(entity).push_children(&children);
            }
        }
        child_entities
    }

    pub fn insert_node_components_recursive_flat(&self, node: TRef<Node>, entity_mut: &mut EntityMut) {
        let child_count = node.get_child_count();
        for i in 0..child_count {
            let child = node
                .get_child(i)
                .unwrap_or_else(|| panic!("expected to find child at position {}", i));
            let child = unsafe { child.assume_safe() };
            let class = child.get_class().to_string();
            let class = class.as_str();
            self.insert_single_type_components(class, child, entity_mut);
            self.insert_node_components_recursive_flat(child, entity_mut);
        }
    }

    fn get_node_super_class(&self, class: &str) -> Option<&str> {
        self.reg_classes.get(&class.to_string()).map(|reg_class| reg_class.0.as_str())
    }

    fn insert_single_type_components(&self, class: &str, node: TRef<Node>, entity_mut: &mut EntityMut) {
        if let Some(reg_class) = self.reg_classes.get(&class.to_string()) {
            if let Some(insert_func) = reg_class.1 {
                insert_func(node, entity_mut);
            }
        }
    }
}

fn insert_fallback_components<T: 'static + SubClass<Node>>(node: TRef<Node>, entity_mut: &mut EntityMut) {
    insert_godot_ref::<T>(node, entity_mut);
}

pub fn insert_godot_ref<'a, T: 'static + SubClass<Node>>(node: TRef<'a, Node>, entity_mut: &mut EntityMut) -> TRef<'a, T> {
    let node = node.cast::<T>().expect("Expected node to be castable into <T>"); // TODO: improve message, what <T>?
    entity_mut.insert(GodotRef(node.claim()));
    node
}
