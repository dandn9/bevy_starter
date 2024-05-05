use std::sync::{Arc, RwLock};

use bevy::prelude::*;

#[derive(Resource)]
pub struct IngredientsList {
    head: Option<Arc<RwLock<IngredientLink>>>,
}

struct IngredientLink {
    id: u32,
    next: Option<Arc<RwLock<IngredientLink>>>,
}

impl IngredientsList {
    pub fn new(num: u32) -> IngredientsList {
        let head = Some(Arc::new(RwLock::new(IngredientLink { id: 0, next: None })));
        let mut curr = head.as_ref().unwrap().clone();

        for i in 1..num {
            let node = Arc::new(RwLock::new(IngredientLink { id: i, next: None }));
            curr.write().unwrap().next = Some(node.clone());
            curr = node;
        }

        IngredientsList { head }
    }
    pub fn init_ui(mut commands: Commands) {
        commands.spawn(NodeBundle {
            style: Style {
                width: Val::Px(40.0),
                height: Val::Px(40.0),
                ..default()
            },
            background_color: Color::RED.into(),
            ..default()
        });
    }
}
