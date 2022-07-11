pub mod material;
pub mod mesh;
mod transform;
mod camera;
mod light;
mod node;
mod texture;
mod image;
pub mod gltf;

pub use transform::{
    Transform2D,
    Transform3D
};
pub use camera::{
    Camera2D,
    Camera3D
};
pub use mesh::Mesh;
pub use texture::Texture;
pub use image::Image;

use xecs::World;

pub fn init(world : &mut World) {
    world.register::<Transform2D>()
        .register::<Transform3D>()
        .register::<Camera2D>()
        .register::<Camera3D>()
        .register::<mesh::MeshData>()
        .register::<Mesh>();
}
