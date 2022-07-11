mod error;
mod buffer;
mod image;
mod mesh;
mod texture;

use std::path::Path;
pub use error::Error;
use gltf::{camera::Projection, Gltf};
use buffer::load_buffers;
use image::load_images;
use xecs::World;
use crate::{Camera3D, Transform3D};

pub fn load_scene<P : AsRef<Path>>(world : &World,path : P) -> Result<(),Error> {
    let path = path.as_ref();
    let gltf = Gltf::open(path)?;
    let buffers = load_buffers(&gltf,&path)?;
    let images = load_images(&gltf,&path,world)?;
    // load meshes
    // mapped gltf index to EntityId
    // read node
    for node in gltf.nodes() {
        if let Some(mesh) = node.mesh() {
        } else if let Some(camera) = node.camera() {
            let camera = match camera.projection() {
                Projection::Orthographic(_) => todo!(),
                Projection::Perspective(perspective) => 
                    Camera3D::perspective(
                        perspective.aspect_ratio().unwrap(),
                        perspective.yfov(),
                        perspective.znear(),
                        perspective.zfar().unwrap())
            };
            world.create_entity()
                .attach(camera)
                .attach(Transform3D::from_gltf_transform(node.transform()));
        } else {
            
        }
    }
    Ok(())
}
