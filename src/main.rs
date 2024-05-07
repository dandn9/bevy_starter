mod components;
mod ingredients_list;
mod resources;
mod systems;

use crate::resources::MouseMovementSequence;
use crate::systems::detect_drag_ingredients;
use bevy::input::common_conditions::input_toggle_active;
use bevy::input::mouse::MouseButtonInput;
use bevy::math::vec3;
use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use bevy::window::PrimaryWindow;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;
use bevy_rapier2d::rapier::prelude::RigidBodyBuilder;
use rand::prelude::*;
use std::f32::consts::PI;

const CAULDRON_SIZE: f32 = 150.0;
const SHELF_SIZE: (f32, f32) = (500.0, 50.0);

fn main() {
    App::new()
        .insert_resource(resources::SpawnIngredientTimer(Timer::from_seconds(
            3.0,
            TimerMode::Repeating,
        )))
        .init_resource::<resources::MouseMovementSequence>()
        .init_resource::<ingredients_list::IngredientsList>()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(10.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::KeyS)),
        )
        .add_systems(
            Startup,
            (
                systems::setup_graphics,
                systems::setup_world,
                ingredients_list::IngredientsList::init_ui,
            ),
        )
        .add_systems(Update, bevy::window::close_on_esc)
        .add_systems(
            Update,
            (
                systems::spawn_ingredients,
                systems::detect_drag_ingredients,
                systems::handle_drag_ingredients
                    .run_if(|sequence: Res<MouseMovementSequence>| sequence.entity.is_some())
                    .after(detect_drag_ingredients),
                systems::handle_outside_ingredients,
            ),
        )
        .add_systems(
            Update,
            (systems::move_cauldron, systems::collision_cauldron),
        )
        .insert_resource(ClearColor(Color::hex("22272e").unwrap()))
        .run();
}
