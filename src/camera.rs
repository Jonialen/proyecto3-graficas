use nalgebra_glm::{Vec3, Mat4, look_at};
use raylib::prelude::*;


pub struct SpaceshipCamera {
    pub position: Vec3,
    pub target: Vec3,
    pub velocity: Vec3,
    pub forward: Vec3,
    pub right: Vec3,
    pub up: Vec3,
    
    pub acceleration: f32,
    pub max_speed: f32,
    pub drag: f32,
    
    pub yaw: f32,
    pub pitch: f32,
    
    pub third_person: bool,
    pub camera_distance: f32,
    pub camera_height: f32,        // NUEVO
    pub camera_smoothing: f32,     // NUEVO
    
    // Estados suavizados para la cámara
    smoothed_position: Vec3,       // NUEVO
    smoothed_rotation: (f32, f32), // NUEVO (yaw, pitch)

    pub warp_mode: bool,
    pub warp_multiplier: f32,
    pub hyper_warp: bool,
}

impl SpaceshipCamera {
    pub fn new(position: Vec3) -> Self {
        let mut camera = Self {
            position,
            target: Vec3::zeros(),
            velocity: Vec3::zeros(),
            forward: Vec3::new(0.0, 0.0, -1.0),
            right: Vec3::new(1.0, 0.0, 0.0),
            up: Vec3::new(0.0, 1.0, 0.0),
            acceleration: 0.002,
            max_speed: 0.15,
            drag: 0.98,
            yaw: 0.0,
            pitch: 0.0,
            third_person: true,
            camera_distance: 5.0,      // AUMENTADO: era 2.5
            camera_height: 1.5,        // NUEVO: altura sobre la nave
            camera_smoothing: 0.15,    // NUEVO: factor de suavizado
            smoothed_position: position,
            smoothed_rotation: (0.0, 0.0),
            warp_mode: false,
            warp_multiplier: 1.0,
            hyper_warp: false,
        };
        camera.update_vectors();
        camera
    }

    fn update_vectors(&mut self) {
        self.forward = Vec3::new(
            self.yaw.cos() * self.pitch.cos(),
            self.pitch.sin(),
            self.yaw.sin() * self.pitch.cos(),
        ).normalize();

        self.right = self.forward.cross(&Vec3::y()).normalize();
        self.up = self.right.cross(&self.forward).normalize();
        
        self.target = self.position + self.forward;
    }

    pub fn update(&mut self, rl: &RaylibHandle) {
        // ROTACIÓN CON MOUSE (más suave)
        if rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_RIGHT) {
            let mouse_delta = rl.get_mouse_delta();
            let sensitivity = 0.002; // Reducida sensibilidad
            
            self.yaw += mouse_delta.x * sensitivity;
            self.pitch += mouse_delta.y * sensitivity;
            self.pitch = self.pitch.clamp(-1.4, 1.4); // Límite más conservador
        }

        // Toggle Warp Modes
        if rl.is_key_pressed(KeyboardKey::KEY_F) {
            self.warp_mode = !self.warp_mode;
            self.hyper_warp = false;
            self.warp_multiplier = if self.warp_mode { 50.0 } else { 1.0 };
        }

        if rl.is_key_pressed(KeyboardKey::KEY_G) {
            self.hyper_warp = !self.hyper_warp;
            self.warp_mode = false;
            self.warp_multiplier = if self.hyper_warp { 500.0 } else { 1.0 };
        }

        if rl.is_key_pressed(KeyboardKey::KEY_H) {
            let ultra = !self.hyper_warp && !self.warp_mode;
            self.hyper_warp = false;
            self.warp_mode = false;
            self.warp_multiplier = if ultra { 5000.0 } else { 1.0 };
        }

        // Toggle tercera persona
        if rl.is_key_pressed(KeyboardKey::KEY_C) {
            self.third_person = !self.third_person;
        }

        // Ajustar distancia de cámara
        let wheel = rl.get_mouse_wheel_move();
        if wheel != 0.0 {
            self.camera_distance = (self.camera_distance - wheel * 0.5).clamp(2.0, 15.0);
        }
        
        // Ajustar altura de cámara con Page Up/Down
        if rl.is_key_down(KeyboardKey::KEY_PAGE_UP) {
            self.camera_height = (self.camera_height + 0.05).min(5.0);
        }
        if rl.is_key_down(KeyboardKey::KEY_PAGE_DOWN) {
            self.camera_height = (self.camera_height - 0.05).max(-2.0);
        }

        // MOVIMIENTO
        let mut movement = Vec3::zeros();

        if rl.is_key_down(KeyboardKey::KEY_W) {
            movement += self.forward;
        }
        if rl.is_key_down(KeyboardKey::KEY_S) {
            movement -= self.forward;
        }
        if rl.is_key_down(KeyboardKey::KEY_A) {
            movement -= self.right;
        }
        if rl.is_key_down(KeyboardKey::KEY_D) {
            movement += self.right;
        }
        if rl.is_key_down(KeyboardKey::KEY_Q) {
            movement += self.up;
        }
        if rl.is_key_down(KeyboardKey::KEY_E) {
            movement -= self.up;
        }

        let mut speed_multiplier = if rl.is_key_down(KeyboardKey::KEY_LEFT_SHIFT) {
            3.0
        } else {
            1.0
        };

        speed_multiplier *= self.warp_multiplier;

        if movement.magnitude() > 0.0 {
            movement = movement.normalize();
            self.velocity += movement * self.acceleration * speed_multiplier;
        }

        let current_speed = self.velocity.magnitude();
        let max_speed = self.max_speed * speed_multiplier;
        if current_speed > max_speed {
            self.velocity = self.velocity.normalize() * max_speed;
        }

        self.velocity *= self.drag;
        self.position += self.velocity;
        
        // Suavizado de posición para la cámara
        self.smoothed_position = self.smoothed_position 
            + (self.position - self.smoothed_position) * self.camera_smoothing;
        
        // Suavizado de rotación
        let yaw_diff = self.yaw - self.smoothed_rotation.0;
        let pitch_diff = self.pitch - self.smoothed_rotation.1;
        
        self.smoothed_rotation.0 += yaw_diff * self.camera_smoothing;
        self.smoothed_rotation.1 += pitch_diff * self.camera_smoothing;
        
        self.update_vectors();
    }

    pub fn teleport_to(&mut self, target_position: Vec3, offset_distance: f32) {
        let safe_distance = (offset_distance * 3.0).max(100.0);
        
        self.position = target_position + Vec3::new(0.0, safe_distance * 0.3, safe_distance);
        self.velocity = Vec3::zeros();
        
        let direction = (target_position - self.position).normalize();
        self.yaw = direction.z.atan2(direction.x);
        self.pitch = direction.y.asin();
        
        // Sincronizar valores suavizados al teleportarse
        self.smoothed_position = self.position;
        self.smoothed_rotation = (self.yaw, self.pitch);
        
        self.update_vectors();
    }

    pub fn get_view_matrix(&self) -> Mat4 {
        if self.third_person {
            // Usar valores suavizados para la cámara
            let smoothed_forward = Vec3::new(
                self.smoothed_rotation.0.cos() * self.smoothed_rotation.1.cos(),
                self.smoothed_rotation.1.sin(),
                self.smoothed_rotation.0.sin() * self.smoothed_rotation.1.cos(),
            ).normalize();
            
            let smoothed_right = smoothed_forward.cross(&Vec3::y()).normalize();
            let smoothed_up = smoothed_right.cross(&smoothed_forward).normalize();
            
            // Posición de cámara detrás y arriba de la nave
            let camera_offset = -smoothed_forward * self.camera_distance 
                + smoothed_up * self.camera_height;
            
            let camera_pos = self.smoothed_position + camera_offset;
            let look_target = self.smoothed_position + smoothed_forward * 2.0;
            
            look_at(&camera_pos, &look_target, &smoothed_up)
        } else {
            look_at(&self.position, &self.target, &self.up)
        }
    }

    pub fn get_ship_model_matrix(&self, scale: f32) -> Mat4 {
        let mut transform = Mat4::identity();
        
        // Usar posición suavizada
        transform = nalgebra_glm::translate(&transform, &self.smoothed_position);
        
        // Rotación suavizada
        let rotation_y = self.smoothed_rotation.0 + std::f32::consts::PI;
        transform = nalgebra_glm::rotate(&transform, rotation_y, &Vec3::y());
        
        let rotation_x = -self.smoothed_rotation.1;
        transform = nalgebra_glm::rotate(&transform, rotation_x, &Vec3::x());
        
        // Escala
        transform = nalgebra_glm::scale(&transform, &Vec3::new(scale, scale, scale));
        
        transform
    }

    pub fn get_nearest_body_distance(&self, bodies_positions: &[Vec3]) -> Option<(usize, f32)> {
        bodies_positions
            .iter()
            .enumerate()
            .map(|(i, pos)| (i, (pos - self.position).magnitude()))
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
    }

    pub fn get_speed_mode(&self) -> &str {
        if self.warp_multiplier >= 5000.0 {
            "ULTRA WARP"
        } else if self.hyper_warp {
            "HYPER WARP"
        } else if self.warp_mode {
            "WARP"
        } else {
            "IMPULSO"
        }
    }

    pub fn get_effective_speed(&self) -> f32 {
        self.velocity.magnitude()
    }

    pub fn get_camera_position(&self) -> Vec3 {
        if self.third_person {
            let smoothed_forward = Vec3::new(
                self.smoothed_rotation.0.cos() * self.smoothed_rotation.1.cos(),
                self.smoothed_rotation.1.sin(),
                self.smoothed_rotation.0.sin() * self.smoothed_rotation.1.cos(),
            ).normalize();
            
            let smoothed_right = smoothed_forward.cross(&Vec3::y()).normalize();
            let smoothed_up = smoothed_right.cross(&smoothed_forward).normalize();
            
            let camera_offset = -smoothed_forward * self.camera_distance 
                + smoothed_up * self.camera_height;
            
            self.smoothed_position + camera_offset
        } else {
            self.position
        }
    }

    /// Verifica y resuelve colisiones con cuerpos celestes
    pub fn check_collisions(&mut self, bodies: &[(Vec3, f32)]) {
        for (body_pos, body_radius) in bodies {
            let to_body = *body_pos - self.position;
            let distance = to_body.magnitude();
            let safe_distance = body_radius * 2.5; // Margen de seguridad

            if distance < safe_distance {
                // Calcular cuánto nos estamos sobrelapando
                let overlap = safe_distance - distance;
                
                if overlap > 0.0 {
                    // Vector de rechazo (alejar de la superficie)
                    let rejection_dir = -to_body.normalize();
                    
                    // Empujar la posición fuera
                    self.position += rejection_dir * (overlap + 1.0);
                    
                    // Cancelar velocidad hacia el planeta
                    let velocity_toward_body = self.velocity.dot(&to_body.normalize());
                    if velocity_toward_body > 0.0 {
                        // Proyectar velocidad tangente a la superficie
                        let normal = to_body.normalize();
                        self.velocity -= normal * velocity_toward_body * 1.2;
                        
                        // Añadir pequeño rebote
                        self.velocity += rejection_dir * 0.5;
                    }
                    
                    // Actualizar valores suavizados inmediatamente
                    self.smoothed_position = self.position;
                }
            }
        }
    }

    /// Detecta proximidad peligrosa (para warnings)
    pub fn get_collision_warning(&self, bodies: &[(Vec3, f32)]) -> Option<(usize, f32, &str)> {
        for (i, (body_pos, body_radius)) in bodies.iter().enumerate() {
            let distance = (body_pos - self.position).magnitude();
            let warning_distance = body_radius * 4.0;
            
            if distance < warning_distance {
                let severity = if distance < body_radius * 2.5 {
                    "CRÍTICA"
                } else if distance < body_radius * 3.5 {
                    "ALTA"
                } else {
                    "MEDIA"
                };
                
                return Some((i, distance, severity));
            }
        }
        None
    }

    pub fn sync_smoothed_position(&mut self) {
        self.smoothed_position = self.position;
        self.smoothed_rotation = (self.yaw, self.pitch);
    }

    /// Calcula la escala apropiada de la nave según la distancia de cámara
    pub fn get_ship_scale(&self) -> f32 {
        // Escala base fija - la nave no debe cambiar de tamaño
        0.35
    }

    /// Matriz de modelo mejorada para la nave con posición fija relativa
    pub fn get_ship_model_matrix_fixed(&self, base_scale: f32) -> Mat4 {
        let mut transform = Mat4::identity();
        
        // Calcular vectores de dirección suavizados
        let forward = Vec3::new(
            self.smoothed_rotation.0.cos() * self.smoothed_rotation.1.cos(),
            self.smoothed_rotation.1.sin(),
            self.smoothed_rotation.0.sin() * self.smoothed_rotation.1.cos(),
        ).normalize();
        
        let right = forward.cross(&Vec3::y()).normalize();
        let up = right.cross(&forward).normalize();
        
        // Posición de la nave relativa a la posición suavizada
        // Ligeramente adelante y arriba para mejor visibilidad
        let offset_forward = 2.0;
        let offset_up = 0.8;
        let offset_right = 0.5;
        
        let ship_position = self.smoothed_position 
            + forward * offset_forward 
            + up * offset_up
            + right * offset_right;
        
        // Traslación
        transform = nalgebra_glm::translate(&transform, &ship_position);
        
        // Rotación Y (yaw)
        let rotation_y = self.smoothed_rotation.0 + std::f32::consts::PI;
        transform = nalgebra_glm::rotate(&transform, rotation_y, &Vec3::y());
        
        // Rotación X (pitch)
        let rotation_x = -self.smoothed_rotation.1;
        transform = nalgebra_glm::rotate(&transform, rotation_x, &Vec3::x());
        
        // Escala fija
        transform = nalgebra_glm::scale(&transform, &Vec3::new(base_scale, base_scale, base_scale));
        
        transform
    }

     pub fn get_proximity_mode(&self, bodies: &[(Vec3, f32)]) -> ProximityMode {
        for (body_pos, body_radius) in bodies {
            let distance = (body_pos - self.position).magnitude();
            let critical_distance = body_radius * 100.0;
            
            if distance < critical_distance {
                return ProximityMode::Critical;
            } else if distance < critical_distance * 2.0 {
                return ProximityMode::Close;
            }
        }
        ProximityMode::Normal
    }
}

#[derive(Debug, PartialEq)]
pub enum ProximityMode {
    Normal,   // Lejos de todo
    Close,    // Relativamente cerca
    Critical, // Muy cerca de un cuerpo
}