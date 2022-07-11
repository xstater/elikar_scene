use xecs::{EntityId, ComponentRead, World};
use crate::Image;

pub struct Texture {
    pub image: EntityId,
    pub view: [f32; 4],
    pub mag_filter: MagFilter,
    pub min_filter: MinFilter,
    pub wrap_s: Wrap,
    pub wrap_t: Wrap,
}

impl Texture {
    pub fn image_view<'a>(&self,world: &'a World) -> ComponentRead<'a, Image> {
        world.entity_component_read::<_>(self.image).unwrap()
    }
}

#[derive(Debug,Clone,Copy,PartialEq,Eq,Hash)]
pub enum MagFilter{
    Nearest,
    Linear,
}

#[derive(Debug,Clone,Copy,PartialEq,Eq,Hash)]
pub enum MinFilter {
    Nearest,
    Linear,
    NearestMipmapNearest,
    LinearMipmapNearest,
    NearestMipmapLinear,
    LinearMipmapLinear,
}

#[derive(Debug,Clone,Copy,PartialEq,Eq,Hash)]
pub enum Wrap{
    ClampToEdge,
    Repeat,
    MirroredRepeat,
}
