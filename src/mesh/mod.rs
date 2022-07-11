use xecs::{EntityId, World, ComponentRead, ComponentWrite};

mod vertices;
mod attributes;
mod data_type;

pub use vertices::Vertices;
pub use data_type::DataType;
pub use attributes::{
    Attribute,
    AttributeType
};

pub struct Mesh {
    /// Mesh data id
    data_id: EntityId
}

impl Mesh {
    pub fn from_data(data_id: EntityId) -> Self {
        Self {
            data_id
        }
    }

    pub fn mesh_data_id(&self) -> EntityId {
        self.data_id
    }
    
    pub fn mesh_data_read<'a>(&self,world: &'a World) -> ComponentRead<'a, MeshData> {
        world.entity_component_read(self.data_id).unwrap()
    }

    pub fn mesh_data_write<'a>(&self,world: &'a World) -> ComponentWrite<'a, MeshData> {
        world.entity_component_write(self.data_id).unwrap()
    }
}

/// Resource
#[derive(Debug,Clone)]
pub struct MeshData {
    pub data: Vec<u8>,
    pub indices: Option<Indices>,
    pub vertices: Vertices,
    pub material: EntityId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Assembly {
    Points,
    Lines,
    LineLoop,
    LineStrip,
    Triangles,
    TriangleStrip,
    TriangleFan,
}


#[derive(Debug,Clone)]
pub enum Indices{
    U16(Vec<u16>),
    U32(Vec<u32>)
}

