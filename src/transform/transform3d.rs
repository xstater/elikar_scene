use nalgebra_glm as glm;
use super::Transform2D;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Transform3D {
    pub translation: glm::TVec3<f32>,
    pub center: glm::TVec3<f32>,
    pub scale: glm::TVec3<f32>,
    pub rotation: glm::Qua<f32>,
}

impl Transform3D {
    pub(in crate) fn from_gltf_transform(transform : gltf::scene::Transform) -> Self {
        match transform {
            #[allow(unused)]
            gltf::scene::Transform::Matrix { matrix } => todo!(),
            gltf::scene::Transform::Decomposed { translation, rotation, scale } => {
                let mut transform = Transform3D::new();
                transform.translation = glm::make_vec3(&translation);
                transform.rotation = glm::make_quat(&rotation);
                transform.scale = glm::make_vec3(&scale);
                transform
            },
        }
    }

    pub fn new() -> Self {
        Transform3D {
            translation: glm::make_vec3(&[0.0, 0.0, 0.0]),
            center: glm::make_vec3(&[0.0, 0.0, 0.0]),
            scale: glm::make_vec3(&[1.0, 1.0, 1.0]),
            rotation: glm::quat_identity(),
        }
    }

    pub fn translation_matrix(&self) -> glm::TMat4<f32> {
        glm::translation(&self.translation)
    }

    pub fn look_at(&mut self,position : &Transform3D){
        let direction = position.translation - self.translation;
        let up = glm::vec4(0.0,0.0,1.0,0.0);
        let up = glm::quat_to_mat4(&self.rotation) * up;
        self.rotation = glm::quat_look_at(&direction, &up.xyz());
    }

    pub fn move_to(&mut self, x: f32, y: f32, z: f32) {
        self.translation = glm::make_vec3(&[x, y, z]);
    }

    pub fn move_by(&mut self, dx: f32, dy: f32, dz: f32) {
        self.translation += glm::make_vec3(&[dx, dy, dz]);
    }

    pub fn scale_matrix(&self) -> glm::TMat4<f32> {
        glm::scaling(&self.scale)
    }

    pub fn scale_to(&mut self, fx: f32, fy: f32, fz: f32) {
        self.scale = glm::make_vec3(&[fx, fy, fz]);
    }

    pub fn flip_x(&mut self) {
        let x = unsafe { self.scale.get_unchecked_mut(0) };
        *x = -*x;
    }

    pub fn flip_y(&mut self) {
        let y = unsafe { self.scale.get_unchecked_mut(1) };
        *y = -*y;
    }

    pub fn flip_z(&mut self) {
        let z = unsafe { self.scale.get_unchecked_mut(2) };
        *z = -*z;
    }

    pub fn rotation_matrix(&self) -> glm::TMat4<f32> {
        glm::quat_to_mat4(&self.rotation)
    }

    pub fn yaw(&self) -> f32 {
        glm::quat_yaw(&self.rotation)
    }

    pub fn pitch(&self) -> f32 {
        glm::quat_pitch(&self.rotation)
    }

    pub fn roll(&self) -> f32 {
        glm::quat_roll(&self.rotation)
    }

    pub fn rotate_x_by(&mut self, angle: f32) {
        self.rotate_by((1.0, 0.0, 0.0), angle)
    }

    pub fn rotate_y_by(&mut self, angle: f32) {
        self.rotate_by((0.0, 1.0, 0.0), angle)
    }

    pub fn rotate_z_by(&mut self, angle: f32) {
        self.rotate_by((0.0, 0.0, 1.0), angle)
    }

    pub fn rotate_by(&mut self, axis: (f32, f32, f32), angle: f32) {
        self.rotation = glm::quat_rotate(
            &self.rotation,
            angle,
            &glm::make_vec3(&[axis.0, axis.1, axis.2]),
        );
    }

    pub fn rotate_to(&mut self, axis: (f32, f32, f32), angle: f32) {
        self.rotation = glm::quat_rotate(
            &glm::quat_identity(),
            angle,
            &glm::make_vec3(&[axis.0, axis.1, axis.2]),
        );
    }

    pub fn model_matrix(&self) -> glm::TMat4<f32> {
        // move center to (0,0,0)
        let center = glm::translation(&-self.center);
        // scale
        let scale = self.scale_matrix() * center;
        // rotate the model
        let rotation = self.rotation_matrix() * scale;
        // move center back
        let center_back = glm::translation(&self.center) * rotation;
        // translation
        let translation = self.translation_matrix() * center_back;
        return translation;
    }

    pub fn to_2d(&self) -> Transform2D {
        Transform2D {
            position: self.translation.xy(),
            center: self.center.xy(),
            rotate: self.yaw(),
            flip: Option::None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::transform::Transform3D;

    #[test]
    fn test() {
        let mut trans = Transform3D::new();
        trans.flip_x();
        assert_eq!(trans.scale.as_slice(), &[-1.0,1.0,1.0]);
        trans.rotate_z_by(43_f32.to_radians());
        assert_eq!(trans.pitch().to_degrees(), 43_f32);
    }

}
