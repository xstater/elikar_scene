use gltf::accessor::{DataType, Dimensions};
use xecs::entity::EntityId;

pub struct Mesh {
    // An id refer to MeshResource in world
    mesh_resource: EntityId
}

impl Mesh {
    pub fn from_resource(mesh_resource_id : EntityId) -> Self {
        Mesh {
            mesh_resource: mesh_resource_id,
        }
    }

    pub fn mesh_resource_id(&self) -> EntityId {
        self.mesh_resource
    }
}

#[derive(Debug,Clone,Copy,PartialEq)]
pub enum Assembly{
    Points,
    Lines,
    LineLoop,
    LineStrip,
    Triangles,
    TriangleStrip,
    TriangleFan
}

pub struct VertexData {
    data : Vec<u8>, // Just a raw interleavable data
    // for example
    // data:
    // +Attrib+---- Attrib * count
    // +------+----
    // +---+--+----
    // +xyz+uv+----
    // attributes:
    // [(Positions,Flot32x3),(TexCoords(0),Flot32x2)]
    attributes : Vec<(Attribute,VertexFormat)>,
    count : usize
}

impl VertexData {
    /// Safety
    /// Safe only when is correct
    pub unsafe fn new(
        data : Vec<u8>,
        attributes : Vec<(Attribute,VertexFormat)>,
        count : usize) -> Self {

        VertexData {
            data,
            attributes,
            count,
        }
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn attributes(&self) -> &[(Attribute,VertexFormat)] {
        &self.attributes
    }

    pub fn count(&self) -> usize {
        self.count
    }

}

/// Shared resource
pub struct MeshResource {
    assembly: Assembly,
    indices : Option<Vec<u16>>,
    vertices : Option<VertexData>
}

impl MeshResource {
    pub fn new() -> Self {
        MeshResource {
            assembly: Assembly::Triangles,
            indices: None,
            vertices: None,
        }
    }

    pub fn change_assembly(&mut self,assembly : Assembly) {
        self.assembly = assembly;
    }

    pub fn assembly(&self) -> Assembly {
        self.assembly
    }

    pub fn update(&mut self,
                  indices : Option<Vec<u16>>,
                  vertices : VertexData) {
        if let Some(indices) = indices {
            self.indices.replace(indices);
        }
        self.vertices.replace(vertices);
    }

    pub fn has(&self) -> bool {
        self.vertices.is_some()
    }

    pub fn fetch(&mut self)
        -> Option<(Option<Vec<u16>>,VertexData)> {
        let vertices = self.vertices.take()?;
        let indices = self.indices.take();
        Some((indices,vertices))
    }

}

#[derive(Debug,Clone,Copy,PartialEq)]
pub enum Attribute {
    Positions,
    Normals,
    Tangents,
    Color(u32),
    TexCoords(u32),
}

impl Attribute{
    pub(in crate) fn try_from_gltf_semantic(semantic : gltf::Semantic) -> Option<Self> {
        Some(match semantic {
            gltf::Semantic::Positions => Attribute::Positions,
            gltf::Semantic::Normals => Attribute::Normals,
            gltf::Semantic::Tangents => Attribute::Tangents,
            gltf::Semantic::Colors(x) => Attribute::Color(x),
            gltf::Semantic::TexCoords(x) => Attribute::TexCoords(x),
            _ => return None
        })
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum VertexFormat {
    /// Two unsigned bytes (u8). `uvec2` in shaders.
    Uint8x2 = 0,
    /// Four unsigned bytes (u8). `uvec4` in shaders.
    Uint8x4 = 1,
    /// Two signed bytes (i8). `ivec2` in shaders.
    Sint8x2 = 2,
    /// Four signed bytes (i8). `ivec4` in shaders.
    Sint8x4 = 3,
    /// Two unsigned bytes (u8). [0, 255] converted to float [0, 1] `vec2` in shaders.
    Unorm8x2 = 4,
    /// Four unsigned bytes (u8). [0, 255] converted to float [0, 1] `vec4` in shaders.
    Unorm8x4 = 5,
    /// Two signed bytes (i8). [-127, 127] converted to float [-1, 1] `vec2` in shaders.
    Snorm8x2 = 6,
    /// Four signed bytes (i8). [-127, 127] converted to float [-1, 1] `vec4` in shaders.
    Snorm8x4 = 7,
    /// Two unsigned shorts (u16). `uvec2` in shaders.
    Uint16x2 = 8,
    /// Four unsigned shorts (u16). `uvec4` in shaders.
    Uint16x4 = 9,
    /// Two signed shorts (i16). `ivec2` in shaders.
    Sint16x2 = 10,
    /// Four signed shorts (i16). `ivec4` in shaders.
    Sint16x4 = 11,
    /// Two unsigned shorts (u16). [0, 65535] converted to float [0, 1] `vec2` in shaders.
    Unorm16x2 = 12,
    /// Four unsigned shorts (u16). [0, 65535] converted to float [0, 1] `vec4` in shaders.
    Unorm16x4 = 13,
    /// Two signed shorts (i16). [-32767, 32767] converted to float [-1, 1] `vec2` in shaders.
    Snorm16x2 = 14,
    /// Four signed shorts (i16). [-32767, 32767] converted to float [-1, 1] `vec4` in shaders.
    Snorm16x4 = 15,
    /// Two half-precision floats (no Rust equiv). `vec2` in shaders.
    Float16x2 = 16,
    /// Four half-precision floats (no Rust equiv). `vec4` in shaders.
    Float16x4 = 17,
    /// One single-precision float (f32). `float` in shaders.
    Float32 = 18,
    /// Two single-precision floats (f32). `vec2` in shaders.
    Float32x2 = 19,
    /// Three single-precision floats (f32). `vec3` in shaders.
    Float32x3 = 20,
    /// Four single-precision floats (f32). `vec4` in shaders.
    Float32x4 = 21,
    /// One unsigned int (u32). `uint` in shaders.
    Uint32 = 22,
    /// Two unsigned ints (u32). `uvec2` in shaders.
    Uint32x2 = 23,
    /// Three unsigned ints (u32). `uvec3` in shaders.
    Uint32x3 = 24,
    /// Four unsigned ints (u32). `uvec4` in shaders.
    Uint32x4 = 25,
    /// One signed int (i32). `int` in shaders.
    Sint32 = 26,
    /// Two signed ints (i32). `ivec2` in shaders.
    Sint32x2 = 27,
    /// Three signed ints (i32). `ivec3` in shaders.
    Sint32x3 = 28,
    /// Four signed ints (i32). `ivec4` in shaders.
    Sint32x4 = 29,
    /// One double-precision float (f64). `double` in shaders. Requires VERTEX_ATTRIBUTE_64BIT features.
    Float64 = 30,
    /// Two double-precision floats (f64). `dvec2` in shaders. Requires VERTEX_ATTRIBUTE_64BIT features.
    Float64x2 = 31,
    /// Three double-precision floats (f64). `dvec3` in shaders. Requires VERTEX_ATTRIBUTE_64BIT features.
    Float64x3 = 32,
    /// Four double-precision floats (f64). `dvec4` in shaders. Requires VERTEX_ATTRIBUTE_64BIT features.
    Float64x4 = 33,
}

impl VertexFormat {
    /// Returns the byte size of the format.
    pub const fn size(&self) -> u64 {
        match self {
            Self::Uint8x2 | Self::Sint8x2 | Self::Unorm8x2 | Self::Snorm8x2 => 2,
            Self::Uint8x4
            | Self::Sint8x4
            | Self::Unorm8x4
            | Self::Snorm8x4
            | Self::Uint16x2
            | Self::Sint16x2
            | Self::Unorm16x2
            | Self::Snorm16x2
            | Self::Float16x2
            | Self::Float32
            | Self::Uint32
            | Self::Sint32 => 4,
            Self::Uint16x4
            | Self::Sint16x4
            | Self::Unorm16x4
            | Self::Snorm16x4
            | Self::Float16x4
            | Self::Float32x2
            | Self::Uint32x2
            | Self::Sint32x2
            | Self::Float64 => 8,
            Self::Float32x3 | Self::Uint32x3 | Self::Sint32x3 => 12,
            Self::Float32x4 | Self::Uint32x4 | Self::Sint32x4 | Self::Float64x2 => 16,
            Self::Float64x3 => 24,
            Self::Float64x4 => 32,
        }
    }

    pub(in crate) const fn try_from_gltf(data_type : DataType,dimensions : Dimensions) -> 
        Option<Self> {
        match data_type {
            DataType::I8 => 
                match dimensions {
                    Dimensions::Vec2 => Some(Self::Sint8x2),
                    Dimensions::Vec4 => Some(Self::Sint8x4),
                    _ => None
                },
            DataType::U8 => 
                match dimensions {
                    Dimensions::Vec2 => Some(Self::Uint8x2),
                    Dimensions::Vec4 => Some(Self::Uint8x4),
                    _ => None
                },
            DataType::I16 => 
                match dimensions {
                    Dimensions::Vec2 => Some(Self::Sint16x2),
                    Dimensions::Vec4 => Some(Self::Sint16x4),
                    _ => None
                },
            DataType::U16 => 
                match dimensions {
                    Dimensions::Vec2 => Some(Self::Uint16x2),
                    Dimensions::Vec4 => Some(Self::Uint16x4),
                    _ => None
                },
            DataType::U32 => 
                match dimensions {
                    Dimensions::Scalar => Some(Self::Uint32),
                    Dimensions::Vec2 => Some(Self::Uint32x2),
                    Dimensions::Vec3 => Some(Self::Uint32x3),
                    Dimensions::Vec4 => Some(Self::Uint32x4),
                    _ => None
                },
            DataType::F32 => 
                match dimensions {
                    Dimensions::Scalar => Some(Self::Float32),
                    Dimensions::Vec2 => Some(Self::Float32x2),
                    Dimensions::Vec3 => Some(Self::Float32x3),
                    Dimensions::Vec4 => Some(Self::Float32x4),
                    _ => None
                },
        }
    }
}
