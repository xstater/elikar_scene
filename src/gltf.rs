use std::{fmt::Display, fs::File, io::{self, Read}, mem::size_of, path::Path, slice::from_raw_parts};
use gltf::{Gltf, accessor::{DataType, Dimensions}, buffer, camera::Projection, mesh::Mode};
use xecs::world::World;
use crate::{Camera3D, Transform3D, mesh::{Assembly, Attribute, Mesh, MeshResource, VertexData, VertexFormat}};

#[derive(Debug)]
pub enum Error {
    GltfError(gltf::Error),
    UnsupportedIndicesFormat,
    IoError(io::Error),
    UnsupportedSemantic,
    UnsupportedDataTypeOrDimensions,
    UnsupportedBufferLayout,
    UnsupportedSparseStorage,
}

impl From<gltf::Error> for Error {
    fn from(err: gltf::Error) -> Self {
        Error::GltfError(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::IoError(err)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::GltfError(err) => 
                write!(f,"Loading GLTF file failed: {}",err),
            Error::UnsupportedIndicesFormat => 
                write!(f,"Loading GLTF file failed: The indices format is not supported"),
            Error::IoError(err) =>
                write!(f,"Loading GLTF file failed: {}",err),
            Error::UnsupportedSemantic =>
                write!(f,"Loading GLTF file failed: The semantic is not supported"),
            Error::UnsupportedDataTypeOrDimensions =>
                write!(f,"Loading GLTF file failed: The data type or Dimensions are not supported"),
            Error::UnsupportedBufferLayout =>
                write!(f,"Loading GLTF file failed: The buffer layout is not supported"),
            Error::UnsupportedSparseStorage =>
                write!(f,"Loading GLTF file failed: The sparse feature is not supported"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::GltfError(err) => Some(err),
            Error::IoError(err) => Some(err),
            _ => None
        }
    }
}

pub fn load_scene<P : AsRef<Path>>(world : &World,path : P) -> Result<(),Error> {
    let gltf = Gltf::open(path)?;
    // load buffers
    let mut buffers = Vec::new();
    for buffer in gltf.buffers() {
        match buffer.source() {
            buffer::Source::Bin => todo!(),
            buffer::Source::Uri(path) => {
                let mut file = File::open(path)?;
                let mut v = Vec::new();
                file.read_to_end(&mut v)?;
                buffers.push(v);
            },
        }
    }
    // load meshes
    // mapped gltf index to EntityId
    let mut meshes = Vec::new();
    for mesh in gltf.meshes() {
        // gltf::primitive => crate::Mesh
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
            let indices = if let Some(indices) = primitive.indices() {
                if indices.dimensions() == Dimensions::Scalar &&
                   indices.data_type()  == DataType::U16{
                    let view = indices.view().ok_or(Error::UnsupportedSparseStorage)?;
                    let buffer = &buffers[view.buffer().index()];
                    // Safety:
                    // Safe here because data type is U16 and dimensions is Scalar
                    let slice = unsafe {
                        from_raw_parts(
                            buffer.as_ptr().offset(view.offset() as _) as *const u16,
                            view.length() / size_of::<u16>())
                    };
                    Some(Vec::from(slice))
                } else {
                    return Err(Error::UnsupportedIndicesFormat)
                }
            } else {
                None
            };
            let mut count = 0;
            let mut attributes = Vec::new();
            // recognize Buffer Layout
            enum BufferLayout{
                // every attributes has it's own array for storing data 
                SeparatorArray,
                // interleavable data
                #[allow(dead_code)]
                Interleavable,
                // can't recognize
                Other
            }
            let mut buffer_layout = BufferLayout::SeparatorArray;
            for (semantic,accessor) in primitive.attributes() {
                count = accessor.count();
                attributes.push(
                    (Attribute::try_from_gltf_semantic(semantic)
                         .ok_or(Error::UnsupportedSemantic)?,
                     VertexFormat::try_from_gltf(
                         accessor.data_type(),
                         accessor.dimensions()
                     ).ok_or(Error::UnsupportedDataTypeOrDimensions)?
                    )
                );
                let view = accessor.view().ok_or(Error::UnsupportedSparseStorage)?;
                if view.stride().is_some() {
                    buffer_layout = BufferLayout::Other;
                }
            }
            // convert to interleavable data
            let mut data = Vec::new();
            match buffer_layout {
                BufferLayout::SeparatorArray => {
                    let sizes = primitive.attributes()
                        .map(|(_,accessor)| {
                            accessor.size()
                        }).collect::<Vec<_>>();
                    let mut buffer_slices = primitive.attributes()
                        .map(|(_,accessor)| {
                            // Unwrap never fails
                            // we checked before
                            let view = accessor.view().unwrap();
                            let buffer = &buffers[view.buffer().index()];
                            let end = view.offset() + view.length();
                            &buffer[view.offset()..end]
                        }).collect::<Vec<_>>();
                    // copy to data
                    for _ in 0..count {
                        for (size,buffer_slice) in sizes.iter().zip(buffer_slices.iter_mut()) {
                            let mut buffer_slice = *buffer_slice;
                            for _ in 0..*size {
                                let (first,rem) = buffer_slice.split_first().unwrap();
                                data.push(*first);
                                buffer_slice = rem;
                            }
                        }
                    }
                },
                BufferLayout::Interleavable => todo!(),
                BufferLayout::Other => return Err(Error::UnsupportedBufferLayout),
            }
            // prepared all
            // create mesh_resource
            let mut mesh_resource = MeshResource::new();
            mesh_resource.update(indices,unsafe {
                VertexData::new(data,attributes,count)
            });
            mesh_resource.change_assembly(assembly);
            let id = world.create_entity()
                .attach(mesh_resource)
                .into_id();
            meshes.push(id);
        }
    }
    // read node
    for node in gltf.nodes() {
        if let Some(mesh) = node.mesh() {
            let mesh_resource_id = meshes[mesh.index()];
            world.create_entity()
                .attach(Mesh::from_resource(mesh_resource_id))
                .attach(Transform3D::from_gltf_transform(node.transform()));
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
