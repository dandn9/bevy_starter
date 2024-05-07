use std::sync::{Arc, RwLock};

use bevy::prelude::*;

pub const INGREDIENTS: u32 = 10;

#[derive(Resource)]
pub struct IngredientsList {
    head: Option<Box<IngredientLink>>,
}

#[derive(Debug)]
struct IngredientLink {
    id: u32,
    next: Option<Box<IngredientLink>>,
}
impl Into<String> for IngredientLink {
    fn into(self) -> String {
        self.id.to_string()
    }
}

#[derive(Component, Debug)]
struct CurrentIngredient(u32);
#[derive(Component, Debug)]
struct ListedIngredient(u32);

impl Default for IngredientsList {
    fn default() -> Self {
        IngredientsList::new(10)
    }
}

fn create_list(n: u32) -> IngredientLink {
    let r = rand::random::<f32>();
    let h = (r * (INGREDIENTS + 1) as f32) as u32;
    if n == 0 {
        IngredientLink { id: h, next: None }
    } else {
        IngredientLink {
            id: h,
            next: Some(Box::new(create_list(n - 1))),
        }
    }
}

impl IngredientsList {
    pub fn new(num: u32) -> IngredientsList {
        let r = rand::random::<f32>();
        let h = (r * (INGREDIENTS + 1) as f32) as u32;

        let head = Some(Box::new(IngredientLink {
            id: h,
            next: Some(Box::new(create_list(num))),
        }));

        IngredientsList { head }
    }
    pub fn init_ui(mut commands: Commands, ingredients_list: ResMut<IngredientsList>) {
        let head = ingredients_list.head.as_ref().expect("Missing head :(");
        let next = head.next.as_ref().expect("Expected next to exist");

        commands
            .spawn(NodeBundle {
                style: Style {
                    width: Val::Px(120.0),
                    height: Val::Px(120.0),
                    ..default()
                },
                background_color: Color::RED.into(),
                ..default()
            })
            .with_children(|p| {
                p.spawn((
                    TextBundle {
                        text: Text::from_section(
                            &head.id.to_string(),
                            TextStyle {
                                font_size: 50.0,
                                color: Color::BLACK,
                                ..default()
                            },
                        ),
                        ..default()
                    },
                    CurrentIngredient(head.id),
                ));
            });

        commands
            .spawn(NodeBundle {
                style: Style {
                    width: Val::Px(120.0),
                    height: Val::Px(120.0),
                    left: Val::Px(160.0),
                    ..default()
                },
                background_color: Color::RED.into(),
                ..default()
            })
            .with_children(|p| {
                p.spawn((
                    TextBundle {
                        text: Text::from_section(
                            &next.id.to_string(),
                            TextStyle {
                                font_size: 50.0,
                                color: Color::BLACK,
                                ..default()
                            },
                        ),
                        ..default()
                    },
                    ListedIngredient(next.id),
                ));
            });
    }
}
