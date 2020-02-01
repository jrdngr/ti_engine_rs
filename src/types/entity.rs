use crate::types::{SceneNode, Vector2};
use crate::utilities;

#[derive(Debug, Clone)]
pub struct Entity {
    pub node: SceneNode,
    pub position: Vector2<f64>,
    pub velocity: Vector2<f64>,
    pub rotation_degrees: f64,
}

impl PartialEq for Entity {
    fn eq(&self, other: &Self) -> bool {
        self.node == other.node
    }
}

impl Entity {
    pub fn move_entity(&mut self, delta: f64) {
        let radians = utilities::to_radians(self.rotation_degrees);

        let x_diff = radians.cos() * self.velocity.x * delta;
        let y_diff = radians.sin() * self.velocity.y * delta;
        
        let diff = (x_diff, y_diff).into();
        self.position += diff;
    }
}
