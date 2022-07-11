use xecs::EntityId;

#[derive(Debug,Clone,PartialEq)]
pub struct Node {
    parent: Option<EntityId>,
    children: Vec<EntityId>,
}

impl Node {
    pub fn new() -> Self {
        Node {
            parent: None,
            children: Vec::new(),
        }
    }

    pub fn from_parent(parent: EntityId) -> Self {
        Node {
            parent: Some(parent),
            children: Vec::new(),
        }
    }

    pub fn add_child(&mut self, child: EntityId) {
        self.children.push(child);
    }

    pub fn remove_child(&mut self, child: EntityId) {
        self.children.retain(|c| *c != child);
    }

    pub fn children(&self) -> &[EntityId] {
        &self.children
    }

    pub fn parent(&self) -> Option<EntityId> {
        self.parent
    }

    pub fn set_parent(&mut self, parent: Option<EntityId>) {
        self.parent = parent;
    }
}
