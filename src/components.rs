use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Ingredient {
    id: u32,
}
impl Ingredient {
    const MAX_INGREDIENT_ID: u32 = 10;

    pub fn new() -> Ingredient {
        let id = (rand::random::<f32>() * (Self::MAX_INGREDIENT_ID + 1) as f32) as u32;
        Ingredient { id }
    }
}
#[derive(Component, Default)]
pub struct Cauldron {
    pub is_selected: bool,
}
