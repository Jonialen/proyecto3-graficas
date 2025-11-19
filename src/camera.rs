use nalgebra_glm::{Vec3, Mat4, look_at};
use raylib::prelude::*;

/// Representa la cámara principal que sigue a la nave espacial.
///
/// Esta estructura implementa las funciones necesarias para un control de cámara
/// en tercera persona o primera persona, incluyendo movimiento, aceleración,
/// rotación con el ratón, modos de velocidad (warp) y suavizado de movimiento.
pub struct SpaceshipCamera {
    /// Posición actual de la nave en el espacio.
    pub position: Vec3,
    /// Punto hacia el cual la cámara apunta.
    pub target: Vec3,
    /// Velocidad actual de la cámara en cada eje.
    pub velocity: Vec3,
    /// Vector de dirección "forward" normalizado.
    pub forward: Vec3,
    /// Vector lateral (derecha) normalizado.
    pub right: Vec3,
    /// Vector "up" normalizado.
    pub up: Vec3,

    /// Aceleración base aplicada al movimiento lineal.
    pub acceleration: f32,
    /// Velocidad máxima permitida antes de aplicar límite físico.
    pub max_speed: f32,
    /// Coeficiente de arrastre que simula fricción en el espacio.
    pub drag: f32,

    /// Ángulo de rotación horizontal (en radianes).
    pub yaw: f32,
    /// Ángulo de rotación vertical (en radianes).
    pub pitch: f32,

    /// Modo de visualización (true = tercera persona, false = primera).
    pub third_person: bool,
    /// Distancia de la cámara respecto al centro de la nave.
    pub camera_distance: f32,
    /// Altura vertical adicional de la cámara sobre la nave.
    pub camera_height: f32,
    /// Factor de interpolación para suavizar el movimiento de la cámara.
    pub camera_smoothing: f32,

    /// Posición suavizada (interpolada) usada para evitar vibraciones.
    smoothed_position: Vec3,
    /// Rotación suavizada (yaw, pitch) usada en vista de tercera persona.
    smoothed_rotation: (f32, f32),

    /// Indica si el modo warp (salto espacial) está activo.
    pub warp_mode: bool,
    /// Multiplicador de velocidad según el modo warp actual.
    pub warp_multiplier: f32,
    /// Indica si está activo el modo “hyper warp”.
    pub hyper_warp: bool,
}

impl SpaceshipCamera {
    /// Crea una nueva cámara espacial en una posición especificada.
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
            camera_distance: 5.0,
            camera_height: 1.5,
            camera_smoothing: 0.15,
            smoothed_position: position,
            smoothed_rotation: (0.0, 0.0),
            warp_mode: false,
            warp_multiplier: 1.0,
            hyper_warp: false,
        };
        camera.update_vectors();
        camera
    }

    /// Actualiza los vectores de dirección (`forward`, `right`, `up`) a partir
    /// de los ángulos de rotación `yaw` y `pitch`.
    fn update_vectors(&mut self) {
        self.forward = Vec3::new(
            self.yaw.cos() * self.pitch.cos(),
            self.pitch.sin(),
            self.yaw.sin() * self.pitch.cos(),
        )
        .normalize();

        self.right = self.forward.cross(&Vec3::y()).normalize();
        self.up = self.right.cross(&self.forward).normalize();

        self.target = self.position + self.forward;
    }

    /// Actualiza el estado de la cámara según la entrada del usuario y las
    /// físicas básicas de movimiento.
    ///
    /// Procesa rotación con el ratón, ajuste de altura, zoom, control de velocidad
    /// y los modos especiales (warp e hyper warp).
    pub fn update(&mut self, rl: &RaylibHandle) {
        // Rotación con el ratón cuando se mantiene el botón derecho.
        if rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_RIGHT) {
            let mouse_delta = rl.get_mouse_delta();
            let sensitivity = 0.002;
            self.yaw += mouse_delta.x * sensitivity;
            self.pitch += mouse_delta.y * sensitivity;
            self.pitch = self.pitch.clamp(-1.4, 1.4);
        }

        // Alternar entre modos de velocidad (Warp, Hyper Warp, Ultra).
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

        // Alternar entre vistas en primera y tercera persona.
        if rl.is_key_pressed(KeyboardKey::KEY_C) {
            self.third_person = !self.third_person;
        }

        // Ajustar zoom con la rueda del ratón.
        let wheel = rl.get_mouse_wheel_move();
        if wheel != 0.0 {
            self.camera_distance =
                (self.camera_distance - wheel * 0.5).clamp(2.0, 15.0);
        }

        // Ajuste fino de la altura de la cámara.
        if rl.is_key_down(KeyboardKey::KEY_PAGE_UP) {
            self.camera_height = (self.camera_height + 0.05).min(5.0);
        }
        if rl.is_key_down(KeyboardKey::KEY_PAGE_DOWN) {
            self.camera_height = (self.camera_height - 0.05).max(-2.0);
        }

        // Movimiento direccional.
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

        // Aceleración adicional al mantener Shift presionado.
        let mut speed_multiplier =
            if rl.is_key_down(KeyboardKey::KEY_LEFT_SHIFT) {
                3.0
            } else {
                1.0
            };

        speed_multiplier *= self.warp_multiplier;

        if movement.magnitude() > 0.0 {
            movement = movement.normalize();
            self.velocity += movement * self.acceleration * speed_multiplier;
        }

        // Aplicación de límite de velocidad.
        let current_speed = self.velocity.magnitude();
        let max_speed = self.max_speed * speed_multiplier;
        if current_speed > max_speed {
            self.velocity = self.velocity.normalize() * max_speed;
        }

        // Arrastre y actualización de la posición.
        self.velocity *= self.drag;
        self.position += self.velocity;

        // Interpolación para suavizar el movimiento de cámara.
        self.smoothed_position = self.smoothed_position
            + (self.position - self.smoothed_position) * self.camera_smoothing;

        // Suavizado de rotación.
        let yaw_diff = self.yaw - self.smoothed_rotation.0;
        let pitch_diff = self.pitch - self.smoothed_rotation.1;

        self.smoothed_rotation.0 += yaw_diff * self.camera_smoothing;
        self.smoothed_rotation.1 += pitch_diff * self.camera_smoothing;

        self.update_vectors();
    }

    /// Devuelve la matriz de vista (`Mat4`) correspondiente a la posición y orientación actuales.
    pub fn get_view_matrix(&self) -> Mat4 {
        if self.third_person {
            let smoothed_forward = Vec3::new(
                self.smoothed_rotation.0.cos() * self.smoothed_rotation.1.cos(),
                self.smoothed_rotation.1.sin(),
                self.smoothed_rotation.0.sin() * self.smoothed_rotation.1.cos(),
            )
            .normalize();

            let smoothed_right = smoothed_forward.cross(&Vec3::y()).normalize();
            let smoothed_up = smoothed_right.cross(&smoothed_forward).normalize();

            // Cámara colocada detrás y encima de la nave.
            let camera_offset =
                -smoothed_forward * self.camera_distance + smoothed_up * self.camera_height;

            let camera_pos = self.smoothed_position + camera_offset;
            let look_target = self.smoothed_position + smoothed_forward * 2.0;

            look_at(&camera_pos, &look_target, &smoothed_up)
        } else {
            look_at(&self.position, &self.target, &self.up)
        }
    }

    /// Calcula el cuerpo celeste más cercano a la cámara.
    pub fn get_nearest_body_distance(
        &self,
        bodies_positions: &[Vec3],
    ) -> Option<(usize, f32)> {
        bodies_positions
            .iter()
            .enumerate()
            .map(|(i, pos)| (i, (pos - self.position).magnitude()))
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
    }

    /// Devuelve el nombre del modo de velocidad actual.
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

    /// Retorna la velocidad efectiva (magnitud del vector de velocidad).
    pub fn get_effective_speed(&self) -> f32 {
        self.velocity.magnitude()
    }

    /// Retorna la posición actual de la cámara, dependiendo del modo de vista.
    pub fn get_camera_position(&self) -> Vec3 {
        if self.third_person {
            let smoothed_forward = Vec3::new(
                self.smoothed_rotation.0.cos() * self.smoothed_rotation.1.cos(),
                self.smoothed_rotation.1.sin(),
                self.smoothed_rotation.0.sin() * self.smoothed_rotation.1.cos(),
            )
            .normalize();

            let smoothed_right = smoothed_forward.cross(&Vec3::y()).normalize();
            let smoothed_up = smoothed_right.cross(&smoothed_forward).normalize();

            let camera_offset =
                -smoothed_forward * self.camera_distance + smoothed_up * self.camera_height;

            self.smoothed_position + camera_offset
        } else {
            self.position
        }
    }

    /// Detecta y resuelve colisiones físicas simples contra cuerpos celestes.
    ///
    /// Si se detecta una intersección, la cámara es empujada hacia fuera del planeta
    /// y se ajusta la velocidad para evitar penetraciones.
    pub fn check_collisions(&mut self, bodies: &[(Vec3, f32)]) {
        for (body_pos, body_radius) in bodies {
            let to_body = *body_pos - self.position;
            let distance = to_body.magnitude();
            let safe_distance = body_radius * 2.5;

            if distance < safe_distance {
                let overlap = safe_distance - distance;
                if overlap > 0.0 {
                    let rejection_dir = -to_body.normalize();
                    self.position += rejection_dir * (overlap + 1.0);

                    let velocity_toward_body = self.velocity.dot(&to_body.normalize());
                    if velocity_toward_body > 0.0 {
                        let normal = to_body.normalize();
                        self.velocity -= normal * velocity_toward_body * 1.2;
                        self.velocity += rejection_dir * 0.5;
                    }

                    self.smoothed_position = self.position;
                }
            }
        }
    }

    /// Devuelve información sobre una colisión potencial inminente para propósitos de alerta.
    pub fn get_collision_warning(
        &self,
        bodies: &[(Vec3, f32)],
    ) -> Option<(usize, f32, &str)> {
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

    /// Sincroniza los valores suavizados con la posición y rotación actual.
    pub fn sync_smoothed_position(&mut self) {
        self.smoothed_position = self.position;
        self.smoothed_rotation = (self.yaw, self.pitch);
    }

    /// Genera una matriz modelo para posicionar la nave en el espacio de cámara
    /// durante el renderizado en tercera persona.
    pub fn get_ship_model_matrix_fixed(&self, base_scale: f32) -> Mat4 {
        let mut transform = Mat4::identity();

        let forward = Vec3::new(
            self.smoothed_rotation.0.cos() * self.smoothed_rotation.1.cos(),
            self.smoothed_rotation.1.sin(),
            self.smoothed_rotation.0.sin() * self.smoothed_rotation.1.cos(),
        )
        .normalize();

        let right = forward.cross(&Vec3::y()).normalize();
        let up = right.cross(&forward).normalize();

        // Posición relativa ajustada para visibilidad de la nave en cámara.
        let offset_forward = 2.5;
        let offset_up = 0.5;
        let offset_right = 0.0;

        let ship_position = self.smoothed_position
            + forward * offset_forward
            + up * offset_up
            + right * offset_right;

        transform = nalgebra_glm::translate(&transform, &ship_position);

        // Rotaciones.
        let rotation_y = self.smoothed_rotation.0 + std::f32::consts::PI;
        transform = nalgebra_glm::rotate(&transform, rotation_y, &Vec3::y());

        let rotation_x = -self.smoothed_rotation.1;
        transform = nalgebra_glm::rotate(&transform, rotation_x, &Vec3::x());

        // Escalado uniforme.
        transform =
            nalgebra_glm::scale(&transform, &Vec3::new(base_scale, base_scale, base_scale));

        transform
    }
}