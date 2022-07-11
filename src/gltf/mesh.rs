use std::{slice::from_raw_parts, mem::size_of};
use gltf::{Gltf, mesh::Mode, Semantic};
use xecs::{World, EntityId};
use crate::mesh::{Assembly, Indices, AttributeType, DataType, Attribute, Vertices, MeshData};
use super::Error;


pub fn load_meshes(gltf: &Gltf,world: &World,buffers: &[Vec<u8>],materials: &[EntityId]) -> Result<Vec<Vec<EntityId>>,Error> {
    let mut ids = Vec::new();
    for mesh in gltf.meshes() {
        ids.push(Vec::new());
        let ids = ids.last_mut().unwrap();
        for primitive in mesh.primitives() {
            let assembly = match primitive.mode() {
                Mode::Points => Assembly::Points,
                Mode::Lines => Assembly::Lines,
                Mode::LineLoop => Assembly::LineLoop,
                Mode::LineStrip => Assembly::LineStrip,
                Mode::Triangles => Assembly::Triangles,
                Mode::TriangleStrip => Assembly::TriangleStrip,
                Mode::TriangleFan => Assembly::TriangleFan,
            };
            // Get indices
            let indices = if let Some(indices) = primitive.indices() {
                // Get buffer and view
                let view = indices.view().ok_or(Error::UnsupportedSparseStorage)?;
                let buffer = &buffers[view.buffer().index()];
                // Check if the indices are u16 or u32
                if indices.dimensions() == gltf::accessor::Dimensions::Scalar &&
                   indices.data_type() == gltf::accessor::DataType::U16 {
                    // Safety:
                    // Safe here because data type is U16 and dimensions is Scalar
                    let slice = unsafe {
                        from_raw_parts(
                            buffer.as_ptr().offset(view.offset() as _) as *const u16,
                            view.length() / size_of::<u16>())
                    };
                    Some(Indices::U16(Vec::from(slice)))
                } else if indices.dimensions() == gltf::accessor::Dimensions::Scalar &&
                          indices.data_type() == gltf::accessor::DataType::U32 {
                    // Safety:
                    // Safe here because data type is U16 and dimensions is Scalar
                    let slice = unsafe {
                        from_raw_parts(
                            buffer.as_ptr().offset(view.offset() as _) as *const u32,
                            view.length() / size_of::<u32>())
                    };
                    Some(Indices::U32(Vec::from(slice)))
                } else {
                    return Err(Error::UnsupportedIndicesFormat);
                }
            } else {
                None
            };
            // Get Vertex Attributes
            let mut attributes = Vec::new();
            let mut is_interleaved = false;
            #[allow(unused)]
            let mut stride = 0;
            let mut data = Vec::new();
            for (semantic,accessor) in primitive.attributes() {
                // convert semantic to attribute type
                let attribute_type = match semantic {
                    Semantic::Positions => AttributeType::Position,
                    Semantic::Normals => AttributeType::Normal,
                    Semantic::TexCoords(index) => AttributeType::TexCoord(index),
                    Semantic::Colors(_) => AttributeType::Color,
                    // ignore unsupported attributes
                    _ => return Err(Error::UnsupportedSemantic(semantic)),
                };
                let view = accessor.view()
                    .ok_or(Error::UnsupportedSparseStorage)?;
                if view.stride().is_some() {
                    is_interleaved = true;
                }
                // Check if mixing the interlace and non-interleaved attributes
                if is_interleaved {
                    // Check if mixing the interlace and non-interleaved attributes
                    if view.stride().is_none() {
                        todo!("Return error: Unsupported interleaved attributes");
                    }
                    todo!("Interleaved attributes");
                } else {
                    // Get buffer and view
                    let buffer = &buffers[view.buffer().index()];
                    // Safety:
                    // Safe here because data type is U16 and dimensions is Scalar
                    let slice = unsafe {
                        from_raw_parts(
                            buffer.as_ptr().offset(view.offset() as _) as *const u8,
                            view.length() * size_of::<u8>())
                    };
                    data.extend_from_slice(slice);
                }
                let data_type = DataType::try_from_gltf(accessor.data_type(),accessor.dimensions())
                    .ok_or(Error::UnsupportedDataTypeOrDimensions)?;
                let count = accessor.count();
                let offset = view.offset();
                let attribute = Attribute {
                    ty: attribute_type,
                    data_type,
                    count,
                    offset,
                };
                attributes.push(attribute);
            }
            let vertices = if is_interleaved {
                Vertices::Interleaved(attributes, stride)
            } else {
                Vertices::Separate(attributes)
            };
            let mesh_data = MeshData {
                data,
                indices,
                vertices,
                material: todo!(),
            };
            let mesh_id = world.create_entity()
                .attach(mesh_data)
                .into_id();
            ids.push(mesh_id);
        }
    }
    Ok(ids)
}
