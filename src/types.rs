pub mod entity;
pub mod language;
pub mod scene_node;
pub mod vector2;

// These are module re-exports. It lets us arrange modules into small files while minimizing
// the complexity of import statements.
// Example:
// The full path to the Entity struct is: ti_engine_rs::types::entity::Entity
// With the re-export, you can also acces it with: ti_engine::types::Entity
pub use entity::Entity;
pub use language::Language;
pub use scene_node::SceneNode;
pub use vector2::Vector2;