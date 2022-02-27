use nalgebra_glm as glm;

pub struct Camera3D {
    projection : glm::TMat4<f32>
}

impl Camera3D {
    pub fn perspective(aspect : f32,fov_y : f32,near : f32,far : f32) -> Self {
        Camera3D {
            projection: glm::perspective(aspect, fov_y, near, far),
        }
    }

    pub fn ortho(left : f32,right : f32,bottom : f32,top : f32,near : f32,far : f32) -> Self {
        Camera3D {
            projection: glm::ortho(left, right, bottom, top, near, far),
        }
    }

    pub fn projection_matrix(&self) -> &glm::TMat4<f32> {
        &self.projection
    }
}
