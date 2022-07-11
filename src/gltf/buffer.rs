use std::{path::Path, borrow::Cow, fs::File, io::Read};
use gltf::{Gltf, buffer};
use super::Error;


pub fn load_buffers(gltf: &Gltf,gltf_path: &Path) -> Result<Vec<Vec<u8>>, Error> {
    let mut buffers = Vec::new();
    for buffer in gltf.buffers() {
        match buffer.source() {
            buffer::Source::Bin => todo!(),
            buffer::Source::Uri(buffer_path) => {
                let mut path = Cow::Borrowed(gltf_path);
                let buffer_path = Path::new(buffer_path);
                let buffer_path = if buffer_path.is_relative() {
                    path.to_mut().pop();
                    path.to_mut().push(buffer_path);
                    path.as_ref()
                } else {
                    buffer_path
                };
                let mut file = File::open(buffer_path)?;
                let mut v = Vec::new();
                file.read_to_end(&mut v)?;
                buffers.push(v);
            },
        }
    }
    Ok(buffers)
}
