use std::collections::HashSet;

use crate::types::GlobalId;

#[derive(Debug, Clone)]
pub struct SceneNode {
    pub drawn: bool,
    pub collidable: bool,
    pub remove: bool,
    pub node_type: String,
    pub parent: Option<GlobalId>,
    id: GlobalId,
    children: HashSet<GlobalId>,
}

impl SceneNode {
    pub fn new(id: GlobalId) -> Self {
        Self {
            id,
            node_type: String::from("Undefined"),
            parent: None,
            drawn: false,
            collidable: false,
            remove: false,
            children: HashSet::new(),
        }
    }

    pub fn id(&self) -> GlobalId {
        self.id
    }

    pub fn children(&self) -> &HashSet<GlobalId> {
        &self.children
    }

    pub fn attach_child(&mut self, id: GlobalId) {
        self.children.insert(id);
    }

    pub fn detach_child(&mut self, id: GlobalId) -> Option<GlobalId> {
        self.children.take(&id)
    }
}

impl PartialEq for SceneNode {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}




// class SceneNode : public sf::Transformable, sf::Drawable { 
// 		virtual sf::FloatRect getHitBox() const;
//
// 		sf::Transform getWorldTransform() const;
// 		sf::Vector2f getWorldPosition() const;
// 		float getWorldRotation() const;
//		
// 		void checkSceneCollisions(SceneNode&, std::set<std::pair<SceneNode*, SceneNode*> >&);
// 		void checkNodeCollisions(SceneNode&, std::set<std::pair<SceneNode*, SceneNode*> >&);
// 		virtual void collide(SceneNode*);
// 		bool collision(SceneNode&, SceneNode&) const;
//
// 		void removeNodes();
//
// 		void update(const float);
// 		virtual void draw(sf::RenderTarget&, sf::RenderStates) const;
//
// 		SceneNode* findNode(sf::Vector2f);
//
//
// 		virtual void updateSelf(const float) = 0;
// 		virtual void drawSelf(sf::RenderTarget&, sf::RenderStates) const = 0;
// };


// sf::Vector2f SceneNode::getWorldPosition() const {
// 	return this->getWorldTransform() * sf::Vector2f();
// }
//

// float SceneNode::getWorldRotation() const {
// 	float rotation = 0;

// 	for (SceneNode* n = parent; n != nullptr; n = n->parent) {
// 		rotation+=n->getRotation();
// 	}

// 	rotation += this->getRotation();

// 	return rotation;
// }


// void SceneNode::checkSceneCollisions(SceneNode& sceneGraphRoot, std::set<std::pair<SceneNode*, SceneNode*> >& collisions) {
// 	this->checkNodeCollisions(sceneGraphRoot, collisions);
// 	for (auto& child : sceneGraphRoot.children) {
// 		this->checkSceneCollisions(*child, collisions);
// 	}	
// }


// void SceneNode::checkNodeCollisions(SceneNode& sceneNode, std::set<std::pair<SceneNode*, SceneNode*> >& collisions) {
// 	if (*this != sceneNode && collision(*this, sceneNode)) {
// 		collisions.insert(std::minmax(this, &sceneNode));
// 	}
// 	for (auto& child : this->children) {
// 		child->checkNodeCollisions(sceneNode, collisions);
// 	}	
// }


// void SceneNode::update(const float delta) {

// 	this->updateSelf(delta);

// 	for (auto& child : this->children) {
// 		if (child != nullptr) {
// 			child->update(delta);
// 		}
// 	}
// }


// void SceneNode::draw(sf::RenderTarget& window, sf::RenderStates states) const {

// 	states.transform *= this->getTransform();

// 	if (this->getDrawn()) {
// 		this->drawSelf(window, states);

// 		for (auto& child : children) {
// 			child->draw(window, states);	
// 		}
// 	}
// }


// void SceneNode::removeNodes() {
// 	auto removesBegin = std::remove_if(this->children.begin(), this->children.end(), std::mem_fn(&SceneNode::getRemove));
// 	this->children.erase(removesBegin, this->children.end());
// 	std::for_each(this->children.begin(), this->children.end(), std::mem_fn(&SceneNode::removeNodes));
// }


// SceneNode& SceneNode::attachChild(std::unique_ptr<SceneNode> child){
// 	child->setParent(this);
// 	children.push_back(std::move(child));
// 	return *children.back();
// }


// sf::Transform SceneNode::getWorldTransform() const {
// 	sf::Transform transform = sf::Transform::Identity;

// 	for (const SceneNode* n = this; n != nullptr; n = n->parent) {
// 		transform = n->getTransform() * transform;
// 	}

// 	return transform;
// }


// bool SceneNode::collision(SceneNode& lhs, SceneNode& rhs) const {
// 	if (!lhs.getCollidable() || !rhs.getCollidable()) {
// 		return false;
// 	}
// 	return lhs.getHitBox().intersects(rhs.getHitBox());
// }


// sf::FloatRect SceneNode::getHitBox() const {
// 	return sf::FloatRect(0, 0, 0, 0); //Test this
// }


// void SceneNode::collide(SceneNode*) {

// }


// SceneNode* SceneNode::findNode(sf::Vector2f point) {
// 	if (this->getHitBox().contains(point)) {
// 		return this;
// 	} else {
// 		for (auto& child : children) {
// 			SceneNode* ptr = child->findNode(point);
// 			if (ptr != nullptr) {
// 				return ptr;
// 			}
// 		}
// 		return nullptr;
// 	}
// }
