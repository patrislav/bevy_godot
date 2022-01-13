use bevy::prelude::{
    Component,
};
use gdnative::{
    Ref,
    GodotObject,
};

#[derive(Component)]
pub struct GodotRef<T: GodotObject>(pub Ref<T>);
