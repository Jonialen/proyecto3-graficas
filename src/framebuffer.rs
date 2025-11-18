use nalgebra_glm::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    // CONSTANTES AGREGADAS
    pub const BLACK: Color = Color { r: 0, g: 0, b: 0 };
    pub const WHITE: Color = Color { r: 255, g: 255, b: 255 };

    #[inline]
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b }
    }

    #[inline]
    pub fn from_vec3(v: Vec3) -> Self {
        Color {
            r: (v.x.clamp(0.0, 1.0) * 255.0) as u8,
            g: (v.y.clamp(0.0, 1.0) * 255.0) as u8,
            b: (v.z.clamp(0.0, 1.0) * 255.0) as u8,
        }
    }

    #[inline]
    pub fn to_vec3(&self) -> Vec3 {
        Vec3::new(
            self.r as f32 / 255.0,
            self.g as f32 / 255.0,
            self.b as f32 / 255.0,
        )
    }

    #[inline]
    pub fn to_raylib(&self) -> raylib::color::Color {
        raylib::color::Color::new(self.r, self.g, self.b, 255)
    }
}

pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<u8>,
    pub zbuffer: Vec<f32>,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Framebuffer {
            width,
            height,
            buffer: vec![0; width * height * 4],
            zbuffer: vec![f32::INFINITY; width * height],
        }
    }

    #[inline]
    pub fn clear(&mut self, color: Color) {
        for i in 0..self.width * self.height {
            let idx = i * 4;
            self.buffer[idx] = color.r;
            self.buffer[idx + 1] = color.g;
            self.buffer[idx + 2] = color.b;
            self.buffer[idx + 3] = 255;
        }
        // IMPORTANTE: Limpiar z-buffer correctamente
        self.zbuffer.fill(1.0); // Usar 1.0 en vez de INFINITY para mejor compatibilidad
    }

    #[inline]
    pub fn set_pixel(&mut self, x: usize, y: usize, color: Color, depth: f32) {
        if x >= self.width || y >= self.height {
            return;
        }

        // NUEVO: Validar profundidad antes de escribir
        if !depth.is_finite() || depth < -1.0 || depth > 1.0 {
            return;
        }

        let index = y * self.width + x;

        if depth < self.zbuffer[index] {
            self.zbuffer[index] = depth;
            let idx = index * 4;
            self.buffer[idx] = color.r;
            self.buffer[idx + 1] = color.g;
            self.buffer[idx + 2] = color.b;
            self.buffer[idx + 3] = 255;
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.buffer
    }
}