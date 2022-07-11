use std::{io, fmt::Display};

#[derive(Debug)]
pub enum Error {
    GltfError(gltf::Error),
    IoError(io::Error),
    UnsupportedIndicesFormat,
    UnsupportedSemantic(gltf::Semantic),
    UnsupportedDataTypeOrDimensions,
    UnsupportedBufferLayout,
    UnsupportedImageFormat,
    UnsupportedSparseStorage,
    PngDecodingError(png::DecodingError),
    JpegDecodingError(jpeg_decoder::Error)
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

impl From<png::DecodingError> for Error {
    fn from(err: png::DecodingError) -> Self {
        Error::PngDecodingError(err)
    }
}

impl From<jpeg_decoder::Error> for Error {
    fn from(err: jpeg_decoder::Error) -> Self {
        Error::JpegDecodingError(err)
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
            Error::UnsupportedSemantic(semantic) =>
                write!(f,"Loading GLTF file failed: The semantic {:?} is not supported",semantic),
            Error::UnsupportedDataTypeOrDimensions =>
                write!(f,"Loading GLTF file failed: The data type or Dimensions are not supported"),
            Error::UnsupportedBufferLayout =>
                write!(f,"Loading GLTF file failed: The buffer layout is not supported"),
            Error::UnsupportedImageFormat =>
                write!(f,"Loading GLTF file failed: The image format is not supported"),
            Error::UnsupportedSparseStorage =>
                write!(f,"Loading GLTF file failed: The sparse feature is not supported"),
            Error::PngDecodingError(err) =>
                write!(f,"Loading GLTF file failed: {}",err),
            Error::JpegDecodingError(err) =>
                write!(f,"Loading GLTF file failed: {}",err),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::GltfError(err) => Some(err),
            Error::IoError(err) => Some(err),
            Error::PngDecodingError(err) => Some(err),
            Error::JpegDecodingError(err) => Some(err),
            _ => None
        }
    }
}