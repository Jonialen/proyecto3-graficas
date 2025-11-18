use nalgebra_glm::Vec3;
use raylib::prelude::*;
use crate::celestial_body::CelestialBody;

pub struct GameUI;

impl GameUI {
    pub fn draw_planet_info(
        d: &mut RaylibDrawHandle,
        body: &CelestialBody,
        distance: f32,
        camera_speed: f32,
    ) {
        let panel_x = 10;
        let panel_y = 200;
        
        d.draw_rectangle(
            panel_x - 5,
            panel_y - 5,
            250,
            120,
            Color::new(0, 0, 0, 180),
        );

        d.draw_text("INFORMACIÓN OBJETIVO", panel_x, panel_y, 16, Color::YELLOW);
        d.draw_text(
            &format!("Nombre: {}", body.name),
            panel_x,
            panel_y + 25,
            14,
            Color::WHITE,
        );
        d.draw_text(
            &format!("Distancia: {:.0} u", distance),
            panel_x,
            panel_y + 45,
            14,
            Color::WHITE,
        );
        d.draw_text(
            &format!("Radio: {:.1} u", body.radius),
            panel_x,
            panel_y + 65,
            14,
            Color::WHITE,
        );

        if camera_speed > 0.1 {
            let eta = distance / camera_speed;
            let eta_text = if eta < 60.0 {
                format!("ETA: {:.0}s", eta)
            } else if eta < 3600.0 {
                format!("ETA: {:.1}min", eta / 60.0)
            } else {
                format!("ETA: {:.1}h", eta / 3600.0)
            };
            d.draw_text(&eta_text, panel_x, panel_y + 85, 14, Color::ORANGE);
        }
    }

    pub fn draw_minimap(
        d: &mut RaylibDrawHandle,
        width: i32,
        height: i32,
        bodies_positions: &[Vec3],
        bodies: &[CelestialBody],
        camera_pos: &Vec3,
    ) {
        let map_size = 200;
        let map_x = width - map_size - 10;
        let map_y = height - map_size - 10;

        d.draw_rectangle(
            map_x - 5,
            map_y - 5,
            map_size + 10,
            map_size + 10,
            Color::new(0, 0, 0, 200),
        );
        d.draw_rectangle_lines(
            map_x - 5,
            map_y - 5,
            map_size + 10,
            map_size + 10,
            Color::SKYBLUE,
        );

        d.draw_text(
            "MAPA SOLAR",
            map_x,
            map_y - 25,
            14,
            Color::SKYBLUE,
        );

        let scale = 250000.0;
        let center_x = map_x + map_size / 2;
        let center_y = map_y + map_size / 2;

        // Sol en el centro
        d.draw_circle(center_x, center_y, 4.0, Color::YELLOW);

        // Planetas
        for (i, pos) in bodies_positions.iter().enumerate() {
            if i == 0 {
                continue; // Skip sol
            }

            let screen_x = center_x + (pos.x / scale * (map_size / 2) as f32) as i32;
            let screen_y = center_y + (pos.z / scale * (map_size / 2) as f32) as i32;

            let color = match bodies[i].body_type {
                crate::celestial_body::CelestialType::Planet => Color::BLUE,
                crate::celestial_body::CelestialType::Moon => Color::GRAY,
                crate::celestial_body::CelestialType::Asteroid => Color::BROWN,
                _ => Color::WHITE,
            };

            if screen_x > map_x
                && screen_x < map_x + map_size
                && screen_y > map_y
                && screen_y < map_y + map_size
            {
                d.draw_circle(screen_x, screen_y, 2.0, color);
            }
        }

        // Nave
        let ship_x = center_x + (camera_pos.x / scale * (map_size / 2) as f32) as i32;
        let ship_y = center_y + (camera_pos.z / scale * (map_size / 2) as f32) as i32;

        if ship_x > map_x
            && ship_x < map_x + map_size
            && ship_y > map_y
            && ship_y < map_y + map_size
        {
            d.draw_circle(ship_x, ship_y, 3.0, Color::GREEN);
        }
    }
}