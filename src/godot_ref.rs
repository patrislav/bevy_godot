use bevy::prelude::{
    Component,
};
use gdnative::prelude::{
    Ref,
    GodotObject,
};

#[derive(Component)]
pub struct GodotRef<T: GodotObject>(pub Ref<T>);
