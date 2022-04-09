use bevy::prelude::{
    App,
    ResMut,
    Changed,
    Component,
    Query,
    Plugin,
};
use bevy::ecs::world::EntityMut;
use gdnative::{api, prelude::{TRef, Node}};

use crate::{GodotRegistry, GodotRef, insert_godot_ref};

#[derive(Component, Default)]
pub struct AnimatedSprite {
    pub animation: String,
    pub playing: bool,
    pub flip_h: bool,
    pub flip_v: bool,
}

#[derive(Default)]
pub struct AnimatedSpritePlugin;

impl Plugin for AnimatedSpritePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(register)
            .add_system(sync_animated_sprite_system)
        ;
    }
}

fn register(mut registry: ResMut<GodotRegistry>) {
    registry.register_class("AnimatedSprite", "Node2D", Some(insert_components));
}

fn insert_components(node: TRef<Node>, entity_mut: &mut EntityMut) {
    let node = insert_godot_ref::<api::AnimatedSprite>(node, entity_mut);
    if let Some(comp) = entity_mut.get::<AnimatedSprite>() {
        sync_animated_sprite(node, comp);
    } else {
        insert_animated_sprite(node, entity_mut);
    }
}

fn sync_animated_sprite(node: TRef<api::AnimatedSprite>, comp: &AnimatedSprite) {
    node.set_animation(comp.animation.clone());
    node.set_flip_h(comp.flip_h);
    node.set_flip_v(comp.flip_v);

    if comp.playing {
        node.play(comp.animation.clone(), false);
    } else {
        node.stop();
    }
}

fn insert_animated_sprite(node: TRef<api::AnimatedSprite>, entity_mut: &mut EntityMut) {
    entity_mut.insert(AnimatedSprite {
        animation: node.animation().to_string().clone(),
        playing: node.is_playing(),
        flip_h: node.is_flipped_h(),
        flip_v: node.is_flipped_v(),
    });
}

fn sync_animated_sprite_system(
    node_q: Query<(&GodotRef<api::AnimatedSprite>, &AnimatedSprite), Changed<AnimatedSprite>>,
) {
    for (gd_ref, anim_sprite) in node_q.iter() {
        let node = unsafe { gd_ref.0.assume_safe() };
        sync_animated_sprite(node, anim_sprite);
    }
}
