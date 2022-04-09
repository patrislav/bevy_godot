use bevy::prelude::{Component, Query, Changed};
use bevy::ecs::world::EntityMut;
use gdnative::{api, prelude::TRef};
use crate::GodotRef;

#[derive(Component, Default)]
pub struct Label {
    pub text: String,
}

pub fn sync_label(node: TRef<api::Label>, comp: &Label) {
    node.set_text(comp.text.clone());
}

pub fn insert_label(node: TRef<api::Label>, entity_mut: &mut EntityMut) {
    entity_mut.insert(Label {
        text: node.text().to_string().clone(),
    });
}

pub fn sync_label_system(node_q: Query<(&GodotRef<api::Label>, &Label), Changed<Label>>) {
    for (gd_ref, label) in node_q.iter() {
        let node = unsafe { gd_ref.0.assume_safe() };
        sync_label(node, label);
    }
}
