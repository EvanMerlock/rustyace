use nalgebra_glm as glm;

pub fn radians(degree: f32) -> f32 {
    degree * (glm::pi::<f32>() / (180.0f32))
}