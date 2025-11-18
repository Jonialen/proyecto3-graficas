use crate::framebuffer::Color;
use nalgebra_glm::Vec3;
use super::noise::*;
use super::utils::*;

/// Shader del Sol clásico (del ejemplo 1)
pub struct ClassicSunShader;

impl super::planet_shaders::PlanetShader for ClassicSunShader {
    fn fragment(&self, pos: &Vec3, normal: &Vec3, time: f32) -> Color {
        let normalized_pos = pos.normalize();

        let turb_offset = Vec3::new(time * 0.1, time * 0.05, 0.0);
        let turbulence_val = turbulence(normalized_pos * 3.0 + turb_offset, 5, 0);

        let spot_noise = perlin_noise(
            normalized_pos.x * 8.0 + time * 0.2,
            normalized_pos.y * 8.0,
            normalized_pos.z * 8.0,
        );
        let solar_spots = smoothstep(0.65, 0.75, spot_noise);

        let base_temp = 0.7 + turbulence_val * 0.15 - solar_spots * 0.3;
        let temp_color = temperature_to_color(base_temp);

        let pulse = (time * 2.0).sin() * 0.05 + 0.95;
        let emission = temp_color * (1.5 + turbulence_val * 0.5) * pulse;

        let view_dir = Vec3::new(0.0, 0.0, 1.0);
        let fresnel_val = (1.0 - normal.dot(&view_dir).abs()).powf(3.0);
        let corona = Vec3::new(1.0, 0.8, 0.3) * fresnel_val * 0.5;

        let final_color = (emission + corona).component_mul(&Vec3::new(1.2, 1.0, 0.8));
        Color::from_vec3(final_color)
    }
}