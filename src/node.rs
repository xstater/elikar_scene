//! # ECS的树形结构
//! 先为需要建立树形结构的entity添加Node组件
use std::sync::{Arc, RwLock};
use xecs::{entity::EntityId, world::World};

pub struct Node{
    parent : Option<EntityId>,
    children : Vec<EntityId>
}

impl Node {
    pub fn new() -> Self {
        Node {
            parent: None,
            children: Vec::new(),
        }
    }

    pub fn is_root(&self) -> bool {
        self.parent.is_none()
    }

    pub fn children(&self) -> &[EntityId] {
        &self.children
    }

    pub fn parent(&self) -> Option<EntityId> {
        self.parent
    }
}

pub struct Manipulator{
    world : Arc<RwLock<World>>,
    id : EntityId
}

impl Manipulator {
    pub fn from_id(world : Arc<RwLock<World>>,id : EntityId) -> Self {
        Manipulator {
            world,
            id,
        }
    }

    pub fn add_child(&mut self,id : EntityId) {
        let world = self.world.read().unwrap();
        let mut nodes = world.component_mut::<Node>().unwrap();
        {
            // add chlid
            let node = nodes.get_mut(self.id)
                .expect(format!("[elikar_scene::node::Manipulator::add_child(id)]\
                                entity id = {} hasn't Node component",self.id)
                        .as_str());
            if !node.children.contains(&id) {
                node.children.push(id);
            }
        }
        {
            // set parent
            let node = nodes.get_mut(id)
                .expect(format!("[elikar_scene::node::Manipulator::add_child(id)]\
                                entity id = {} hasn't Node component",id)
                        .as_str());
            node.parent = Some(self.id);
        }
    }

    pub fn remove_child(&mut self,id : EntityId) {
        let world = self.world.read().unwrap();
        let mut nodes = world.component_mut::<Node>().unwrap();
        {
            // remove chlid
            let node = nodes.get_mut(self.id)
                .expect(format!("[elikar_scene::node::Manipulator::remove_child(id)]\
                                entity id = {} hasn't Node component",self.id)
                        .as_str());
            let index = node.children.iter()
                .enumerate()
                .find(|(_,child)|**child == id)
                .map(|(id,_)|id);
            if let Some(index) = index {
                node.children.remove(index);
            }
        }
        {
            // set parent
            let node = nodes.get_mut(id)
                .expect(format!("[elikar_scene::node::Manipulator::remove_child(id)]\
                                entity id = {} hasn't Node component",id)
                        .as_str());
            node.parent = None;
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, RwLock};
    use xecs::{entity::EntityId, world::World};
    use super::{Manipulator, Node};

    #[test]
    fn basic() {
        let mut world = World::new();
        world.register::<Node>();
        for _ in 0..5 {
            world.create_entity()
                .attach(Node::new());
        }

        let _1 = EntityId::new(1).unwrap();
        let _2 = EntityId::new(2).unwrap();
        let _3 = EntityId::new(3).unwrap();
        let _4 = EntityId::new(4).unwrap();
        let _5 = EntityId::new(5).unwrap();

        let world = Arc::new(RwLock::new(world));
        //   1
        //  2 3
        //   4 5
        {
            let mut root = Manipulator::from_id(world.clone(),_1);
            root.add_child(_2);
            root.add_child(_3);
        }
        {
            let mut node = Manipulator::from_id( world.clone(),_3);
            node.add_child(_4);
            node.add_child(_5);
        }

        {
            let world = world.read().unwrap();
            let nodes = world.component_ref::<Node>().unwrap();
            {
                let node = nodes.get(_1).unwrap();
                assert_eq!(node.children(), &[_2,_3]);
                assert_eq!(node.parent(),None);
            }
            {
                let node = nodes.get(_2).unwrap();
                assert_eq!(node.children(), &[]);
                assert_eq!(node.parent(),Some(_1));
            }
            {
                let node = nodes.get(_3).unwrap();
                assert_eq!(node.children(), &[_4,_5]);
                assert_eq!(node.parent(),Some(_1));
            }
            {
                let node = nodes.get(_4).unwrap();
                assert_eq!(node.children(), &[]);
                assert_eq!(node.parent(),Some(_3));
            }
            {
                let node = nodes.get(_5).unwrap();
                assert_eq!(node.children(), &[]);
                assert_eq!(node.parent(),Some(_3));
            }
        }
    }
}
