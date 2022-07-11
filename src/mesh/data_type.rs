#[repr(C)]
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum DataType {
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

impl DataType {
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

    pub fn try_from_gltf(data_type: gltf::accessor::DataType,dimensions: gltf::accessor::Dimensions) -> Option<Self>{
        Some(match (data_type,dimensions) {
            (gltf::accessor::DataType::U8,gltf::accessor::Dimensions::Vec2) => Self::Uint8x2,
            (gltf::accessor::DataType::U8,gltf::accessor::Dimensions::Vec4) => Self::Uint8x4,

            (gltf::accessor::DataType::I8,gltf::accessor::Dimensions::Vec2) => Self::Sint8x2,
            (gltf::accessor::DataType::I8,gltf::accessor::Dimensions::Vec4) => Self::Sint8x4,

            (gltf::accessor::DataType::U16,gltf::accessor::Dimensions::Vec2) => Self::Uint16x2,
            (gltf::accessor::DataType::U16,gltf::accessor::Dimensions::Vec4) => Self::Uint16x4,

            (gltf::accessor::DataType::I16,gltf::accessor::Dimensions::Vec2) => Self::Sint16x2,
            (gltf::accessor::DataType::I16,gltf::accessor::Dimensions::Vec4) => Self::Sint16x4,

            (gltf::accessor::DataType::U32,gltf::accessor::Dimensions::Scalar) => Self::Uint32,
            (gltf::accessor::DataType::U32,gltf::accessor::Dimensions::Vec2) => Self::Uint32x2,
            (gltf::accessor::DataType::U32,gltf::accessor::Dimensions::Vec3) => Self::Uint32x3,
            (gltf::accessor::DataType::U32,gltf::accessor::Dimensions::Vec4) => Self::Uint32x4,

            (gltf::accessor::DataType::F32,gltf::accessor::Dimensions::Scalar) => Self::Float32,
            (gltf::accessor::DataType::F32,gltf::accessor::Dimensions::Vec2) => Self::Float32x2,
            (gltf::accessor::DataType::F32,gltf::accessor::Dimensions::Vec3) => Self::Float32x3,
            (gltf::accessor::DataType::F32,gltf::accessor::Dimensions::Vec4) => Self::Float32x4,

            _ => return None
        })
    }
}
