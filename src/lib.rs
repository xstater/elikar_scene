use std::sync::{Arc, RwLock};
use xecs::world::World;

pub mod transform;
pub mod node;

pub fn init(world : Arc<RwLock<World>>) {
    let mut world = world.write().unwrap();
    world.register::<transform::Transform2D>()
        .register::<transform::Transform3D>();
}
