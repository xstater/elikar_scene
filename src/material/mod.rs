use xecs::EntityId;

pub struct Material {
    alpha: f32,
    double_sided: bool,
    color_texture: Option<EntityId>,
}