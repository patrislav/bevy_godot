mod godot_binding;
mod godot_ref;
mod godot_registry;
mod components;

pub use godot_ref::*;
pub use godot_registry::*;
pub use components::*;
pub use godot_binding::*;

use bevy::prelude::IntoExclusiveSystem;

pub struct DefaultGodotPlugins;

impl bevy::app::PluginGroup for DefaultGodotPlugins {
    fn build(&mut self, group: &mut bevy::app::PluginGroupBuilder) {
        group.add(bevy::log::LogPlugin::default());
        group.add(bevy::core::CorePlugin::default());
        group.add(bevy::transform::TransformPlugin::default());
        group.add(bevy::diagnostic::DiagnosticsPlugin::default());
        group.add(GodotBindingPlugin::default());
    }
}

#[derive(Default)]
pub struct GodotBindingPlugin;

impl bevy::app::Plugin for GodotBindingPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.init_resource::<GodotRegistry>();

        app.add_system(sync_godot_bindings_system.exclusive_system());
        app.add_system(components::spawn_godot_scenes_system.exclusive_system());
        app.add_system(components::sync_node2d_transform_system);
        app.add_system(components::sync_animated_sprite_system);
    }
}
