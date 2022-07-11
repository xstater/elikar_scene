use std::{path::Path, borrow::Cow, fs::File};
use gltf::{Gltf, image};
use xecs::{World, EntityId};
use crate::{Image, image::ColorType};
use super::Error;

pub fn load_images(gltf: &Gltf,gltf_path: &Path,world: &World) -> Result<Vec<EntityId>,Error> {
    let mut images = Vec::new();
    for image in gltf.images() {
        match image.source() {
            image::Source::View { .. } => todo!(),
            image::Source::Uri { uri, .. } => {
                let mut path = Cow::Borrowed(gltf_path);
                let image_path = Path::new(uri);
                let image_path = if image_path.is_relative() {
                    path.to_mut().pop();
                    path.to_mut().push(image_path);
                    path.as_ref()
                } else {
                    image_path
                };
                let ext = image_path.extension()
                    .ok_or(Error::UnsupportedImageFormat)?;
                if ext == "png" || ext == "PNG" {
                    let file = File::open(image_path)?;
                    let png_decoder = png::Decoder::new(file);
                    let mut reader = png_decoder.read_info()?;
                    let mut buffer = vec![0;reader.output_buffer_size()];
                    let info = reader.next_frame(&mut buffer)?;
                    if info.color_type == png::ColorType::Rgb &&
                       info.bit_depth == png::BitDepth::Eight {
                        let image = Image {
                            width: info.width,
                            height: info.height,
                            color_type: ColorType::RGB,
                            data: buffer,
                        };
                        let id = world.create_entity()
                            .attach(image)
                            .into_id();
                        images.push(id);
                    } else if info.color_type == png::ColorType::Rgba &&
                              info.bit_depth == png::BitDepth::Eight {
                        let image = Image {
                            width: info.width,
                            height: info.height,
                            color_type: ColorType::RGBA,
                            data: buffer,
                        };
                        let id = world.create_entity()
                            .attach(image)
                            .into_id();
                        images.push(id);
                    } else {
                        return Err(Error::UnsupportedImageFormat);
                    }
                } else if ext == "jpg" || ext == "JPG" || ext == "jpeg" || ext == "JPEG" {
                    let file = File::open(image_path)?;
                    let mut jpeg_decoder = jpeg_decoder::Decoder::new(file);
                    jpeg_decoder.read_info()?;
                    let info = jpeg_decoder.info().unwrap();
                    if info.pixel_format == jpeg_decoder::PixelFormat::RGB24 {
                        let image = Image {
                            width: info.width as _,
                            height: info.height as _,
                            color_type: ColorType::RGB,
                            data: jpeg_decoder.decode()?,
                        };
                        let id = world.create_entity()
                            .attach(image)
                            .into_id();
                        images.push(id);
                    } else {
                        return Err(Error::UnsupportedImageFormat);
                    }
                } else {
                    return Err(Error::UnsupportedImageFormat)
                }
            },
        }
    }
    Ok(images)
}