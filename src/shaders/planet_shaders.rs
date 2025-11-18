use crate::framebuffer::Color;
use nalgebra_glm::Vec3;
use super::noise::*;
use super::utils::*;

/// Trait para shaders de planetas
pub trait PlanetShader {
    fn fragment(&self, pos: &Vec3, normal: &Vec3, time: f32) -> Color;
}

// =================== MERCURIO ===================
pub struct MercuryShader;

impl PlanetShader for MercuryShader {
    fn fragment(&self, pos: &Vec3, normal: &Vec3, _time: f32) -> Color {
        let normalized_pos = pos.normalize();

        // Superficie crateada y árida
        let crater_noise = turbulence(normalized_pos * 12.0, 4, 0);
        let crater_factor = smoothstep(0.65, 0.75, crater_noise);

        let base_color = Vec3::new(0.5, 0.45, 0.4); // Gris rocoso
        let crater_color = Vec3::new(0.35, 0.3, 0.28);
        let surface_color = mix_vec3(base_color, crater_color, crater_factor * 0.6);

        // Iluminación intensa (cerca del sol)
        let light_dir = Vec3::new(1.0, 0.5, 1.0).normalize();
        let diffuse = normal.dot(&light_dir).max(0.0) * 0.8 + 0.2;

        Color::from_vec3(surface_color * diffuse * 1.1)
    }
}

// =================== VENUS ===================
pub struct VenusShader;

impl PlanetShader for VenusShader {
    fn fragment(&self, pos: &Vec3, normal: &Vec3, time: f32) -> Color {
        let normalized_pos = pos.normalize();

        // Atmósfera densa con nubes de ácido sulfúrico
        let cloud_layer1 = turbulence(normalized_pos * 4.0 + Vec3::new(time * 0.05, 0.0, 0.0), 3, 1);
        let cloud_layer2 = turbulence(normalized_pos * 6.0 - Vec3::new(time * 0.08, time * 0.03, 0.0), 4, 1);

        let cloud_pattern = (cloud_layer1 + cloud_layer2) * 0.5;

        // Colores amarillentos/anaranjados característicos de Venus
        let base_color = Vec3::new(0.9, 0.85, 0.6);
        let cloud_color = Vec3::new(0.95, 0.8, 0.5);
        let atmosphere_color = mix_vec3(base_color, cloud_color, cloud_pattern);

        // Iluminación suave por la atmósfera densa
        let light_dir = Vec3::new(1.0, 0.3, 1.0).normalize();
        let diffuse = normal.dot(&light_dir).abs() * 0.5 + 0.5;

        Color::from_vec3(atmosphere_color * diffuse)
    }
}

// =================== TIERRA ===================
pub struct EarthShader;

impl PlanetShader for EarthShader {
    fn fragment(&self, pos: &Vec3, normal: &Vec3, time: f32) -> Color {
        let normalized_pos = pos.normalize();

        // Continentes y océanos
        let height = normalized_pos.y;
        let continent_noise = turbulence(normalized_pos * 3.0, 3, 0);

        let base_color = if continent_noise > 0.5 {
            // Tierra
            if height > 0.5 {
                Vec3::new(0.7, 0.65, 0.5) // Montañas
            } else if height > 0.2 {
                Vec3::new(0.3, 0.6, 0.2) // Bosques
            } else {
                Vec3::new(0.6, 0.7, 0.4) // Planicies
            }
        } else {
            // Océano
            Vec3::new(0.1, 0.3, 0.6)
        };

        // Nubes animadas
        let cloud_pattern = turbulence(
            normalized_pos * 8.0 + Vec3::new(time * 0.02, 0.0, time * 0.01),
            3,
            1,
        );
        let clouds = smoothstep(0.6, 0.7, cloud_pattern);
        let cloud_color = Vec3::new(1.0, 1.0, 1.0);

        let color_with_clouds = mix_vec3(base_color, cloud_color, clouds * 0.8);

        // Iluminación
        let light_dir = Vec3::new(1.0, 0.5, 1.0).normalize();
        let diffuse = normal.dot(&light_dir).max(0.0) * 0.7 + 0.3;

        // Especular en océanos
        let specular = if continent_noise <= 0.5 {
            let view_dir = Vec3::new(0.0, 0.0, 1.0);
            let half_vec = (light_dir + view_dir).normalize();
            normal.dot(&half_vec).max(0.0).powf(32.0) * 0.4
        } else {
            0.0
        };

        let final_color = color_with_clouds * diffuse + Vec3::new(1.0, 1.0, 1.0) * specular;
        Color::from_vec3(final_color)
    }
}

// =================== MARTE ===================
pub struct MarsShader;

impl PlanetShader for MarsShader {
    fn fragment(&self, pos: &Vec3, normal: &Vec3, _time: f32) -> Color {
        let normalized_pos = pos.normalize();

        // Superficie rojiza característica
        let terrain_noise = turbulence(normalized_pos * 5.0, 4, 0);
        
        let base_red = Vec3::new(0.7, 0.3, 0.2);
        let dark_red = Vec3::new(0.5, 0.2, 0.15);
        let base_color = mix_vec3(base_red, dark_red, terrain_noise * 0.5);

        // Casquetes polares
        let polar_ice = smoothstep(0.7, 0.85, normalized_pos.y.abs());
        let ice_color = Vec3::new(0.9, 0.9, 0.95);
        let surface_color = mix_vec3(base_color, ice_color, polar_ice * 0.7);

        // Cráteres
        let crater_noise = turbulence(normalized_pos * 15.0, 3, 0);
        let crater_factor = smoothstep(0.7, 0.8, crater_noise);
        let cratered_color = mix_vec3(surface_color, surface_color * 0.6, crater_factor * 0.4);

        // Iluminación
        let light_dir = Vec3::new(1.0, 0.5, 1.0).normalize();
        let diffuse = normal.dot(&light_dir).max(0.0) * 0.7 + 0.3;

        Color::from_vec3(cratered_color * diffuse)
    }
}

// =================== JÚPITER ===================
pub struct JupiterShader;

impl PlanetShader for JupiterShader {
    fn fragment(&self, pos: &Vec3, normal: &Vec3, time: f32) -> Color {
        let normalized_pos = pos.normalize();

        // Bandas atmosféricas
        let latitude = normalized_pos.y;
        let band_count = 14.0;
        let band = ((latitude + 1.0) * 0.5 * band_count).floor();
        
        let band_colors = [
            Vec3::new(0.8, 0.7, 0.6),
            Vec3::new(0.9, 0.8, 0.7),
            Vec3::new(0.7, 0.6, 0.5),
            Vec3::new(0.85, 0.75, 0.65),
        ];
        let base_color = band_colors[band as usize % band_colors.len()];

        // Turbulencia atmosférica
        let longitude = normalized_pos.z.atan2(normalized_pos.x);
        let turb = simplex_noise(
            longitude * 10.0 + time * 0.3,
            latitude * 8.0,
            time * 0.1,
        );
        let turbulent_color = mix_vec3(base_color, base_color * 1.15, turb * 0.3);

        // Gran Mancha Roja
        let spot_center = Vec3::new(0.5, -0.15, 0.0).normalize();
        let dist_to_spot = (normalized_pos - spot_center).magnitude();
        let spot_factor = smoothstep(0.3, 0.15, dist_to_spot);
        let spot_color = Vec3::new(0.8, 0.3, 0.2);
        let color_with_spot = mix_vec3(turbulent_color, spot_color, spot_factor * 0.8);

        // Iluminación suave
        let light_dir = Vec3::new(1.0, 0.3, 1.0).normalize();
        let terminator = smoothstep(-0.2, 0.3, normal.dot(&light_dir));
        let final_color = color_with_spot * (0.3 + terminator * 0.7);

        Color::from_vec3(final_color)
    }
}

// =================== SATURNO ===================
pub struct SaturnShader;

impl PlanetShader for SaturnShader {
    fn fragment(&self, pos: &Vec3, normal: &Vec3, time: f32) -> Color {
        let normalized_pos = pos.normalize();

        // Bandas más suaves que Júpiter
        let latitude = normalized_pos.y;
        let band_pattern = (latitude * 10.0 + time * 0.1).sin() * 0.5 + 0.5;

        let light_band = Vec3::new(0.9, 0.85, 0.7);
        let dark_band = Vec3::new(0.85, 0.8, 0.65);
        let base_color = mix_vec3(dark_band, light_band, band_pattern);

        // Turbulencia sutil
        let turb = simplex_noise(
            normalized_pos.x * 6.0 + time * 0.2,
            normalized_pos.y * 6.0,
            normalized_pos.z * 6.0,
        );
        let surface_color = base_color * (0.9 + turb * 0.2);

        // Iluminación
        let light_dir = Vec3::new(1.0, 0.3, 1.0).normalize();
        let diffuse = normal.dot(&light_dir).abs() * 0.6 + 0.4;

        Color::from_vec3(surface_color * diffuse)
    }
}

// =================== URANO ===================
pub struct UranusShader;

impl PlanetShader for UranusShader {
    fn fragment(&self, pos: &Vec3, normal: &Vec3, _time: f32) -> Color {
        let normalized_pos = pos.normalize();

        // Color cian característico (metano en la atmósfera)
        let base_color = Vec3::new(0.6, 0.8, 0.85);

        // Variación atmosférica sutil
        let atmosphere_noise = turbulence(normalized_pos * 4.0, 3, 1);
        let varied_color = base_color * (0.9 + atmosphere_noise * 0.2);

        // Iluminación suave
        let light_dir = Vec3::new(1.0, 0.3, 1.0).normalize();
        let diffuse = normal.dot(&light_dir).abs() * 0.5 + 0.5;

        Color::from_vec3(varied_color * diffuse)
    }
}

// =================== NEPTUNO ===================
pub struct NeptuneShader;

impl PlanetShader for NeptuneShader {
    fn fragment(&self, pos: &Vec3, normal: &Vec3, time: f32) -> Color {
        let normalized_pos = pos.normalize();

        // Azul profundo característico
        let base_color = Vec3::new(0.3, 0.4, 0.8);

        // Tormentas y vórtices
        let storm_pattern = simplex_noise(
            normalized_pos.x * 8.0 + time * 0.15,
            normalized_pos.y * 8.0,
            normalized_pos.z * 8.0 + time * 0.1,
        );
        
        let storm_color = Vec3::new(0.4, 0.5, 0.9);
        let atmosphere_color = mix_vec3(base_color, storm_color, storm_pattern * 0.4);

        // Gran Mancha Oscura
        let spot_center = Vec3::new(0.3, 0.2, 0.0).normalize();
        let dist_to_spot = (normalized_pos - spot_center).magnitude();
        let spot_factor = smoothstep(0.25, 0.15, dist_to_spot);
        let dark_spot = Vec3::new(0.2, 0.25, 0.5);
        let color_with_spot = mix_vec3(atmosphere_color, dark_spot, spot_factor * 0.6);

        // Iluminación
        let light_dir = Vec3::new(1.0, 0.3, 1.0).normalize();
        let diffuse = normal.dot(&light_dir).abs() * 0.6 + 0.4;

        Color::from_vec3(color_with_spot * diffuse)
    }
}

// =================== GENÉRICOS ===================

/// Planeta rocoso genérico (ya lo tenías)
pub struct RockyPlanet;

impl PlanetShader for RockyPlanet {
    fn fragment(&self, pos: &Vec3, normal: &Vec3, _time: f32) -> Color {
        let normalized_pos = pos.normalize();

        let height = normalized_pos.y;
        let base_color = if height > 0.4 {
            Vec3::new(0.7, 0.5, 0.3)
        } else if height > 0.0 {
            Vec3::new(0.4, 0.6, 0.3)
        } else if height > -0.3 {
            Vec3::new(0.8, 0.7, 0.5)
        } else {
            Vec3::new(0.1, 0.3, 0.6)
        };

        let continent_noise = turbulence(normalized_pos * 3.0, 3, 0);
        let color_variation = mix_vec3(base_color, base_color * 0.8, continent_noise * 0.3);

        let light_dir = Vec3::new(1.0, 0.5, 1.0).normalize();
        let diffuse = normal.dot(&light_dir).abs() * 0.6 + 0.4;

        Color::from_vec3(color_variation * diffuse)
    }
}

/// Shader para lunas
pub struct MoonShader;

impl PlanetShader for MoonShader {
    fn fragment(&self, pos: &Vec3, normal: &Vec3, _time: f32) -> Color {
        let normalized_pos = pos.normalize();

        let crater_noise = turbulence(normalized_pos * 8.0, 3, 0);
        let crater = smoothstep(0.6, 0.8, crater_noise);
        
        let base_color = Vec3::new(0.4, 0.4, 0.45);
        let crater_color = Vec3::new(0.25, 0.25, 0.28);
        let surface_color = mix_vec3(base_color, crater_color, crater * 0.6);

        let detail = perlin_noise(
            normalized_pos.x * 30.0,
            normalized_pos.y * 30.0,
            normalized_pos.z * 30.0,
        );
        let detailed_color = surface_color * (0.9 + detail * 0.2);

        let light_dir = Vec3::new(1.0, 0.5, 1.0).normalize();
        let diffuse = normal.dot(&light_dir).abs() * 0.7 + 0.3;

        Color::from_vec3(detailed_color * diffuse)
    }
}

/// Shader para anillos planetarios
pub struct RingShader;

impl PlanetShader for RingShader {
    fn fragment(&self, pos: &Vec3, normal: &Vec3, time: f32) -> Color {
        let dist_from_center = (pos.x * pos.x + pos.z * pos.z).sqrt();

        let band_count = 15.0;
        let band = (dist_from_center * band_count).floor();

        let color1 = Vec3::new(0.8, 0.7, 0.6);
        let color2 = Vec3::new(0.6, 0.5, 0.4);
        let base_color = if band as i32 % 2 == 0 {
            color1
        } else {
            color2
        };

        let noise_val = perlin_noise(pos.x * 20.0, time * 0.1, pos.z * 20.0);
        let color_with_noise = base_color * (0.8 + noise_val * 0.4);

        let light_dir = Vec3::new(1.0, 0.5, 1.0).normalize();
        let n_dot_l = normal.dot(&light_dir).abs();
        let lit_color = color_with_noise * (0.5 + n_dot_l * 0.5);

        let alpha_inner = smoothstep(0.0, 0.05, dist_from_center - 1.3);
        let alpha_outer = smoothstep(2.2, 2.0, dist_from_center);
        let alpha = alpha_inner * alpha_outer;

        if alpha < 0.3 {
            Color::new(5, 5, 15) // Color del fondo
        } else {
            Color::from_vec3(lit_color * alpha)
        }
    }
}

/// Shader metálico simple para la nave
pub struct SimpleMetallicShader;

impl PlanetShader for SimpleMetallicShader {
    fn fragment(&self, _pos: &Vec3, normal: &Vec3, _time: f32) -> Color {
        let base_color = Vec3::new(0.7, 0.75, 0.8); // Color gris metálico
        
        let light_dir = Vec3::new(1.0, 1.0, 1.0).normalize();
        let diffuse = normal.dot(&light_dir).abs() * 0.6 + 0.4;
        
        // Especular
        let view_dir = Vec3::new(0.0, 0.0, 1.0);
        let half_vec = (light_dir + view_dir).normalize();
        let specular = normal.dot(&half_vec).max(0.0).powf(32.0) * 0.5;
        
        let final_color = base_color * diffuse + Vec3::new(1.0, 1.0, 1.0) * specular;
        Color::from_vec3(final_color)
    }
}

// =================== ASTEROIDES ===================
pub struct AsteroidShader;

impl PlanetShader for AsteroidShader {
    fn fragment(&self, pos: &Vec3, normal: &Vec3, _time: f32) -> Color {
        let normalized_pos = pos.normalize();

        // Superficie extremadamente rugosa y crateada
        let rough_noise = turbulence(normalized_pos * 20.0, 4, 0);
        let crater_detail = turbulence(normalized_pos * 50.0, 3, 2);
        
        // Colores rocosos variados (gris, marrón oscuro)
        let base_gray = Vec3::new(0.35, 0.33, 0.30);
        let dark_brown = Vec3::new(0.25, 0.22, 0.20);
        let rust = Vec3::new(0.45, 0.35, 0.30);
        
        // Mezclar colores según el ruido
        let color_mix = if rough_noise > 0.6 {
            mix_vec3(base_gray, rust, (rough_noise - 0.6) * 2.5)
        } else if rough_noise > 0.4 {
            mix_vec3(dark_brown, base_gray, (rough_noise - 0.4) * 5.0)
        } else {
            mix_vec3(dark_brown * 0.8, dark_brown, rough_noise * 2.5)
        };

        // Añadir detalles de cráteres
        let surface_color = color_mix * (0.7 + crater_detail * 0.6);

        // Iluminación muy contrastada (sin atmósfera)
        let light_dir = Vec3::new(1.0, 0.5, 1.0).normalize();
        let n_dot_l = normal.dot(&light_dir).max(0.0);
        
        // Lambert + ambient muy bajo (espacio oscuro)
        let diffuse = n_dot_l * 0.9 + 0.1;
        
        // Pequeño especular metálico (minerales)
        let view_dir = Vec3::new(0.0, 0.0, 1.0);
        let half_vec = (light_dir + view_dir).normalize();
        let specular = normal.dot(&half_vec).max(0.0).powf(64.0) * 0.15;
        
        let final_color = surface_color * diffuse 
            + Vec3::new(0.5, 0.5, 0.5) * specular;
        
        Color::from_vec3(final_color)
    }
}