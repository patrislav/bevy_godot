mod godot_binding;
mod godot_ref;
mod godot_registry;
mod godot_scene;
mod plugins;

pub use godot_ref::*;
pub use godot_registry::*;
pub use godot_scene::*;
pub use godot_binding::*;
pub use plugins::*;

use bevy::prelude::{IntoExclusiveSystem, ResMut};
use gdnative::api;

pub struct DefaultGodotPlugins;

impl bevy::app::PluginGroup for DefaultGodotPlugins {
    fn build(&mut self, group: &mut bevy::app::PluginGroupBuilder) {
        group.add(bevy::log::LogPlugin::default());
        group.add(bevy::core::CorePlugin::default());
        group.add(bevy::transform::TransformPlugin::default());
        group.add(bevy::diagnostic::DiagnosticsPlugin::default());
        group.add(GodotBindingPlugin::default());
        group.add(plugins::AnimatedSpritePlugin::default());
        group.add(plugins::Node2DPlugin::default());
        group.add(plugins::LabelPlugin::default());
    }
}

#[derive(Default)]
pub struct GodotBindingPlugin;

impl bevy::app::Plugin for GodotBindingPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.init_resource::<GodotRegistry>();

        app.add_startup_system(register_fallbacks);
        app.add_system(sync_godot_bindings_system.exclusive_system());
        app.add_system(spawn_godot_scenes_system.exclusive_system());
    }
}

fn register_fallbacks(mut registry: ResMut<GodotRegistry>) {
    registry.register_fallback_class::<api::Node, _>("Node", "Object");

    registry.register_fallback_class::<api::CanvasItem, _>("CanvasItem", "Node");
    registry.register_fallback_class::<api::Node2D, _>("Node2D", "CanvasItem");
    registry.register_fallback_class::<api::Sprite, _>("Sprite", "Node2D");
    registry.register_fallback_class::<api::AnimatedSprite, _>("AnimatedSprite", "Node2D");
    registry.register_fallback_class::<api::Path2D, _>("Path2D", "Node2D");
    registry.register_fallback_class::<api::PathFollow2D, _>("PathFollow2D", "Node2D");
    registry.register_fallback_class::<api::CollisionObject2D, _>("CollisionObject2D", "Node2D");
    registry.register_fallback_class::<api::Area2D, _>("Area2D", "CollisionObject2D");

    registry.register_fallback_class::<api::Control, _>("Control", "CanvasItem");
    registry.register_fallback_class::<api::Label, _>("Label", "Control");
}
