use bevy::prelude::{
    Component,
};
use gdnative::{
    Ref,
    GodotObject,
    core_types::{
        Rect2,
    },
    api::{
        Node2D,
    },
};

#[derive(Component)]
pub struct GodotRef<T: GodotObject>(pub Ref<T>);
