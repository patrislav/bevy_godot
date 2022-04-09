use bevy::prelude::*;
use bevy::ecs::world::EntityMut;
use gdnative::{api, prelude::{TRef, Node}};
use crate::{GodotRef, GodotRegistry, insert_godot_ref};

#[derive(Component, Default)]
pub struct Label {
    pub text: String,
}

#[derive(Default)]
pub struct LabelPlugin;

impl Plugin for LabelPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(register)
            .add_system(sync_label_system)
        ;
    }
}

fn register(mut registry: ResMut<GodotRegistry>) {
    registry.register_class("Label", "Control", Some(insert_components));
}

fn insert_components(node: TRef<Node>, entity_mut: &mut EntityMut) {
    let node = insert_godot_ref::<api::Label>(node, entity_mut);
    if let Some(comp) = entity_mut.get::<Label>() {
        sync_label(node, comp);
    } else {
        insert_label(node, entity_mut);
    }
}

fn sync_label(node: TRef<api::Label>, comp: &Label) {
    node.set_text(comp.text.clone());
}

fn insert_label(node: TRef<api::Label>, entity_mut: &mut EntityMut) {
    entity_mut.insert(Label {
        text: node.text().to_string().clone(),
    });
}

fn sync_label_system(node_q: Query<(&GodotRef<api::Label>, &Label), Changed<Label>>) {
    for (gd_ref, label) in node_q.iter() {
        let node = unsafe { gd_ref.0.assume_safe() };
        sync_label(node, label);
    }
}
