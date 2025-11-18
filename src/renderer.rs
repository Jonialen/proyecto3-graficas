use crate::framebuffer::{Framebuffer, Color};
use crate::mesh::{ObjMesh, Vertex};
use crate::shaders::PlanetShader;
use nalgebra_glm::{Mat4, Vec2, Vec3, Vec4};

pub struct Renderer {
    pub width: f32,
    pub height: f32,
}

impl Renderer {
    pub fn new(width: usize, height: usize) -> Self {
        Renderer {
            width: width as f32,
            height: height as f32,
        }
    }

    fn is_valid_vertex(v: &TransformedVertex) -> bool {
        v.screen_pos.x.is_finite() 
            && v.screen_pos.y.is_finite()
            && v.depth.is_finite()
            && v.world_pos.x.is_finite()
            && v.world_pos.y.is_finite()
            && v.world_pos.z.is_finite()
    }

    pub fn render_mesh(
        &self,
        framebuffer: &mut Framebuffer,
        mesh: &ObjMesh,
        shader: &dyn PlanetShader,
        model_matrix: &Mat4,
        view_matrix: &Mat4,
        projection_matrix: &Mat4,
        time: f32,
    ) {
        let mvp = projection_matrix * view_matrix * model_matrix;

        let transformed_vertices: Vec<_> = mesh
            .vertices
            .iter()
            .map(|v| self.transform_vertex(v, model_matrix, &mvp))
            .collect();

        for i in (0..mesh.indices.len()).step_by(3) {
            let i0 = mesh.indices[i] as usize;
            let i1 = mesh.indices[i + 1] as usize;
            let i2 = mesh.indices[i + 2] as usize;

            if i0 < transformed_vertices.len()
                && i1 < transformed_vertices.len()
                && i2 < transformed_vertices.len()
            {
                self.rasterize_triangle(
                    framebuffer,
                    &transformed_vertices[i0],
                    &transformed_vertices[i1],
                    &transformed_vertices[i2],
                    shader,
                    time,
                );
            }
        }
    }

    pub fn render_mesh_with_bias(
        &self,
        framebuffer: &mut Framebuffer,
        mesh: &ObjMesh,
        shader: &dyn PlanetShader,
        model_matrix: &Mat4,
        view_matrix: &Mat4,
        projection_matrix: &Mat4,
        time: f32,
        depth_bias: f32,
    ) {
        let mvp = projection_matrix * view_matrix * model_matrix;

        let transformed_vertices: Vec<_> = mesh
            .vertices
            .iter()
            .map(|v| self.transform_vertex(v, model_matrix, &mvp))
            .collect();

        for i in (0..mesh.indices.len()).step_by(3) {
            let i0 = mesh.indices[i] as usize;
            let i1 = mesh.indices[i + 1] as usize;
            let i2 = mesh.indices[i + 2] as usize;

            if i0 < transformed_vertices.len()
                && i1 < transformed_vertices.len()
                && i2 < transformed_vertices.len()
            {
                self.rasterize_triangle_with_bias(
                    framebuffer,
                    &transformed_vertices[i0],
                    &transformed_vertices[i1],
                    &transformed_vertices[i2],
                    shader,
                    time,
                    depth_bias,
                );
            }
        }
    }

    fn rasterize_triangle_with_bias(
        &self,
        framebuffer: &mut Framebuffer,
        v0: &TransformedVertex,
        v1: &TransformedVertex,
        v2: &TransformedVertex,
        shader: &dyn PlanetShader,
        time: f32,
        depth_bias: f32,
    ) {                
        if !Self::is_valid_vertex(v0) 
            || !Self::is_valid_vertex(v1) 
            || !Self::is_valid_vertex(v2) {
            return;
        }

        // Back-face culling
        let edge1 = Vec2::new(
            v1.screen_pos.x - v0.screen_pos.x,
            v1.screen_pos.y - v0.screen_pos.y,
        );
        let edge2 = Vec2::new(
            v2.screen_pos.x - v0.screen_pos.x,
            v2.screen_pos.y - v0.screen_pos.y,
        );
        let cross = edge1.x * edge2.y - edge1.y * edge2.x;
        
        if cross <= 0.0 {
            return;
        }

        // Validación de profundidad
        if v0.depth < -1.0 || v0.depth > 1.0 ||
            v1.depth < -1.0 || v1.depth > 1.0 ||
            v2.depth < -1.0 || v2.depth > 1.0 {
            return;
        }

        let min_x = v0.screen_pos.x.min(v1.screen_pos.x).min(v2.screen_pos.x)
            .floor().max(0.0) as usize;
        let max_x = v0.screen_pos.x.max(v1.screen_pos.x).max(v2.screen_pos.x)
            .ceil().min(self.width - 1.0) as usize;
        let min_y = v0.screen_pos.y.min(v1.screen_pos.y).min(v2.screen_pos.y)
            .floor().max(0.0) as usize;
        let max_y = v0.screen_pos.y.max(v1.screen_pos.y).max(v2.screen_pos.y)
            .ceil().min(self.height - 1.0) as usize;

        if min_x >= max_x || min_y >= max_y {
            return;
        }

        let bbox_width = max_x - min_x;
        let bbox_height = max_y - min_y;
        if bbox_width > self.width as usize * 2 || bbox_height > self.height as usize * 2 {
            return;
        }

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let p = Vec2::new(x as f32 + 0.5, y as f32 + 0.5);

                let (w0, w1, w2) = barycentric(
                    &p,
                    &v0.screen_pos,
                    &v1.screen_pos,
                    &v2.screen_pos
                );

                if w0 >= 0.0 && w1 >= 0.0 && w2 >= 0.0 {
                    let depth = w0 * v0.depth + w1 * v1.depth + w2 * v2.depth;
                    
                    // Aplicar depth bias (valores negativos = más cerca de la cámara)
                    let biased_depth = depth + depth_bias;
                    
                    if !biased_depth.is_finite() || biased_depth < -1.0 || biased_depth > 1.0 {
                        continue;
                    }

                    let world_pos = v0.world_pos * w0 
                        + v1.world_pos * w1 
                        + v2.world_pos * w2;
                    
                    if !world_pos.x.is_finite() 
                        || !world_pos.y.is_finite() 
                        || !world_pos.z.is_finite() {
                        continue;
                    }

                    let world_normal = (v0.world_normal * w0 
                        + v1.world_normal * w1 
                        + v2.world_normal * w2)
                        .normalize();

                    let color = shader.fragment(&world_pos, &world_normal, time);
                    
                    // Usar el depth con bias para el z-buffer
                    framebuffer.set_pixel(x, y, color, biased_depth);
                }
            }
        }
    }

    pub fn render_orbit(
        &self,
        framebuffer: &mut Framebuffer,
        orbit_points: &[Vec3],
        parent_position: Vec3,
        view_matrix: &Mat4,
        projection_matrix: &Mat4,
        color: Color,
    ) {
        if orbit_points.len() < 2 {
            return;
        }

        let vp = projection_matrix * view_matrix;

        let screen_points: Vec<Option<Vec2>> = orbit_points
            .iter()
            .map(|point| {
                let world_pos = parent_position + *point;
                self.project_point(&world_pos, &vp)
            })
            .collect();

        for i in 0..screen_points.len() {
            let next_i = (i + 1) % screen_points.len();

            if let (Some(p1), Some(p2)) = (&screen_points[i], &screen_points[next_i]) {
                self.draw_line(framebuffer, p1, p2, color);
            }
        }
    }

    fn project_point(&self, point: &Vec3, vp: &Mat4) -> Option<Vec2> {
        let pos4 = Vec4::new(point.x, point.y, point.z, 1.0);
        let clip_pos = vp * pos4;

        let w = clip_pos.w;
        if w.abs() < 1e-6 || w < 0.0 {
            return None;
        }

        let ndc = clip_pos.xyz() / w;

        if ndc.z < -1.0 || ndc.z > 1.0 {
            return None;
        }

        let screen_x = (ndc.x + 1.0) * 0.5 * self.width;
        let screen_y = (1.0 - ndc.y) * 0.5 * self.height;

        if screen_x >= 0.0 && screen_x < self.width && screen_y >= 0.0 && screen_y < self.height {
            Some(Vec2::new(screen_x, screen_y))
        } else {
            None
        }
    }

    fn draw_line(&self, framebuffer: &mut Framebuffer, p1: &Vec2, p2: &Vec2, color: Color) {
        let mut x0 = p1.x as i32;
        let mut y0 = p1.y as i32;
        let x1 = p2.x as i32;
        let y1 = p2.y as i32;

        let dx = (x1 - x0).abs();
        let dy = -(y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;

        loop {
            if x0 >= 0 && x0 < self.width as i32 && y0 >= 0 && y0 < self.height as i32 {
                framebuffer.set_pixel(x0 as usize, y0 as usize, color, 0.0);
            }

            if x0 == x1 && y0 == y1 {
                break;
            }

            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x0 += sx;
            }
            if e2 <= dx {
                err += dx;
                y0 += sy;
            }
        }
    }

    fn transform_vertex(
        &self,
        vertex: &Vertex,
        model_matrix: &Mat4,
        mvp: &Mat4,
    ) -> TransformedVertex {
        let pos4 = Vec4::new(vertex.position.x, vertex.position.y, vertex.position.z, 1.0);

        let world_pos = model_matrix * pos4;
        let normal4 = Vec4::new(vertex.normal.x, vertex.normal.y, vertex.normal.z, 0.0);
        let world_normal = (model_matrix * normal4).xyz().normalize();

        let clip_pos = mvp * pos4;

        let w = clip_pos.w;
        if w.abs() < 1e-6 {
            return TransformedVertex {
                screen_pos: Vec2::new(-1000.0, -1000.0),
                depth: 1.0,
                world_pos: world_pos.xyz(),
                world_normal,
            };
        }
        let ndc = clip_pos.xyz() / w;

        let screen = Vec2::new(
            (ndc.x + 1.0) * 0.5 * self.width,
            (1.0 - ndc.y) * 0.5 * self.height,
        );

        TransformedVertex {
            screen_pos: screen,
            depth: ndc.z,
            world_pos: world_pos.xyz(),
            world_normal,
        }
    }

    fn rasterize_triangle(
        &self,
        framebuffer: &mut Framebuffer,
        v0: &TransformedVertex,
        v1: &TransformedVertex,
        v2: &TransformedVertex,
        shader: &dyn PlanetShader,
        time: f32,
    ) {                
        if !Self::is_valid_vertex(v0) 
            || !Self::is_valid_vertex(v1) 
            || !Self::is_valid_vertex(v2) {
            return;
        }

        // Back-face culling
        let edge1 = Vec2::new(
            v1.screen_pos.x - v0.screen_pos.x,
            v1.screen_pos.y - v0.screen_pos.y,
        );
        let edge2 = Vec2::new(
            v2.screen_pos.x - v0.screen_pos.x,
            v2.screen_pos.y - v0.screen_pos.y,
        );
        let cross = edge1.x * edge2.y - edge1.y * edge2.x;
        
        if cross <= 0.0 {
            return;
        }

        // ✅ MEJORADO: Validación más robusta de profundidad
        if v0.depth < -1.0 || v0.depth > 1.0 ||
            v1.depth < -1.0 || v1.depth > 1.0 ||
            v2.depth < -1.0 || v2.depth > 1.0 {
            return;
        }

        let min_x = v0.screen_pos.x.min(v1.screen_pos.x).min(v2.screen_pos.x)
            .floor().max(0.0) as usize;
        let max_x = v0.screen_pos.x.max(v1.screen_pos.x).max(v2.screen_pos.x)
            .ceil().min(self.width - 1.0) as usize;
        let min_y = v0.screen_pos.y.min(v1.screen_pos.y).min(v2.screen_pos.y)
            .floor().max(0.0) as usize;
        let max_y = v0.screen_pos.y.max(v1.screen_pos.y).max(v2.screen_pos.y)
            .ceil().min(self.height - 1.0) as usize;

        if min_x >= max_x || min_y >= max_y {
            return;
        }

        let bbox_width = max_x - min_x;
        let bbox_height = max_y - min_y;
        if bbox_width > self.width as usize * 2 || bbox_height > self.height as usize * 2 {
            return;
        }

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let p = Vec2::new(x as f32 + 0.5, y as f32 + 0.5);

                let (w0, w1, w2) = barycentric(
                    &p,
                    &v0.screen_pos,
                    &v1.screen_pos,
                    &v2.screen_pos
                );

                if w0 >= 0.0 && w1 >= 0.0 && w2 >= 0.0 {
                    // ✅ CORRECTO: Interpolar depth en NDC space
                    let depth = w0 * v0.depth + w1 * v1.depth + w2 * v2.depth;
                    
                    // Validación final
                    if !depth.is_finite() || depth < -1.0 || depth > 1.0 {
                        continue;
                    }

                    let world_pos = v0.world_pos * w0 
                        + v1.world_pos * w1 
                        + v2.world_pos * w2;
                    
                    if !world_pos.x.is_finite() 
                        || !world_pos.y.is_finite() 
                        || !world_pos.z.is_finite() {
                        continue;
                    }

                    let world_normal = (v0.world_normal * w0 
                        + v1.world_normal * w1 
                        + v2.world_normal * w2)
                        .normalize();

                    let color = shader.fragment(&world_pos, &world_normal, time);
                    
                    // ✅ Pasar depth directamente (sin normalizar)
                    framebuffer.set_pixel(x, y, color, depth);
                }
            }
        }
    }   
        
    pub fn is_in_frustum(
        &self,
        object_position: &Vec3,
        object_radius: f32,
        view_matrix: &Mat4,
        projection_matrix: &Mat4,
    ) -> bool {
        let vp = projection_matrix * view_matrix;
        let pos4 = Vec4::new(object_position.x, object_position.y, object_position.z, 1.0);
        let clip_pos = vp * pos4;

        let view_pos = view_matrix * pos4;
        
        // CAMBIADO: Permitir objetos grandes detrás de la cámara
        // si parte de ellos podría ser visible
        if view_pos.z > object_radius * 2.0 {
            return false;
        }

        let w = clip_pos.w;
        // if w <= 0.0 && view_pos.z.abs() > object_radius {
        if w <= 0.0 {
            return false; // Solo cullear si está completamente detrás
        }

        // Margen más generoso para objetos grandes
        let screen_size = object_radius / w.abs();
        let margin = (screen_size * 2.0).max(1.0).min(20.0);
        
        // Verificar si está dentro del frustum con margen
        let x_test = clip_pos.x.abs() < w.abs() * (1.0 + margin);
        let y_test = clip_pos.y.abs() < w.abs() * (1.0 + margin);
        let z_test = clip_pos.z > -w.abs() * (1.0 + margin) 
            && clip_pos.z < w.abs();
        
        x_test && y_test && z_test
    }

    // NUEVO: Verificar si un objeto está demasiado cerca de la cámara
    pub fn is_too_close_to_camera(
        &self,
        object_position: &Vec3,
        object_radius: f32,
        camera_position: &Vec3,
        min_distance_multiplier: f32,
    ) -> bool {
        let distance = (object_position - camera_position).magnitude();
        distance < object_radius * min_distance_multiplier
    }

    pub fn calculate_lod(&self, distance: f32) -> u32 {
        if distance < 5.0 {
            64
        } else if distance < 20.0 {
            32
        } else if distance < 50.0 {
            16
        } else {
            8
        }
    }

    pub fn render_line(
        &self,
        framebuffer: &mut Framebuffer,
        start: &Vec3,
        end: &Vec3,
        view_matrix: &Mat4,
        projection_matrix: &Mat4,
        color: Color,
    ) {
        let vp = projection_matrix * view_matrix;
        
        if let (Some(p1), Some(p2)) = (
            self.project_point(start, &vp),
            self.project_point(end, &vp),
        ) {
            self.draw_line(framebuffer, &p1, &p2, color);
        }
    }
        /// Renderiza sin z-test (siempre visible, como overlay)
    pub fn render_mesh_overlay(
        &self,
        framebuffer: &mut Framebuffer,
        mesh: &ObjMesh,
        shader: &dyn PlanetShader,
        model_matrix: &Mat4,
        view_matrix: &Mat4,
        projection_matrix: &Mat4,
        time: f32,
    ) {
        let mvp = projection_matrix * view_matrix * model_matrix;

        let transformed_vertices: Vec<_> = mesh
            .vertices
            .iter()
            .map(|v| self.transform_vertex(v, model_matrix, &mvp))
            .collect();

        for i in (0..mesh.indices.len()).step_by(3) {
            let i0 = mesh.indices[i] as usize;
            let i1 = mesh.indices[i + 1] as usize;
            let i2 = mesh.indices[i + 2] as usize;

            if i0 < transformed_vertices.len()
                && i1 < transformed_vertices.len()
                && i2 < transformed_vertices.len()
            {
                self.rasterize_triangle_overlay(
                    framebuffer,
                    &transformed_vertices[i0],
                    &transformed_vertices[i1],
                    &transformed_vertices[i2],
                    shader,
                    time,
                );
            }
        }
    }

    fn rasterize_triangle_overlay(
        &self,
        framebuffer: &mut Framebuffer,
        v0: &TransformedVertex,
        v1: &TransformedVertex,
        v2: &TransformedVertex,
        shader: &dyn PlanetShader,
        time: f32,
    ) {
        if !Self::is_valid_vertex(v0) 
            || !Self::is_valid_vertex(v1) 
            || !Self::is_valid_vertex(v2) {
            return;
        }

        // Back-face culling
        let edge1 = Vec2::new(
            v1.screen_pos.x - v0.screen_pos.x,
            v1.screen_pos.y - v0.screen_pos.y,
        );
        let edge2 = Vec2::new(
            v2.screen_pos.x - v0.screen_pos.x,
            v2.screen_pos.y - v0.screen_pos.y,
        );
        let cross = edge1.x * edge2.y - edge1.y * edge2.x;
        
        if cross <= 0.0 {
            return;
        }

        // ✅ VALIDAR que los vértices estén dentro del rango visible
        if v0.depth > 1.0 || v1.depth > 1.0 || v2.depth > 1.0 {
            return;
        }

        let min_x = v0.screen_pos.x.min(v1.screen_pos.x).min(v2.screen_pos.x)
            .floor().max(0.0) as usize;
        let max_x = v0.screen_pos.x.max(v1.screen_pos.x).max(v2.screen_pos.x)
            .ceil().min(self.width - 1.0) as usize;
        let min_y = v0.screen_pos.y.min(v1.screen_pos.y).min(v2.screen_pos.y)
            .floor().max(0.0) as usize;
        let max_y = v0.screen_pos.y.max(v1.screen_pos.y).max(v2.screen_pos.y)
            .ceil().min(self.height - 1.0) as usize;

        if min_x >= max_x || min_y >= max_y {
            return;
        }

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let p = Vec2::new(x as f32 + 0.5, y as f32 + 0.5);

                let (w0, w1, w2) = barycentric(
                    &p,
                    &v0.screen_pos,
                    &v1.screen_pos,
                    &v2.screen_pos
                );

                if w0 >= 0.0 && w1 >= 0.0 && w2 >= 0.0 {
                    // ✅ Calcular profundidad interpolada
                    let _depth = w0 * v0.depth + w1 * v1.depth + w2 * v2.depth;
                    
                    // ✅ Solo renderizar si está ADELANTE de lo que hay
                    let index = y * framebuffer.width + x;
                    
                    // Si hay algo muy cerca (depth muy bajo), no sobrescribir
                    if framebuffer.zbuffer[index] < -0.9 {
                        continue;
                    }
                    
                    let world_pos = v0.world_pos * w0 
                        + v1.world_pos * w1 
                        + v2.world_pos * w2;
                    
                    if !world_pos.x.is_finite() 
                        || !world_pos.y.is_finite() 
                        || !world_pos.z.is_finite() {
                        continue;
                    }

                    let world_normal = (v0.world_normal * w0 
                        + v1.world_normal * w1 
                        + v2.world_normal * w2)
                        .normalize();

                    let color = shader.fragment(&world_pos, &world_normal, time);
                    
                    // ✅ Escribir con alpha blending suave
                    let idx = index * 4;
                    let alpha = 0.95; // 95% nave, 5% fondo
                    
                    framebuffer.buffer[idx] = (color.r as f32 * alpha 
                        + framebuffer.buffer[idx] as f32 * (1.0 - alpha)) as u8;
                    framebuffer.buffer[idx + 1] = (color.g as f32 * alpha 
                        + framebuffer.buffer[idx + 1] as f32 * (1.0 - alpha)) as u8;
                    framebuffer.buffer[idx + 2] = (color.b as f32 * alpha 
                        + framebuffer.buffer[idx + 2] as f32 * (1.0 - alpha)) as u8;
                    framebuffer.buffer[idx + 3] = 255;
                    
                    // ✅ NO actualizar z-buffer para permitir que otros objetos se dibujen normalmente
                }
            }
        }
    }
}

struct TransformedVertex {
    screen_pos: Vec2,
    depth: f32,
    world_pos: Vec3,
    world_normal: Vec3,
}

#[inline]
fn barycentric(p: &Vec2, a: &Vec2, b: &Vec2, c: &Vec2) -> (f32, f32, f32) {
    let v0 = *b - *a;
    let v1 = *c - *a;
    let v2 = *p - *a;

    let d00 = v0.dot(&v0);
    let d01 = v0.dot(&v1);
    let d11 = v1.dot(&v1);
    let d20 = v2.dot(&v0);
    let d21 = v2.dot(&v1);

    let denom = d00 * d11 - d01 * d01;

    if denom.abs() < 1e-8 {
        return (0.0, 0.0, 0.0);
    }

    let v = (d11 * d20 - d01 * d21) / denom;
    let w = (d00 * d21 - d01 * d20) / denom;
    let u = 1.0 - v - w;

    (u, v, w)
}