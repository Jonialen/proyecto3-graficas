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
            camera_distance: 2.5,
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
        // ROTACIÓN CON MOUSE
        if rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_RIGHT) {
            let mouse_delta = rl.get_mouse_delta();
            self.yaw += mouse_delta.x * 0.003;
            self.pitch += mouse_delta.y * 0.003;
            self.pitch = self.pitch.clamp(-1.5, 1.5);
        }

        // Toggle Warp Mode (F) - x50 (era x10)
        if rl.is_key_pressed(KeyboardKey::KEY_F) {
            self.warp_mode = !self.warp_mode;
            self.hyper_warp = false;
            self.warp_multiplier = if self.warp_mode { 50.0 } else { 1.0 };
        }

        // Toggle Hyper Warp Mode (G) - x500 (era x100)
        if rl.is_key_pressed(KeyboardKey::KEY_G) {
            self.hyper_warp = !self.hyper_warp;
            self.warp_mode = false;
            self.warp_multiplier = if self.hyper_warp { 500.0 } else { 1.0 };
        }

        // Ultra Warp (H) - x5000 (era x1000)
        if rl.is_key_pressed(KeyboardKey::KEY_H) {
            let ultra = !self.hyper_warp && !self.warp_mode;
            self.hyper_warp = false;
            self.warp_mode = false;
            self.warp_multiplier = if ultra { 5000.0 } else { 1.0 };
        }

        // Toggle tercera persona con C
        if rl.is_key_pressed(KeyboardKey::KEY_C) {
            self.third_person = !self.third_person;
        }

        // Ajustar distancia de cámara con scroll
        let wheel = rl.get_mouse_wheel_move();
        if wheel != 0.0 {
            self.camera_distance = (self.camera_distance - wheel * 0.2).clamp(1.0, 6.0);
        }

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
            3.0  // SHIFT ahora da x3 (era x2)
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
        self.update_vectors();
    }

    pub fn teleport_to(&mut self, target_position: Vec3, offset_distance: f32) {
        // Distancia segura aumentada proporcionalmente
        let safe_distance = (offset_distance * 3.0).max(100.0);
        
        self.position = target_position + Vec3::new(0.0, safe_distance * 0.3, safe_distance);
        self.velocity = Vec3::zeros();
        
        let direction = (target_position - self.position).normalize();
        self.yaw = direction.z.atan2(direction.x);
        self.pitch = direction.y.asin();
        
        self.update_vectors();
    }

    pub fn get_view_matrix(&self) -> Mat4 {
        if self.third_person {
            let camera_pos = self.position - self.forward * self.camera_distance + self.up * 0.5;
            look_at(&camera_pos, &self.position, &self.up)
        } else {
            look_at(&self.position, &self.target, &self.up)
        }
    }

    pub fn get_ship_model_matrix(&self, scale: f32) -> Mat4 {
        let mut transform = Mat4::identity();
        
        transform = nalgebra_glm::translate(&transform, &self.position);
        
        let rotation_y = self.yaw + std::f32::consts::PI;
        transform = nalgebra_glm::rotate(&transform, rotation_y, &Vec3::y());
        
        let rotation_x = -self.pitch;
        transform = nalgebra_glm::rotate(&transform, rotation_x, &Vec3::x());
        
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
            self.position - self.forward * self.camera_distance + self.up * 0.5
        } else {
            self.position
        }
    }
}