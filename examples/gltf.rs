use elikar_scene::{Camera3D, mesh::MeshResource};
use xecs::{query::WithId, world::World};

fn main() {
    let mut world = World::new();
    
    elikar_scene::init(&mut world);

    elikar_scene::gltf::load_scene(&world,"gltf_files/cube.gltf").unwrap();

    for (id,mesh_resource) in world.query::<&mut MeshResource>().with_id() {
        let (indices,vertices) = mesh_resource.fetch().unwrap();
        let indices = indices.unwrap();
        println!("mesh resource id {}:",id);
        println!("indices {:?},attributes:{:?},size:{},data:{:?}",
                 &indices,
                 vertices.attributes(),
                 vertices.data().len(),
                 vertices.data());
    }

    for (id,camera) in world.query::<&Camera3D>().with_id() {
        println!("camera id {}:",id);
        println!("perspective:{:?}",camera.projection_matrix());
    }
}
