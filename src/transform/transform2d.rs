use nalgebra_glm as glm;

use super::Transform3D;

#[derive(Debug,Clone,Copy,PartialEq,Eq,PartialOrd,Ord,Hash)]
pub enum Flip {
    Horizontal,
    Vertical,
    All
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Transform2D {
    pub position : glm::TVec2<f32>,
    pub center : glm::TVec2<f32>,
    pub rotate : f32,
    pub flip : Option<Flip>
}

impl Transform2D {
    pub fn new() -> Self {
        Transform2D {
            position: glm::TVec2::zeros(),
            center: glm::TVec2::zeros(),
            rotate: 0.0,
            flip: Option::None,
        }
    }

    pub fn move_to(&mut self,x : f32,y : f32) {
        self.position = glm::TVec2::from_row_slice(&[x,y]);
    }

    pub fn move_by(&mut self,x : f32,y : f32) {
        self.position += glm::TVec2::from_row_slice(&[x,y]);
    }

    pub fn rotate_to(&mut self,angle : f32) {
        self.rotate = angle;
    }

    pub fn rotate_by(&mut self,angle : f32) {
        self.rotate += angle;
    }

    pub fn to_3d(&self) -> Transform3D {
        todo!()
    }

}
