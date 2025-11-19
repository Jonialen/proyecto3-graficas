use nalgebra_glm::Vec3;
use raylib::prelude::*;
use crate::celestial_body::{CelestialBody, CelestialType};

type RaylibColor = raylib::color::Color;

pub struct Minimap {
    pub size: i32,
    pub zoom_level: f32,
    pub show_orbits: bool,
    pub show_labels: bool,
    pub show_distances: bool,
    padding: i32,
    
    highlight_planet: Option<usize>,
    highlight_pulse: f32,
}

impl Minimap {
    pub fn new(size: i32) -> Self {
        Self {
            size,
            zoom_level: 250000.0,
            show_orbits: true,
            show_labels: true,
            show_distances: false,
            padding: 15,
            highlight_planet: None,
            highlight_pulse: 0.0,
        }
    }

    pub fn auto_zoom(&mut self, bodies_positions: &[Vec3]) {
        // Encontrar el planeta más lejano del Sol
        let mut max_distance = 0.0f32;
        for pos in bodies_positions.iter().skip(1) {  // Skip sol
            let dist = (pos.x.powi(2) + pos.z.powi(2)).sqrt();
            max_distance = max_distance.max(dist);
        }
        
        // Ajustar zoom para que quepa todo con un margen del 20%
        if max_distance > 0.0 {
            self.zoom_level = max_distance * 1.2;
        }
    }

    pub fn adjust_zoom(&mut self, delta: f32) {
        self.zoom_level *= 1.0 + delta * 0.1;
        self.zoom_level = self.zoom_level.clamp(50000.0, 500000.0);
    }

    pub fn render(
        &mut self,
        d: &mut RaylibDrawHandle,
        screen_width: i32,
        screen_height: i32,
        bodies_positions: &[Vec3],
        bodies: &[CelestialBody],
        camera_pos: &Vec3,
        camera_forward: &Vec3,
        time: f32,
    ) {
        self.highlight_pulse += time * 3.0;
        
        let map_x = screen_width - self.size - 10;
        let map_y = screen_height - self.size - 10;
        let center_x = map_x + self.size / 2;
        let center_y = map_y + self.size / 2;

        self.draw_background(d, map_x, map_y);
        self.draw_grid(d, center_x, center_y);
        self.draw_distance_circles(d, center_x, center_y);

        if self.show_orbits {
            self.draw_orbits(d, center_x, center_y, bodies);
        }

        self.draw_sun(d, center_x, center_y);
        self.draw_celestial_bodies(d, center_x, center_y, bodies_positions, bodies, camera_pos);
        self.draw_ship(d, center_x, center_y, camera_pos, camera_forward);

        if self.show_labels {
            self.draw_labels(d, center_x, center_y, bodies_positions, bodies, camera_pos);
        }

        self.draw_frame_and_title(d, map_x, map_y);
        self.draw_info_panel(d, map_x, map_y, camera_pos);
        self.draw_controls_hint(d, map_x, map_y);
    }

    fn draw_background(&self, d: &mut RaylibDrawHandle, x: i32, y: i32) {
        d.draw_rectangle(
            x - 5,
            y - 5,
            self.size + 10,
            self.size + 10,
            RaylibColor::new(5, 5, 20, 220),
        );

        let center_x = x + self.size / 2;
        let center_y = y + self.size / 2;
        
        for i in 0..10 {
            let radius = (self.size as f32 * 0.7 * (10 - i) as f32 / 10.0) as i32;
            let alpha = (10 + i * 5) as u8;
            d.draw_circle(
                center_x,
                center_y,
                radius as f32,
                RaylibColor::new(10, 10, 30, alpha),
            );
        }
    }

    fn draw_grid(&self, d: &mut RaylibDrawHandle, center_x: i32, center_y: i32) {
        let grid_color = RaylibColor::new(30, 30, 50, 80);
        let half_size = self.size / 2;

        for i in -4..=4 {
            let x = center_x + i * half_size / 4;
            d.draw_line(x, center_y - half_size, x, center_y + half_size, grid_color);
        }

        for i in -4..=4 {
            let y = center_y + i * half_size / 4;
            d.draw_line(center_x - half_size, y, center_x + half_size, y, grid_color);
        }
    }

    fn draw_distance_circles(&self, d: &mut RaylibDrawHandle, center_x: i32, center_y: i32) {
        let circle_color = RaylibColor::new(50, 50, 80, 60);
        let distances = [0.25, 0.5, 0.75];

        for &dist in &distances {
            let radius = (self.size as f32 / 2.0 * dist) as i32;
            d.draw_circle_lines(center_x, center_y, radius as f32, circle_color);
        }
    }

    fn draw_orbits(
        &self,
        d: &mut RaylibDrawHandle,
        center_x: i32,
        center_y: i32,
        bodies: &[CelestialBody],
    ) {
        for body in bodies.iter() {
            if body.body_type == CelestialType::Star
                || body.body_type == CelestialType::Asteroid
                || body.body_type == CelestialType::Moon
            {
                continue;
            }

            if let Some(ref params) = body.orbital_params {
                let radius = (params.semi_major_axis / self.zoom_level * (self.size as f32 / 2.0)) as i32;
                
                if radius > 5 && radius < self.size / 2 {
                    let orbit_color = RaylibColor::new(60, 80, 120, 100);
                    d.draw_circle_lines(center_x, center_y, radius as f32, orbit_color);
                }
            }
        }
    }

    fn draw_sun(&self, d: &mut RaylibDrawHandle, center_x: i32, center_y: i32) {
        for i in 0..5 {
            let glow_radius = 8.0 + i as f32 * 2.0;
            let alpha = (100 - i * 20) as u8;
            d.draw_circle(center_x, center_y, glow_radius, RaylibColor::new(255, 200, 0, alpha));
        }

        d.draw_circle(center_x, center_y, 6.0, RaylibColor::new(255, 220, 0, 255));
        d.draw_circle_lines(center_x, center_y, 6.0, RaylibColor::new(255, 255, 100, 255));
    }

    fn draw_celestial_bodies(
        &mut self,
        d: &mut RaylibDrawHandle,
        center_x: i32,
        center_y: i32,
        bodies_positions: &[Vec3],
        bodies: &[CelestialBody],
        camera_pos: &Vec3,
    ) {
        let half_size = self.size / 2;

        for (i, pos) in bodies_positions.iter().enumerate() {
            if i == 0 {
                continue;
            }

            let body = &bodies[i];

            let screen_x = center_x + (pos.x / self.zoom_level * half_size as f32) as i32;
            let screen_y = center_y + (pos.z / self.zoom_level * half_size as f32) as i32;

            if screen_x < center_x - half_size
                || screen_x > center_x + half_size
                || screen_y < center_y - half_size
                || screen_y > center_y + half_size
            {
                continue;
            }

            let (color, size) = self.get_body_appearance(body);
            let dist_to_camera = (pos - camera_pos).magnitude();
            let is_near = dist_to_camera < 10000.0;

            if is_near {
                let pulse = (self.highlight_pulse.sin() * 0.3 + 0.7) as f32;
                d.draw_circle(
                    screen_x,
                    screen_y,
                    size * 2.0 * pulse,
                    RaylibColor::new(255, 255, 0, 100),
                );
            }

            d.draw_circle(screen_x, screen_y, size, color);
            
            let border_color = if is_near {
                RaylibColor::new(255, 255, 100, 200)
            } else {
                RaylibColor::new(color.r / 2, color.g / 2, color.b / 2, 150)
            };
            d.draw_circle_lines(screen_x, screen_y, size, border_color);

            if self.show_distances && is_near {
                let dist_text = if dist_to_camera < 1000.0 {
                    format!("{:.0}u", dist_to_camera)
                } else {
                    format!("{:.1}k", dist_to_camera / 1000.0)
                };
                d.draw_text(
                    &dist_text,
                    screen_x + 8,
                    screen_y - 4,
                    10,
                    RaylibColor::new(200, 200, 255, 200),
                );
            }
        }
    }

    fn get_body_appearance(&self, body: &CelestialBody) -> (RaylibColor, f32) {
        match body.body_type {
            CelestialType::Planet => {
                let size = 4.0;
                let color = match body.name.as_str() {
                    "Mercurio" => RaylibColor::new(180, 150, 120, 255),
                    "Venus" => RaylibColor::new(255, 200, 100, 255),
                    "Tierra" => RaylibColor::new(50, 120, 200, 255),
                    "Marte" => RaylibColor::new(200, 80, 50, 255),
                    "Júpiter" => RaylibColor::new(220, 180, 140, 255),
                    "Saturno" => RaylibColor::new(230, 200, 150, 255),
                    "Urano" => RaylibColor::new(100, 180, 200, 255),
                    "Neptuno" => RaylibColor::new(60, 100, 220, 255),
                    _ => RaylibColor::new(150, 150, 150, 255),
                };
                (color, size)
            }
            CelestialType::Moon => (RaylibColor::new(150, 150, 160, 200), 2.5),
            CelestialType::Asteroid => (RaylibColor::new(120, 100, 90, 150), 1.5),
            _ => (RaylibColor::new(255, 255, 255, 255), 3.0),
        }
    }

    fn draw_ship(
        &self,
        d: &mut RaylibDrawHandle,
        center_x: i32,
        center_y: i32,
        camera_pos: &Vec3,
        camera_forward: &Vec3,
    ) {
        let half_size = self.size / 2;
        let ship_x = center_x + (camera_pos.x / self.zoom_level * half_size as f32) as i32;
        let ship_y = center_y + (camera_pos.z / self.zoom_level * half_size as f32) as i32;

        if ship_x < center_x - half_size
            || ship_x > center_x + half_size
            || ship_y < center_y - half_size
            || ship_y > center_y + half_size
        {
            self.draw_offscreen_indicator(d, center_x, center_y, ship_x, ship_y);
            return;
        }

        d.draw_circle(ship_x, ship_y, 6.0, RaylibColor::new(0, 255, 0, 100));
        d.draw_circle(ship_x, ship_y, 4.0, RaylibColor::new(0, 255, 0, 180));
        d.draw_circle(ship_x, ship_y, 3.0, RaylibColor::new(100, 255, 100, 255));

        let angle = camera_forward.z.atan2(camera_forward.x);
        let arrow_len = 10.0;
        let end_x = ship_x as f32 + angle.cos() * arrow_len;
        let end_y = ship_y as f32 + angle.sin() * arrow_len;

        d.draw_line_ex(
            Vector2::new(ship_x as f32, ship_y as f32),
            Vector2::new(end_x, end_y),
            2.0,
            RaylibColor::new(150, 255, 150, 255),
        );

        let arrow_angle1 = angle + 2.5;
        let arrow_angle2 = angle - 2.5;
        let arrow_size = 5.0;

        d.draw_line_ex(
            Vector2::new(end_x, end_y),
            Vector2::new(end_x + arrow_angle1.cos() * arrow_size, end_y + arrow_angle1.sin() * arrow_size),
            1.5,
            RaylibColor::new(150, 255, 150, 255),
        );
        d.draw_line_ex(
            Vector2::new(end_x, end_y),
            Vector2::new(end_x + arrow_angle2.cos() * arrow_size, end_y + arrow_angle2.sin() * arrow_size),
            1.5,
            RaylibColor::new(150, 255, 150, 255),
        );
    }

    fn draw_offscreen_indicator(
        &self,
        d: &mut RaylibDrawHandle,
        center_x: i32,
        center_y: i32,
        target_x: i32,
        target_y: i32,
    ) {
        let dx = target_x - center_x;
        let dy = target_y - center_y;
        let angle = (dy as f32).atan2(dx as f32);

        let half_size = (self.size / 2 - 15) as f32;
        let indicator_x = center_x as f32 + angle.cos() * half_size;
        let indicator_y = center_y as f32 + angle.sin() * half_size;

        d.draw_circle(indicator_x as i32, indicator_y as i32, 5.0, RaylibColor::new(255, 100, 100, 200));
        
        let arrow_len = 8.0;
        let end_x = indicator_x + angle.cos() * arrow_len;
        let end_y = indicator_y + angle.sin() * arrow_len;

        d.draw_line_ex(
            Vector2::new(indicator_x, indicator_y),
            Vector2::new(end_x, end_y),
            2.0,
            RaylibColor::new(255, 150, 150, 255),
        );
    }

    fn draw_labels(
        &self,
        d: &mut RaylibDrawHandle,
        center_x: i32,
        center_y: i32,
        bodies_positions: &[Vec3],
        bodies: &[CelestialBody],
        camera_pos: &Vec3,
    ) {
        let half_size = self.size / 2;

        for (i, pos) in bodies_positions.iter().enumerate() {
            if i == 0 {
                continue;
            }

            let body = &bodies[i];
            if body.body_type != CelestialType::Planet {
                continue;
            }

            let screen_x = center_x + (pos.x / self.zoom_level * half_size as f32) as i32;
            let screen_y = center_y + (pos.z / self.zoom_level * half_size as f32) as i32;

            if screen_x >= center_x - half_size
                && screen_x <= center_x + half_size
                && screen_y >= center_y - half_size
                && screen_y <= center_y + half_size
            {
                let dist = (pos - camera_pos).magnitude();
                let label = if dist < 1000.0 {
                    body.name.clone()
                } else {
                    body.name.chars().take(3).collect()
                };

                d.draw_text(
                    &label,
                    screen_x + 6,
                    screen_y - 6,
                    10,
                    RaylibColor::new(200, 200, 255, 180),
                );
            }
        }
    }

    fn draw_frame_and_title(&self, d: &mut RaylibDrawHandle, x: i32, y: i32) {
        d.draw_rectangle_lines(
            x - 5,
            y - 5,
            self.size + 10,
            self.size + 10,
            RaylibColor::new(100, 150, 200, 255),
        );

        d.draw_rectangle_lines(
            x - 6,
            y - 6,
            self.size + 12,
            self.size + 12,
            RaylibColor::new(80, 120, 180, 150),
        );

        d.draw_text(
            "MAPA ESTELAR",
            x,
            y - 25,
            14,
            RaylibColor::new(150, 200, 255, 255),
        );
    }

    fn draw_info_panel(&self, d: &mut RaylibDrawHandle, x: i32, y: i32, camera_pos: &Vec3) {
        let info_y = y + self.size + 10;

        d.draw_rectangle(
            x - 5,
            info_y,
            self.size + 10,
            50,
            RaylibColor::new(10, 10, 30, 200),
        );

        d.draw_text(
            &format!("Zoom: {:.0}k", self.zoom_level / 1000.0),
            x + 5,
            info_y + 5,
            12,
            RaylibColor::new(180, 180, 220, 255),
        );

        d.draw_text(
            &format!("Pos: {:.0}, {:.0}", camera_pos.x, camera_pos.z),
            x + 5,
            info_y + 20,
            10,
            RaylibColor::new(150, 150, 200, 255),
        );

        let legend_x = x + self.size / 2;
        d.draw_circle(legend_x, info_y + 12, 3.0, RaylibColor::new(50, 120, 200, 255));
        d.draw_text("Planeta", legend_x + 8, info_y + 8, 10, RaylibColor::new(180, 180, 200, 255));

        d.draw_circle(legend_x, info_y + 28, 2.0, RaylibColor::new(150, 150, 160, 255));
        d.draw_text("Luna", legend_x + 8, info_y + 24, 10, RaylibColor::new(180, 180, 200, 255));
    }

    fn draw_controls_hint(&self, d: &mut RaylibDrawHandle, x: i32, y: i32) {
        let hint_y = y - 40;
        d.draw_text(
            "[ / ] Zoom | L Labels | K Dist",
            x,
            hint_y,
            10,
            RaylibColor::new(150, 150, 180, 200),
        );
    }

    pub fn handle_input(&mut self, rl: &RaylibHandle) {
        if rl.is_key_down(KeyboardKey::KEY_LEFT_BRACKET) {
            self.adjust_zoom(-1.0);
        }
        if rl.is_key_down(KeyboardKey::KEY_RIGHT_BRACKET) {
            self.adjust_zoom(1.0);
        }

        if rl.is_key_pressed(KeyboardKey::KEY_L) {
            self.show_labels = !self.show_labels;
        }

        if rl.is_key_pressed(KeyboardKey::KEY_K) {
            self.show_distances = !self.show_distances;
        }
    }
}