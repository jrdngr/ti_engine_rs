use slotmap::{new_key_type, DenseSlotMap};

use crate::types::scene_node::SceneNode;

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
