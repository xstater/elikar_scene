pub struct Image {
    pub width: u32,
    pub height: u32,
    pub color_type: ColorType,
    pub data: Vec<u8>
}

#[derive(Debug,Clone,Copy,PartialEq,Eq,Hash)]
pub enum ColorType {
    RGB,
    RGBA,
}