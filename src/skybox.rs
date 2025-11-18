use crate::framebuffer::{Framebuffer, Color};
use nalgebra_glm::{Vec3, Mat4};
use rand::Rng;

pub struct Skybox {
    stars: Vec<Star>,
}

struct Star {
    direction: Vec3,
    brightness: u8,
    size: u8,
}

impl Skybox {
    pub fn new(star_count: usize) -> Self {
        let mut rng = rand::rng();
        let mut stars = Vec::with_capacity(star_count);

        for _ in 0..star_count {
            let theta = rng.random_range(0.0..std::f32::consts::PI * 2.0);
            let phi = rng.random_range(0.0..std::f32::consts::PI);

            let direction = Vec3::new(
                phi.sin() * theta.cos(),
                phi.sin() * theta.sin(),
                phi.cos(),
            )
            .normalize();

            stars.push(Star {
                direction,
                brightness: rng.random_range(150..255),
                size: rng.random_range(1..3),
            });
        }

        Self { stars }
    }

    pub fn render(
        &self,
        framebuffer: &mut Framebuffer,
        view_matrix: &Mat4,
        projection_matrix: &Mat4,
        width: f32,
        height: f32,
    ) {
        let mut rotation_only = *view_matrix;
        rotation_only[(0, 3)] = 0.0;
        rotation_only[(1, 3)] = 0.0;
        rotation_only[(2, 3)] = 0.0;

        let vp = projection_matrix * rotation_only;

        for star in &self.stars {
            let far_point = star.direction * 10000.0;
            let pos4 = nalgebra_glm::Vec4::new(
                far_point.x,
                far_point.y,
                far_point.z,
                1.0,
            );
            let clip_pos = vp * pos4;

            let w = clip_pos.w;
            if w <= 0.0 {
                continue;
            }

            let ndc = clip_pos.xyz() / w;
            
            if ndc.z < -1.0 || ndc.z > 1.0 {
                continue;
            }

            let screen_x = ((ndc.x + 1.0) * 0.5 * width) as usize;
            let screen_y = ((1.0 - ndc.y) * 0.5 * height) as usize;

            if screen_x < width as usize && screen_y < height as usize {
                let color = Color::new(
                    star.brightness,
                    star.brightness,
                    star.brightness,
                );

                for dx in 0..star.size {
                    for dy in 0..star.size {
                        let x = screen_x + dx as usize;
                        let y = screen_y + dy as usize;
                        if x < width as usize && y < height as usize {
                            framebuffer.set_pixel(x, y, color, 0.999);
                        }
                    }
                }
            }
        }
    }
}