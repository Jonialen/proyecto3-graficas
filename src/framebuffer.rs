use nalgebra_glm::Vec3;

/// Representa un color RGB de 8 bits por canal.
///
/// Esta estructura se utiliza tanto para operaciones de rasterización internas
/// como para conversión a tipos de color utilizados por otras librerías (por ejemplo, Raylib).
#[derive(Debug, Clone, Copy)]
pub struct Color {
    /// Componente de rojo (0–255).
    pub r: u8,
    /// Componente de verde (0–255).
    pub g: u8,
    /// Componente de azul (0–255).
    pub b: u8,
}

impl Color {
    /// Color constante: negro puro.
    pub const BLACK: Color = Color { r: 0, g: 0, b: 0 };

    /// Crea un nuevo color desde componentes RGB explícitas.
    #[inline]
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b }
    }

    /// Convierte un vector 3D con valores normalizados `[0.0, 1.0]`
    /// en un color RGB de 8 bits por canal.
    #[inline]
    pub fn from_vec3(v: Vec3) -> Self {
        Color {
            r: (v.x.clamp(0.0, 1.0) * 255.0) as u8,
            g: (v.y.clamp(0.0, 1.0) * 255.0) as u8,
            b: (v.z.clamp(0.0, 1.0) * 255.0) as u8,
        }
    }

    /// Convierte este color al tipo `raylib::color::Color`
    /// para su utilización en la API de Raylib.
    #[inline]
    pub fn to_raylib(&self) -> raylib::color::Color {
        raylib::color::Color::new(self.r, self.g, self.b, 255)
    }
}

/// Framebuffer de software utilizado para el renderizado manual por píxeles.
///
/// Contiene dos buffers paralelos:
/// - `buffer`: almacena los valores de color RGBA.
/// - `zbuffer`: gestiona la profundidad por píxel para el z-test.
///
/// Los valores de profundidad se comparan directamente en espacio NDC,
/// donde -1.0 representa el plano cercano (cerca de la cámara)
/// y 1.0 el plano lejano.
pub struct Framebuffer {
    /// Ancho de la imagen en píxeles.
    pub width: usize,
    /// Alto de la imagen en píxeles.
    pub height: usize,
    /// Buffer de color (RGBA) en formato lineal.
    pub buffer: Vec<u8>,
    /// Buffer de profundidad (z-buffer) con un valor por píxel.
    pub zbuffer: Vec<f32>,
}

impl Framebuffer {
    /// Crea un nuevo framebuffer vacío de las dimensiones indicadas.
    ///
    /// Inicializa el z-buffer con `f32::INFINITY` (sin profundidad asignada)
    /// y el buffer de color con valores en negro.
    pub fn new(width: usize, height: usize) -> Self {
        Framebuffer {
            width,
            height,
            buffer: vec![0; width * height * 4],
            zbuffer: vec![f32::INFINITY; width * height],
        }
    }

    /// Limpia el contenido del framebuffer con un color uniforme.
    ///
    /// También reinicia el z-buffer estableciendo todos los valores
    /// a `f32::INFINITY`.
    #[inline]
    pub fn clear(&mut self, color: Color) {
        for i in 0..self.width * self.height {
            let idx = i * 4;
            self.buffer[idx] = color.r;
            self.buffer[idx + 1] = color.g;
            self.buffer[idx + 2] = color.b;
            self.buffer[idx + 3] = 255;
        }
        self.zbuffer.fill(f32::INFINITY);
    }

    /// Establece el color de un píxel específico en el framebuffer, aplicando z-test.
    ///
    /// Si el valor de profundidad (`depth`) es menor que el actual en el z-buffer,
    /// el píxel se actualiza. De lo contrario, el fragmento se descarta.
    ///
    /// # Parámetros
    /// * `x` - Coordenada X del píxel.
    /// * `y` - Coordenada Y del píxel.
    /// * `color` - Color a escribir.
    /// * `depth` - Valor de profundidad Z (en espacio NDC).
    #[inline]
    pub fn set_pixel(&mut self, x: usize, y: usize, color: Color, depth: f32) {
        if x >= self.width || y >= self.height {
            return; // Fuera de límites
        }

        // Validación de profundidad finita.
        if !depth.is_finite() {
            return;
        }

        let index = y * self.width + x;

        // Comparación de profundidad (z-test estándar).
        // En NDC: -1.0 (cerca) → 1.0 (lejos).
        if depth < self.zbuffer[index] {
            self.zbuffer[index] = depth;
            let idx = index * 4;
            self.buffer[idx] = color.r;
            self.buffer[idx + 1] = color.g;
            self.buffer[idx + 2] = color.b;
            self.buffer[idx + 3] = 255;
        }
    }

    /// Retorna el buffer de color como una porción de bytes (`&[u8]`).
    ///
    /// Permite subir el framebuffer a texturas o librerías externas sin copiar memoria.
    pub fn as_bytes(&self) -> &[u8] {
        &self.buffer
    }
}