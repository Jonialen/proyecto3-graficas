use nalgebra_glm::Vec3;
use crate::framebuffer::{Framebuffer, Color};

pub struct WarpEffect {
    pub active: bool,
    pub progress: f32,      // 0.0 -> 1.0
    pub duration: f32,      // duración en segundos
    start_pos: Vec3,
    end_pos: Vec3,
    elapsed: f32,
}

impl WarpEffect {
    pub fn new() -> Self {
        Self {
            active: false,
            progress: 0.0,
            duration: 1.5,
            start_pos: Vec3::zeros(),
            end_pos: Vec3::zeros(),
            elapsed: 0.0,
        }
    }

    /// Inicia un warp desde una posición a otra
    pub fn start_warp(&mut self, from: Vec3, to: Vec3, duration: f32) {
        self.active = true;
        self.progress = 0.0;
        self.elapsed = 0.0;
        self.start_pos = from;
        self.end_pos = to;
        self.duration = duration;
    }

    /// Actualiza el estado del warp
    pub fn update(&mut self, delta_time: f32) -> Option<Vec3> {
        if !self.active {
            return None;
        }

        self.elapsed += delta_time;
        self.progress = (self.elapsed / self.duration).min(1.0);

        // Curva de ease-in-out
        let t = self.smooth_step(self.progress);
        let current_pos = self.start_pos + (self.end_pos - self.start_pos) * t;

        if self.progress >= 1.0 {
            self.active = false;
            return Some(self.end_pos); // Posición final
        }

        Some(current_pos)
    }

    /// Función de suavizado (ease in-out cubic)
    fn smooth_step(&self, t: f32) -> f32 {
        if t < 0.5 {
            4.0 * t * t * t
        } else {
            1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
        }
    }

    /// Renderiza el efecto visual del warp
    pub fn render(&self, framebuffer: &mut Framebuffer) {
        if !self.active {
            return;
        }

        let width = framebuffer.width;
        let height = framebuffer.height;
        let center_x = width / 2;
        let center_y = height / 2;

        // Efecto de túnel/vórtice
        let intensity = if self.progress < 0.5 {
            self.progress * 2.0 // Acelera
        } else {
            (1.0 - self.progress) * 2.0 // Decelera
        };

        // Distorsión radial desde el centro
        for y in 0..height {
            for x in 0..width {
                let dx = x as f32 - center_x as f32;
                let dy = y as f32 - center_y as f32;
                let dist = (dx * dx + dy * dy).sqrt();
                let max_dist = (width as f32 * width as f32 + height as f32 * height as f32).sqrt() / 2.0;
                let normalized_dist = dist / max_dist;

                // Líneas radiales pulsantes
                let angle = dy.atan2(dx);
                let line_pattern = (angle * 20.0 + self.elapsed * 10.0).sin() * 0.5 + 0.5;
                
                if normalized_dist > 0.3 && line_pattern > 0.7 {
                    let alpha = (intensity * (1.0 - normalized_dist) * 255.0) as u8;
                    if alpha > 30 {
                        let color = Color::new(
                            (100.0 + intensity * 155.0) as u8,
                            (150.0 + intensity * 105.0) as u8,
                            255,
                        );
                        blend_pixel(framebuffer, x, y, color, alpha);
                    }
                }
            }
        }

        // Flash de fade blanco al inicio/final
        let fade_alpha = if self.progress < 0.1 {
            (self.progress * 10.0 * 200.0) as u8
        } else if self.progress > 0.9 {
            ((1.0 - self.progress) * 10.0 * 200.0) as u8
        } else {
            0
        };

        if fade_alpha > 0 {
            for y in 0..height {
                for x in 0..width {
                    blend_pixel(framebuffer, x, y, Color::new(255, 255, 255), fade_alpha);
                }
            }
        }

        // Efecto de "estrellas" acelerando
        for i in 0..50 {
            let angle = (i as f32 * 0.628) + self.elapsed * 2.0;
            let speed = 200.0 + i as f32 * 10.0;
            let star_dist = speed * intensity;
            
            let star_x = center_x as f32 + angle.cos() * star_dist;
            let star_y = center_y as f32 + angle.sin() * star_dist;

            if star_x >= 0.0 && star_x < width as f32 && star_y >= 0.0 && star_y < height as f32 {
                let star_color = Color::new(200, 220, 255);
                draw_star(framebuffer, star_x as usize, star_y as usize, star_color, 2);
            }
        }
    }

    pub fn is_active(&self) -> bool {
        self.active
    }
}

// Funciones auxiliares
fn blend_pixel(framebuffer: &mut Framebuffer, x: usize, y: usize, color: Color, alpha: u8) {
    if x >= framebuffer.width || y >= framebuffer.height {
        return;
    }
    
    let idx = (y * framebuffer.width + x) * 4;
    let inv_alpha = 255 - alpha;
    
    framebuffer.buffer[idx] = ((framebuffer.buffer[idx] as u16 * inv_alpha as u16 + color.r as u16 * alpha as u16) / 255) as u8;
    framebuffer.buffer[idx + 1] = ((framebuffer.buffer[idx + 1] as u16 * inv_alpha as u16 + color.g as u16 * alpha as u16) / 255) as u8;
    framebuffer.buffer[idx + 2] = ((framebuffer.buffer[idx + 2] as u16 * inv_alpha as u16 + color.b as u16 * alpha as u16) / 255) as u8;
}

fn draw_star(framebuffer: &mut Framebuffer, x: usize, y: usize, color: Color, size: usize) {
    for dy in 0..size {
        for dx in 0..size {
            let px = x + dx;
            let py = y + dy;
            if px < framebuffer.width && py < framebuffer.height {
                framebuffer.set_pixel(px, py, color, 0.0);
            }
        }
    }
}