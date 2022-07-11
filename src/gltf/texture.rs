use gltf::Gltf;
use xecs::{World, EntityId};
use super::Error;

pub fn load_textures(gltf : &Gltf,world : &World,images : &[EntityId]) -> Result<Vec<EntityId>,Error> {
    let mut textures = Vec::new();
    for texture in gltf.textures() {
        let image_id = images[texture.source().index()];
        if let Some(sample_id) = texture.sampler().index() {
            
        } else {
            // default sampler
            
        }
    }
    Ok(textures)
}