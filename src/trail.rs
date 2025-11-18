use nalgebra_glm::Vec3;
use crate::framebuffer::{Framebuffer, Color};
use crate::renderer::Renderer;
use nalgebra_glm::Mat4;

pub struct ShipTrail {
    positions: Vec<Vec3>,
    max_length: usize,
    sample_interval: f32,
    last_sample_time: f32,
}

impl ShipTrail {
    pub fn new(max_length: usize) -> Self {
        Self {
            positions: Vec::new(),
            max_length,
            sample_interval: 0.1,
            last_sample_time: 0.0,
        }
    }

    pub fn update(&mut self, position: Vec3, time: f32) {
        if time - self.last_sample_time >= self.sample_interval {
            self.positions.push(position);
            if self.positions.len() > self.max_length {
                self.positions.remove(0);
            }
            self.last_sample_time = time;
        }
    }

    pub fn render(
        &self,
        framebuffer: &mut Framebuffer,
        renderer: &Renderer,
        view_matrix: &Mat4,
        projection_matrix: &Mat4,
    ) {
        if self.positions.len() < 2 {
            return;
        }

        for i in 0..self.positions.len() - 1 {
            let alpha = (i as f32 / self.positions.len() as f32 * 255.0) as u8;
            let color = Color::new(
                (100.0 * (alpha as f32 / 255.0)) as u8,
                (200.0 * (alpha as f32 / 255.0)) as u8,
                255,
            );

            renderer.render_line(
                framebuffer,
                &self.positions[i],
                &self.positions[i + 1],
                view_matrix,
                projection_matrix,
                color,
            );
        }
    }

    pub fn clear(&mut self) {
        self.positions.clear();
    }
}