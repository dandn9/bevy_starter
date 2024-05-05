use crate::components::{Cauldron, Ingredient};
use crate::resources::{MouseMovementSequence, SpawnIngredientTimer};
use crate::{components, CAULDRON_SIZE, SHELF_SIZE};
use bevy::math::vec3;
use bevy::prelude::*;
use bevy::render::camera::camera_system;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use bevy::window::PrimaryWindow;
use bevy_rapier2d::prelude::*;
use rand::prelude::*;
use std::f32::consts::PI;

pub fn setup_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window.get_single().expect("More than one window");

    let rect = Mesh2dHandle(meshes.add(Rectangle::new(CAULDRON_SIZE, CAULDRON_SIZE)));

    // They probably should not be same size
    let shelf1size = (SHELF_SIZE.0, SHELF_SIZE.1);
    let shelf1 = Mesh2dHandle(meshes.add(Rectangle::new(shelf1size.0, shelf1size.1)));
    let shelf2size = (SHELF_SIZE.0 - 100.0, SHELF_SIZE.1);
    let shelf2 = Mesh2dHandle(meshes.add(Rectangle::new(shelf2size.0, shelf2size.1)));

    // Spawn the cauldron
    commands.spawn((
        Name::new("Cauldron"),
        MaterialMesh2dBundle {
            mesh: rect,
            material: materials.add(Color::RED),
            transform: Transform::from_xyz(
                0.0,
                (-window.resolution.height() * 0.5) + CAULDRON_SIZE / 2.0,
                0.0,
            ),
            ..default()
        },
        Collider::cuboid(CAULDRON_SIZE / 2., CAULDRON_SIZE / 2.),
        CollisionGroups::new(Group::GROUP_1, Group::ALL),
        Sensor,
        Cauldron::default(),
        ActiveEvents::COLLISION_EVENTS,
    ));

    // Spawn the shelves
    commands.spawn((
        Name::new("Shelf1"),
        MaterialMesh2dBundle {
            mesh: shelf1,
            material: materials.add(Color::BLUE),
            transform: Transform {
                translation: vec3(-200.0, 214.0, 0.0),
                rotation: Quat::from_rotation_z(PI / -15.7),
                ..default()
            },
            ..default()
        },
        RigidBody::Fixed,
        CollisionGroups::new(Group::GROUP_1, Group::ALL),
        Collider::cuboid(shelf1size.0 / 2.0, shelf1size.1 / 2.0),
    ));
    commands.spawn((
        Name::new("Shelf2"),
        MaterialMesh2dBundle {
            mesh: shelf2,
            material: materials.add(Color::BLUE),
            transform: Transform {
                translation: vec3(300.0, 58., 0.),
                rotation: Quat::from_rotation_z(PI / 10.0),
                ..default()
            },
            ..default()
        },
        RigidBody::Fixed,
        CollisionGroups::new(Group::GROUP_1, Group::ALL),
        Collider::cuboid(shelf2size.0 / 2.0, shelf1size.1 / 2.0),
    ));
}
pub fn setup_graphics(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
pub fn collision_cauldron(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    mut mouse_movement_sequence: ResMut<MouseMovementSequence>,
) {
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(_, collided_with, ..) => {
                commands.entity(*collided_with).despawn();
                // Cleanup the resource if it was the same we were dragging
                if let Some(entity) = mouse_movement_sequence.entity {
                    if entity == *collided_with {
                        mouse_movement_sequence.reset();
                    }
                }
            }
            _ => {}
        }
    }
}
pub fn move_cauldron(
    mut cauldron: Query<&mut Transform, With<Cauldron>>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    window: Query<&Window, With<PrimaryWindow>>,
) {
    if mouse_button.pressed(MouseButton::Left) {
        let mut cauldron_transform = cauldron.get_single_mut().expect("More than one cauldron");
        let window = window.get_single().expect("More than one window");

        if let Some(position) = window.cursor_position() {
            let mut x = f32::clamp(
                position.x - window.width() / 2.0,
                -window.width() / 2. + CAULDRON_SIZE / 2.,
                window.width() / 2. - CAULDRON_SIZE / 2.,
            );
            let mut y = f32::clamp(
                -(position.y - window.height() / 2.0),
                window.height() / -2.0 + CAULDRON_SIZE / 2.0,
                window.height() / -2.0 + CAULDRON_SIZE / 2.0 + 120.0, // 100.0 => wiggle room
            );

            // This can be improved imo
            let cauldron_l = cauldron_transform.translation.x - CAULDRON_SIZE / 2.0;
            let cauldron_r = cauldron_transform.translation.x + CAULDRON_SIZE / 2.0;
            let cauldron_b = cauldron_transform.translation.y - CAULDRON_SIZE / 2.0;
            let cauldron_t = cauldron_transform.translation.y + CAULDRON_SIZE / 2.0;

            // Check that the cursor is inside the cauldron
            if x >= cauldron_l && x <= cauldron_r && y >= cauldron_b && y <= cauldron_t {
                // TODO: make this slower and add some wiggle room for cauldron to be not selected
                x = f32::lerp(cauldron_transform.translation.x, x, 0.7);
                y = f32::lerp(cauldron_transform.translation.y, y, 0.7);
                cauldron_transform.translation = vec3(x, y, 0.0);
            }
        }
    }
}

pub fn spawn_ingredients(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<SpawnIngredientTimer>,
) {
    if !timer.0.tick(time.delta()).finished() {
        return;
    }
    let size = rand::random::<f32>();

    commands.spawn((
        RigidBody::Dynamic,
        // GravityScale(1.0 / size),
        ColliderMassProperties::Mass(size * 4.0 + 0.2),
        Collider::ball(20.0 + size * 40.0),
        CollisionGroups::new(Group::GROUP_2, Group::ALL),
        Restitution::coefficient(f32::max(0.8 - size, 0.0)),
        TransformBundle::from(Transform::from_xyz(
            -200.0 + (rand::random::<f32>() - 0.5) * 40.,
            580.0,
            0.0,
        )),
        Ingredient::new(),
    ));
}
pub fn handle_outside_ingredients(
    ingredients: Query<(&Collider, &Transform, Entity), With<Ingredient>>,
    window: Query<&Window, With<PrimaryWindow>>,

    mut mouse_movement_sequence: ResMut<MouseMovementSequence>,
    mut commands: Commands,
) {
    let window = window.single();

    let win_xmin = window.width() / -2.0;
    let win_xmax = window.width() / 2.0;
    let win_ymin = window.height() / -2.0;

    for (collider, transform, entity) in ingredients.iter() {
        let radius = collider.as_ball().unwrap().radius();

        if !box_intersects_sphere(
            win_xmin,
            win_xmax,
            win_ymin,
            f32::MAX,
            transform.translation.xy(),
            radius,
        ) {
            commands.entity(entity).despawn();
            if let Some(mouse_entity) = mouse_movement_sequence.entity {
                if mouse_entity == entity {
                    mouse_movement_sequence.reset();
                }
            }
        }
    }
}

fn box_intersects_sphere(
    box_xmin: f32,
    box_xmax: f32,
    box_ymin: f32,
    box_ymax: f32,
    sphere_position: Vec2,
    sphere_radius: f32,
) -> bool {
    let x = f32::max(box_xmin, f32::min(sphere_position.x, box_xmax));
    let y = f32::max(box_ymin, f32::min(sphere_position.y, box_ymax));

    let distance = f32::sqrt((x - sphere_position.x).powi(2) + (y - sphere_position.y).powi(2));

    distance < sphere_radius
}

pub fn handle_drag_ingredients(
    camera: Query<(&Camera, &GlobalTransform)>,
    window: Query<&Window, With<PrimaryWindow>>,
    mut gizmos: Gizmos,
    mut commands: Commands,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut mouse_movement_sequence: ResMut<MouseMovementSequence>,
) {
    let (camera, camera_transform) = camera.single();
    let Some(cursor_position) = window.single().cursor_position() else {
        return;
    };
    let Some(point) = camera.viewport_to_world_2d(camera_transform, cursor_position) else {
        return;
    };
    mouse_movement_sequence.samples.push(point);

    let impulse_vector = calc_impulse_vector(&mouse_movement_sequence);
    // Draw gizmo
    {
        gizmos.arrow_2d(
            mouse_movement_sequence.starting_position.unwrap(),
            mouse_movement_sequence.starting_position.unwrap() + impulse_vector,
            Color::RED,
        );
    }

    if !mouse_button.pressed(MouseButton::Left) {
        commands
            .entity(mouse_movement_sequence.entity.unwrap())
            .insert(ExternalImpulse {
                impulse: impulse_vector,
                torque_impulse: 14.0,
            });

        // Handle sequence end
        mouse_movement_sequence.reset();

        return;
    }
}

fn calc_impulse_vector(sequence: &MouseMovementSequence) -> Vec2 {
    let mut impulse_vector = Vec2::new(0.0, 0.0);

    for sample in sequence.samples.iter() {
        impulse_vector += *sample - sequence.starting_position.unwrap();
    }

    impulse_vector /= sequence.samples.len() as f32;

    let diff_len = ((sequence.starting_position.unwrap() + impulse_vector)
        - sequence.starting_position.unwrap())
    .length()
    .log(2.);

    // The bigger the stroke length, the more we want to increase the impulse vector
    impulse_vector *= diff_len;

    impulse_vector
}
pub fn detect_drag_ingredients(
    camera: Query<(&Camera, &GlobalTransform)>,
    window: Query<&Window, With<PrimaryWindow>>,
    rapier_context: Res<RapierContext>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut mouse_movement_sequence: ResMut<MouseMovementSequence>,
) {
    // Return if it's already dragging
    let None = mouse_movement_sequence.entity else {
        return;
    };
    if !mouse_button.pressed(MouseButton::Left) {
        return;
    }
    let (camera, camera_transform) = camera.single();
    let Some(cursor_position) = window.single().cursor_position() else {
        return;
    };
    let Some(point) = camera.viewport_to_world_2d(camera_transform, cursor_position) else {
        return;
    };

    let filter = QueryFilter::new().groups(CollisionGroups::new(Group::ALL, Group::GROUP_2));
    rapier_context.intersections_with_point(point.into(), filter, |entity| {
        mouse_movement_sequence.entity = Some(entity);
        mouse_movement_sequence.starting_position = Some(point);
        true
    });
}
