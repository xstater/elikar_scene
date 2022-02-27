use xecs::world::World;

mod image;
mod material;
pub mod mesh;
mod transform;
mod camera;
pub mod gltf;

pub use transform::{
    Transform2D,
    Transform3D
};
pub use camera::{
    Camera2D,
    Camera3D
};

pub fn init(world : &mut World) {
    world.register::<Transform2D>()
        .register::<Transform3D>()
        .register::<Camera2D>()
        .register::<Camera3D>()
        .register::<mesh::MeshResource>()
        .register::<mesh::Mesh>();
}
