use bevy::prelude::*;

#[derive(Resource)]
pub struct SpawnIngredientTimer(pub Timer);

#[derive(Resource, Default, Debug)]
pub struct MouseMovementSequence {
    pub starting_position: Option<Vec2>,
    // Samples taken along the trajectory of the mouse
    pub samples: Vec<Vec2>,
    pub entity: Option<Entity>,
}

impl MouseMovementSequence {
    pub fn reset(&mut self) {
        self.starting_position = None;
        self.samples.clear();
        self.entity = None;
    }
}
