use bevy::prelude::{
    Changed,
    Component,
    Query,
};
use bevy::ecs::world::EntityMut;
use gdnative::{api, TRef};

use crate::GodotRef;

#[derive(Component, Default)]
pub struct AnimatedSprite {
    pub animation: String,
    pub playing: bool,
    pub flip_h: bool,
    pub flip_v: bool,
}

pub fn sync_animated_sprite(node: TRef<api::AnimatedSprite>, comp: &AnimatedSprite) {
    node.set_animation(comp.animation.clone());
    node.set_flip_h(comp.flip_h);
    node.set_flip_v(comp.flip_v);

    if comp.playing {
        node.play(comp.animation.clone(), false);
    } else {
        node.stop();
    }
}

pub fn insert_animated_sprite(node: TRef<api::AnimatedSprite>, entity_mut: &mut EntityMut) {
    entity_mut.insert(AnimatedSprite {
        animation: node.animation().to_string().clone(),
        playing: node.is_playing(),
        flip_h: node.is_flipped_h(),
        flip_v: node.is_flipped_v(),
    });
}

pub fn sync_animated_sprite_system(
    node_q: Query<(&GodotRef<api::AnimatedSprite>, &AnimatedSprite), Changed<AnimatedSprite>>,
) {
    for (gd_ref, anim_sprite) in node_q.iter() {
        let node = unsafe { gd_ref.0.assume_safe() };
        sync_animated_sprite(node, anim_sprite);
    }
}
