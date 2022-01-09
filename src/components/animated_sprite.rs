use bevy::prelude::{
    Component,
    Query,
    Changed,
};
use crate::GodotRef;

#[derive(Component, Default)]
pub struct AnimatedSprite {
    pub animation: String,
    pub playing: bool,
    pub flip_h: bool,
    pub flip_v: bool,
}

pub fn sync_animated_sprite(
    node_q: Query<(&GodotRef<gdnative::api::AnimatedSprite>, &AnimatedSprite), Changed<AnimatedSprite>>,
) {
    for (gd_ref, anim_sprite) in node_q.iter() {
        let node = unsafe { gd_ref.0.assume_safe() };

        node.set_animation(anim_sprite.animation.clone());
        node.set_flip_h(anim_sprite.flip_h);
        node.set_flip_v(anim_sprite.flip_v);

        if anim_sprite.playing {
            node.play(anim_sprite.animation.clone(), false);
        } else {
            node.stop();
        }
    }
}
