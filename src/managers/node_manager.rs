use slotmap::{new_key_type, DenseSlotMap};

use crate::types::SceneNode;

// Anything that looks like a function that ends with "!" is a macro.
// It takes its input as an abstract syntax tree, modifies the tree directly,
// and returns a new abstract syntax tree.
// 
// This macro lets you declare a type for the keys in your DenseSlotMap and then
// does all of the work for you to make it work with the DenseSlotMap
new_key_type!{ pub struct GlobalId; }

pub struct NodeManager {
    nodes: DenseSlotMap<GlobalId, SceneNode>,
}

impl Default for NodeManager {
    fn default() -> Self {
        Self {
            nodes: DenseSlotMap::with_key(),
        }
    }
}

impl NodeManager {
    pub fn create_node(&mut self) -> GlobalId {
        self.nodes.insert_with_key(|id| SceneNode::new(id))
    }

    pub fn get_node(&self, id: GlobalId) -> Option<&SceneNode> {
        self.nodes.get(id)
    }

    pub fn get_node_mut(&mut self, id: GlobalId) -> Option<&mut SceneNode> {
        self.nodes.get_mut(id)
    }

    pub fn remove_node(&mut self, id: GlobalId) -> Option<SceneNode> {
        self.nodes.remove(id)
    }
}
