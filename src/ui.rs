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

}