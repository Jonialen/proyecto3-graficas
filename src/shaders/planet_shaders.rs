use crate::framebuffer::Color;
use nalgebra_glm::Vec3;
use super::noise::*;
use super::utils::*;

/// Trait para shaders de planetas
pub trait PlanetShader {
    fn fragment(&self, pos: &Vec3, normal: &Vec3, time: f32) -> Color;
}

// ===================================================================================
// ========== SOL (SHADER MEJORADO) ===================
// ===================================================================================
pub struct ClassicSunShader;

impl PlanetShader for ClassicSunShader {
    fn fragment(&self, pos: &Vec3, normal: &Vec3, time: f32) -> Color {
        let normalized_pos = *normal;

        // Turbulencia multi-capa más compleja
        let turb1 = turbulence(normalized_pos * 2.0 + Vec3::new(time * 0.1, 0.0, 0.0), 4, 0);
        let turb2 = turbulence(normalized_pos * 5.0 + Vec3::new(0.0, time * 0.15, 0.0), 3, 0);
        let turb_combined = turb1 * 0.6 + turb2 * 0.4;

        // Manchas solares realistas
        let spot_noise = cellular_noise(
            normalized_pos.x * 6.0 + time * 0.05,
            normalized_pos.y * 6.0,
            normalized_pos.z * 6.0
        );
        let solar_spots = smoothstep(0.7, 0.85, spot_noise) * 0.4;

        // Temperatura variable con zonas calientes y frías
        let base_temp = 0.75 + turb_combined * 0.2 - solar_spots;
        let temp_color = temperature_to_color(base_temp.clamp(0.0, 1.0));

        // Pulsación sutil
        let pulse = pulse(time, 1.5, 0.92, 1.08);

        // Emisión intensa
        let emission = temp_color * (2.2 + turb_combined * 0.8) * pulse;

        // Corona solar (efecto Fresnel mejorado)
        let view_dir = Vec3::new(0.0, 0.0, 1.0);
        let fresnel_val = fresnel(&view_dir, normal, 2.5);
        let corona_color = Vec3::new(1.0, 0.85, 0.4);
        let corona = corona_color * fresnel_val * 1.2;

        // Protuberancias solares en los bordes
        let prominence = fresnel_val * turb1 * 0.8;
        let prominence_color = Vec3::new(1.0, 0.3, 0.0) * prominence;

        let final_color = emission + corona + prominence_color;
        Color::from_vec3(final_color)
    }
}

// ===================================================================================
// ========== MERCURIO ===================
// ===================================================================================
pub struct MercuryShader;

impl PlanetShader for MercuryShader {
    fn fragment(&self, pos: &Vec3, normal: &Vec3, _time: f32) -> Color {
        let normalized_pos = *normal;

        // Cráteres de impacto multi-escala
        let large_craters = cellular_noise(
            normalized_pos.x * 8.0,
            normalized_pos.y * 8.0,
            normalized_pos.z * 8.0
        );
        let small_craters = cellular_noise(
            normalized_pos.x * 25.0,
            normalized_pos.y * 25.0,
            normalized_pos.z * 25.0
        );
        let crater_pattern = large_craters * 0.7 + small_craters * 0.3;

        // Variación de color basada en composición
        let composition = perlin_noise(
            normalized_pos.x * 3.0,
            normalized_pos.y * 3.0,
            normalized_pos.z * 3.0
        );

        let base_gray = Vec3::new(0.45, 0.42, 0.38);
        let dark_gray = Vec3::new(0.30, 0.28, 0.25);
        let light_gray = Vec3::new(0.55, 0.52, 0.48);

        let surface_color = if crater_pattern > 0.6 {
            mix_vec3(dark_gray, base_gray, (crater_pattern - 0.6) * 2.5)
        } else {
            mix_vec3(base_gray, light_gray, composition * 0.4)
        };

        // Polvo fino (ruido de alta frecuencia)
        let dust = perlin_noise(
            normalized_pos.x * 40.0,
            normalized_pos.y * 40.0,
            normalized_pos.z * 40.0
        ) * 0.1;

        // Iluminación intensa del Sol cercano
        let light_dir = Vec3::new(1.0, 0.4, 0.8).normalize();
        let n_dot_l = normal.dot(&light_dir).max(0.0);
        
        // Terminator más suave
        let diffuse = smoothstep(-0.1, 0.3, n_dot_l) * 0.9 + 0.1;

        let final_color = (surface_color + Vec3::new(dust, dust, dust)) * diffuse * 1.2;
        Color::from_vec3(final_color)
    }
}

// ===================================================================================
// ========== VENUS ===================
// ===================================================================================
pub struct VenusShader;

impl PlanetShader for VenusShader {
    fn fragment(&self, pos: &Vec3, normal: &Vec3, time: f32) -> Color {
        let normalized_pos = *normal;

        // Múltiples capas de nubes a diferentes alturas
        let high_clouds = simplex_noise(
            normalized_pos.x * 3.0 + time * 0.08,
            normalized_pos.y * 3.0,
            normalized_pos.z * 3.0 + time * 0.05
        );
        
        let mid_clouds = simplex_noise(
            normalized_pos.x * 5.0 - time * 0.12,
            normalized_pos.y * 5.0 + time * 0.06,
            normalized_pos.z * 5.0
        );
        
        let low_clouds = turbulence(
            normalized_pos * 7.0 + Vec3::new(time * 0.15, -time * 0.08, 0.0),
            3,
            1
        );

        // Combinación de capas
        let cloud_pattern = high_clouds * 0.4 + mid_clouds * 0.35 + low_clouds * 0.25;

        // Variación de temperatura atmosférica
        let temp_variation = perlin_noise(
            normalized_pos.x * 2.0,
            normalized_pos.y * 2.0 + time * 0.02,
            normalized_pos.z * 2.0
        );

        // Colores atmosféricos
        let base_yellow = Vec3::new(0.95, 0.88, 0.60);
        let bright_yellow = Vec3::new(1.0, 0.92, 0.65);
        let orange_tint = Vec3::new(0.98, 0.80, 0.50);
        
        let color = mix_vec3(
            mix_vec3(base_yellow, bright_yellow, cloud_pattern),
            orange_tint,
            temp_variation * 0.3
        );

        // Iluminación atmosférica suave
        let light_dir = Vec3::new(1.0, 0.3, 1.0).normalize();
        let n_dot_l = normal.dot(&light_dir);
        
        // Subsurface scattering simulado
        let subsurface = smoothstep(-0.3, 0.5, n_dot_l) * 0.6 + 0.4;
        
        // Glow atmosférico en los bordes
        let view_dir = Vec3::new(0.0, 0.0, 1.0);
        let atmosphere_glow = fresnel(&view_dir, normal, 3.0) * 0.3;
        let glow_color = Vec3::new(1.0, 0.85, 0.55);

        let final_color = color * subsurface + glow_color * atmosphere_glow;
        Color::from_vec3(final_color)
    }
}

// ===================================================================================
// ========== TIERRA ===================
// ===================================================================================
pub struct EarthShader;

impl PlanetShader for EarthShader {
    fn fragment(&self, pos: &Vec3, normal: &Vec3, time: f32) -> Color {
        let normalized_pos = *normal;

        // Continentes y océanos con mejor definición
        let continent_noise = turbulence(normalized_pos * 4.0, 4, 0);
        let coastal_detail = perlin_noise(
            normalized_pos.x * 12.0,
            normalized_pos.y * 12.0,
            normalized_pos.z * 12.0
        );

        let is_land = continent_noise > 0.48;
        let coastal_blend = smoothstep(0.45, 0.51, continent_noise + coastal_detail * 0.1);

        // Variación de elevación en tierra
        let elevation = turbulence(normalized_pos * 6.0, 3, 0);
        let latitude = normalized_pos.y;

        let ocean_deep = Vec3::new(0.05, 0.15, 0.35);
        let ocean_shallow = Vec3::new(0.15, 0.35, 0.55);
        let beach = Vec3::new(0.75, 0.70, 0.55);
        let plains = Vec3::new(0.35, 0.55, 0.25);
        let forest = Vec3::new(0.20, 0.45, 0.15);
        let mountain = Vec3::new(0.50, 0.50, 0.48);
        let snow = Vec3::new(0.90, 0.90, 0.95);

        let base_color = if is_land {
            // Biomas terrestres
            if latitude.abs() > 0.65 || elevation > 0.75 {
                mix_vec3(mountain, snow, smoothstep(0.7, 0.8, elevation))
            } else if elevation > 0.6 {
                mountain
            } else if latitude.abs() < 0.3 && elevation < 0.55 {
                forest // Bosques tropicales
            } else {
                plains
            }
        } else {
            // Profundidad oceánica
            mix_vec3(ocean_deep, ocean_shallow, coastal_blend)
        };

        // Costa/playas
        let shore = smoothstep(0.47, 0.49, continent_noise);
        let color_with_shore = mix_vec3(base_color, beach, shore * (1.0 - coastal_blend));

        // Sistema de nubes mejorado
        let cloud_layer1 = turbulence(
            normalized_pos * 8.0 + Vec3::new(time * 0.03, 0.0, time * 0.015),
            4,
            1
        );
        let cloud_layer2 = simplex_noise(
            normalized_pos.x * 15.0 + time * 0.05,
            normalized_pos.y * 15.0,
            normalized_pos.z * 15.0 + time * 0.025
        );
        
        let clouds = smoothstep(0.58, 0.72, cloud_layer1 * 0.7 + cloud_layer2 * 0.3);
        let cloud_color = Vec3::new(1.0, 1.0, 1.0);
        let color_with_clouds = mix_vec3(color_with_shore, cloud_color, clouds * 0.85);

        // Iluminación
        let light_dir = Vec3::new(1.0, 0.4, 0.8).normalize();
        let n_dot_l = normal.dot(&light_dir).max(0.0);
        let diffuse = n_dot_l * 0.75 + 0.25;

        // Especular en océanos
        let view_dir = Vec3::new(0.0, 0.0, 1.0);
        let specular = if !is_land && clouds < 0.3 {
            let half_vec = (light_dir + view_dir).normalize();
            normal.dot(&half_vec).max(0.0).powf(64.0) * 0.6 * (1.0 - clouds)
        } else {
            0.0
        };

        // Atmósfera azul en los bordes
        let atmosphere = fresnel(&view_dir, normal, 3.0);
        let atmosphere_color = Vec3::new(0.3, 0.5, 0.8) * atmosphere * 0.4;

        let final_color = color_with_clouds * diffuse 
            + Vec3::new(1.0, 1.0, 1.0) * specular
            + atmosphere_color;
            
        Color::from_vec3(final_color)
    }
}

// ===================================================================================
// ========== MARTE ===================
// ===================================================================================
pub struct MarsShader;

impl PlanetShader for MarsShader {
    fn fragment(&self, pos: &Vec3, normal: &Vec3, time: f32) -> Color {
        let normalized_pos = *normal;

        // Terreno marciano estratificado
        let large_terrain = turbulence(normalized_pos * 3.0, 4, 0);
        let medium_terrain = turbulence(normalized_pos * 8.0, 3, 0);
        let fine_dust = perlin_noise(
            normalized_pos.x * 30.0,
            normalized_pos.y * 30.0,
            normalized_pos.z * 30.0
        );

        // Colores característicos de Marte
        let rust_red = Vec3::new(0.70, 0.30, 0.20);
        let dark_red = Vec3::new(0.50, 0.22, 0.15);
        let orange_sand = Vec3::new(0.75, 0.45, 0.25);
        let dark_rock = Vec3::new(0.35, 0.25, 0.20);

        // Mezcla de colores basada en el terreno
        let terrain_color = if large_terrain > 0.6 {
            mix_vec3(dark_rock, rust_red, (large_terrain - 0.6) * 2.5)
        } else if large_terrain > 0.4 {
            mix_vec3(rust_red, orange_sand, (large_terrain - 0.4) * 5.0)
        } else {
            mix_vec3(dark_red, rust_red, large_terrain * 2.5)
        };

        // Dunas y arena
        let dune_pattern = (normalized_pos.x * 20.0 + normalized_pos.z * 15.0).sin() * 0.5 + 0.5;
        let sandy_areas = smoothstep(0.45, 0.55, medium_terrain);
        let surface = mix_vec3(terrain_color, orange_sand, sandy_areas * dune_pattern * 0.4);

        // Casquetes polares
        let latitude = normalized_pos.y;
        let polar_ice = smoothstep(0.72, 0.88, latitude.abs());
        let ice_color = Vec3::new(0.92, 0.90, 0.95);
        let color_with_ice = mix_vec3(surface, ice_color, polar_ice * 0.8);

        // Cráteres
        let crater_noise = cellular_noise(
            normalized_pos.x * 12.0,
            normalized_pos.y * 12.0,
            normalized_pos.z * 12.0
        );
        let craters = smoothstep(0.75, 0.85, crater_noise);
        let final_surface = mix_vec3(color_with_ice, color_with_ice * 0.65, craters * 0.3);

        // Tormentas de polvo (opcional, animadas)
        let dust_storm = simplex_noise(
            normalized_pos.x * 5.0 + time * 0.1,
            normalized_pos.y * 5.0,
            normalized_pos.z * 5.0 + time * 0.08
        );
        let storm_opacity = smoothstep(0.7, 0.8, dust_storm) * 0.2;
        let storm_color = Vec3::new(0.85, 0.55, 0.35);
        let color_with_storm = mix_vec3(final_surface, storm_color, storm_opacity);

        // Iluminación
        let light_dir = Vec3::new(1.0, 0.4, 0.8).normalize();
        let n_dot_l = normal.dot(&light_dir).max(0.0);
        let diffuse = n_dot_l * 0.75 + 0.25;

        // Polvo atmosférico añade tinte rojizo
        let dust_scatter = fine_dust * 0.15;
        
        let final_color = (color_with_storm + Vec3::new(dust_scatter, 0.0, 0.0)) * diffuse;
        Color::from_vec3(final_color)
    }
}

// ===================================================================================
// ========== JÚPITER ===================
// ===================================================================================
pub struct JupiterShader;

impl PlanetShader for JupiterShader {
    fn fragment(&self, pos: &Vec3, normal: &Vec3, time: f32) -> Color {
        let normalized_pos = *normal;
        let latitude = normalized_pos.y;
        let longitude = normalized_pos.z.atan2(normalized_pos.x);

        // Bandas atmosféricas realistas
        let band_count = 16.0;
        let band_position = (latitude + 1.0) * 0.5;
        let band_index = (band_position * band_count).floor();
        
        // Colores de las bandas (alternar claros y oscuros)
        let light_band = Vec3::new(0.88, 0.80, 0.70);
        let dark_band = Vec3::new(0.72, 0.62, 0.52);
        let cream_band = Vec3::new(0.92, 0.85, 0.75);
        let brown_band = Vec3::new(0.65, 0.55, 0.45);

        let band_color = match (band_index as i32) % 4 {
            0 => light_band,
            1 => dark_band,
            2 => cream_band,
            _ => brown_band,
        };

        // Transición suave entre bandas
        let band_transition = smoothstep(
            band_index / band_count,
            (band_index + 1.0) / band_count,
            band_position
        );
        let next_band_color = match ((band_index as i32) + 1) % 4 {
            0 => light_band,
            1 => dark_band,
            2 => cream_band,
            _ => brown_band,
        };
        let base_color = mix_vec3(band_color, next_band_color, band_transition);

        // Turbulencia atmosférica compleja
        let turb1 = simplex_noise(
            longitude * 12.0 + time * 0.4,
            latitude * 10.0,
            time * 0.15
        );
        let turb2 = turbulence(
            Vec3::new(longitude * 20.0, latitude * 15.0, time * 0.2),
            3,
            1
        );
        let turbulence_pattern = turb1 * 0.6 + turb2 * 0.4;

        // Vórtices y remolinos
        let vortex = cellular_noise(
            longitude * 8.0 + turbulence_pattern * 2.0,
            latitude * 8.0,
            time * 0.1
        );
        let vortex_color = mix_vec3(base_color, base_color * 1.2, vortex * 0.3);

        // Gran Mancha Roja (más grande y detallada)
        let spot_center = Vec3::new(0.5, -0.18, 0.0).normalize();
        let dist_to_spot = (normalized_pos - spot_center).magnitude();
        
        let spot_radius = 0.12;
        let spot_core = smoothstep(spot_radius, spot_radius * 0.5, dist_to_spot);
        let spot_edge = smoothstep(spot_radius * 1.5, spot_radius, dist_to_spot);
        
        // Rotación interna de la mancha
        let spot_rotation = perlin_noise(
            (normalized_pos - spot_center).x * 15.0 + time * 0.5,
            (normalized_pos - spot_center).y * 15.0,
            (normalized_pos - spot_center).z * 15.0
        );
        
        let spot_dark_red = Vec3::new(0.65, 0.25, 0.18);
        let spot_bright_red = Vec3::new(0.85, 0.35, 0.22);
        let spot_orange = Vec3::new(0.90, 0.50, 0.28);
        
        let spot_color = mix_vec3(
            mix_vec3(spot_dark_red, spot_bright_red, spot_core),
            spot_orange,
            spot_rotation * spot_edge * 0.5
        );
        
        let color_with_spot = mix_vec3(vortex_color, spot_color, spot_edge * 0.9);

        // Pequeñas manchas adicionales
        let small_spots = cellular_noise(
            normalized_pos.x * 25.0,
            normalized_pos.y * 25.0,
            normalized_pos.z * 25.0
        );
        let mini_vortices = smoothstep(0.82, 0.88, small_spots) * 0.15;
        let final_surface = color_with_spot * (1.0 - mini_vortices) 
            + color_with_spot * 0.7 * mini_vortices;

        // Iluminación atmosférica suave
        let light_dir = Vec3::new(1.0, 0.3, 1.0).normalize();
        let n_dot_l = normal.dot(&light_dir);
        let terminator = smoothstep(-0.25, 0.4, n_dot_l);
        
        // Subsurface scattering simulado
        let subsurface = smoothstep(-0.4, 0.2, n_dot_l) * 0.3;
        
        let final_color = final_surface * (0.25 + terminator * 0.75 + subsurface);
        Color::from_vec3(final_color)
    }
}

// ===================================================================================
// ========== SATURNO ===================
// ===================================================================================
pub struct SaturnShader;

impl PlanetShader for SaturnShader {
    fn fragment(&self, pos: &Vec3, normal: &Vec3, time: f32) -> Color {
        let normalized_pos = *normal;
        let latitude = normalized_pos.y;

        // Bandas más sutiles y numerosas que Júpiter
        let band_count = 22.0;
        let band_pattern = ((latitude + 1.0) * 0.5 * band_count).sin() * 0.5 + 0.5;

        let pale_yellow = Vec3::new(0.92, 0.88, 0.72);
        let cream = Vec3::new(0.90, 0.85, 0.70);
        let light_tan = Vec3::new(0.88, 0.82, 0.68);
        
        let base_color = mix_vec3(
            mix_vec3(cream, pale_yellow, band_pattern),
            light_tan,
            band_pattern * 0.5
        );

        // Turbulencia atmosférica muy sutil
        let longitude = normalized_pos.z.atan2(normalized_pos.x);
        let turb = simplex_noise(
            longitude * 8.0 + time * 0.25,
            latitude * 10.0,
            time * 0.1
        );
        
        let mut surface_color = base_color * (0.92 + turb * 0.16);

        // Hexágono polar norte (simulado)
        if latitude > 0.75 {
            let angle = normalized_pos.z.atan2(normalized_pos.x);
            let hexagon = ((angle * 3.0).cos()).abs();
            let hex_intensity = smoothstep(0.76, 0.85, latitude) * hexagon;
            let hex_color = Vec3::new(0.75, 0.70, 0.58);
            surface_color = mix_vec3(surface_color, hex_color, hex_intensity * 0.4);
        }

        // Iluminación suave
        let light_dir = Vec3::new(1.0, 0.3, 1.0).normalize();
        let n_dot_l = normal.dot(&light_dir);
        let diffuse = smoothstep(-0.1, 0.5, n_dot_l) * 0.65 + 0.35;

        Color::from_vec3(surface_color * diffuse)
    }
}

// ===================================================================================
// ========== URANO ===================
// ===================================================================================
pub struct UranusShader;

impl PlanetShader for UranusShader {
    fn fragment(&self, pos: &Vec3, normal: &Vec3, time: f32) -> Color {
        let normalized_pos = *normal;

        // Color cian característico (metano)
        let cyan_bright = Vec3::new(0.65, 0.82, 0.88);
        let cyan_pale = Vec3::new(0.72, 0.85, 0.90);
        let blue_tint = Vec3::new(0.58, 0.78, 0.85);

        // Atmósfera muy uniforme con variación sutil
        let atmosphere_noise = simplex_noise(
            normalized_pos.x * 3.0 + time * 0.05,
            normalized_pos.y * 3.0,
            normalized_pos.z * 3.0
        );
        
        let base_color = mix_vec3(
            mix_vec3(cyan_pale, cyan_bright, atmosphere_noise),
            blue_tint,
            atmosphere_noise * 0.3
        );

        // Bandas muy tenues (rotación extrema)
        let latitude = normalized_pos.x; // Urano está inclinado 98°
        let faint_bands = (latitude * 15.0 + time * 0.2).sin() * 0.5 + 0.5;
        let banded_color = base_color * (0.95 + faint_bands * 0.1);

        // Mancha oscura ocasional
        let dark_spot_center = Vec3::new(0.3, 0.4, 0.0).normalize();
        let dist_to_spot = (normalized_pos - dark_spot_center).magnitude();
        let dark_spot = smoothstep(0.2, 0.1, dist_to_spot);
        let spot_color = Vec3::new(0.45, 0.60, 0.70);
        let color_with_spot = mix_vec3(banded_color, spot_color, dark_spot * 0.5);

        // Iluminación muy suave (lejos del Sol)
        let light_dir = Vec3::new(1.0, 0.3, 1.0).normalize();
        let n_dot_l = normal.dot(&light_dir);
        let diffuse = smoothstep(-0.2, 0.6, n_dot_l) * 0.55 + 0.45;

        // Glow atmosférico en los bordes
        let view_dir = Vec3::new(0.0, 0.0, 1.0);
        let atmosphere_glow = fresnel(&view_dir, normal, 4.0) * 0.25;
        let glow_color = Vec3::new(0.7, 0.9, 1.0);

        let final_color = color_with_spot * diffuse + glow_color * atmosphere_glow;
        Color::from_vec3(final_color)
    }
}

// ===================================================================================
// ========== NEPTUNO ===================
// ===================================================================================
pub struct NeptuneShader;

impl PlanetShader for NeptuneShader {
    fn fragment(&self, pos: &Vec3, normal: &Vec3, time: f32) -> Color {
        let normalized_pos = *normal;
        let latitude = normalized_pos.y;
        let longitude = normalized_pos.z.atan2(normalized_pos.x);

        // Azul profundo característico
        let deep_blue = Vec3::new(0.25, 0.35, 0.75);
        let bright_blue = Vec3::new(0.35, 0.50, 0.85);
        let royal_blue = Vec3::new(0.30, 0.42, 0.80);

        // Tormentas y vórtices complejos
        let storm_large = simplex_noise(
            longitude * 5.0 + time * 0.2,
            latitude * 5.0,
            time * 0.15
        );
        
        let storm_small = turbulence(
            normalized_pos * 12.0 + Vec3::new(time * 0.3, 0.0, time * 0.25),
            4,
            1
        );
        
        let storm_pattern = storm_large * 0.6 + storm_small * 0.4;
        
        let storm_color = mix_vec3(
            mix_vec3(deep_blue, bright_blue, storm_pattern),
            royal_blue,
            storm_pattern * 0.5
        );

        // Gran Mancha Oscura (análoga a la de Júpiter)
        let spot_center = Vec3::new(0.35, 0.25, 0.0).normalize();
        let dist_to_spot = (normalized_pos - spot_center).magnitude();
        
        let spot_outer = smoothstep(0.28, 0.18, dist_to_spot);
        let spot_inner = smoothstep(0.18, 0.08, dist_to_spot);
        
        // Rotación interna de la mancha
        let spot_swirl = perlin_noise(
            (normalized_pos - spot_center).x * 20.0 + time * 0.4,
            (normalized_pos - spot_center).y * 20.0,
            (normalized_pos - spot_center).z * 20.0
        );
        
        let dark_spot_color = Vec3::new(0.15, 0.20, 0.50);
        let spot_edge_color = Vec3::new(0.25, 0.35, 0.70);
        
        let spot = mix_vec3(
            dark_spot_color,
            spot_edge_color,
            (1.0 - spot_inner) * spot_swirl
        );
        
        let color_with_spot = mix_vec3(storm_color, spot, spot_outer * 0.8);

        // Bandas atmosféricas sutiles
        let bands = ((latitude * 12.0 + longitude * 2.0 + time * 0.1).sin() * 0.5 + 0.5) * 0.15;
        let final_surface = color_with_spot * (1.0 - bands) + color_with_spot * 1.2 * bands;

        // Pequeños vórtices adicionales
        let mini_vortex = cellular_noise(
            normalized_pos.x * 18.0 + time * 0.15,
            normalized_pos.y * 18.0,
            normalized_pos.z * 18.0
        );
        let vortex_spots = smoothstep(0.78, 0.85, mini_vortex) * 0.2;
        let atmosphere = mix_vec3(final_surface, bright_blue, vortex_spots);

        // Iluminación (muy lejos del Sol)
        let light_dir = Vec3::new(1.0, 0.3, 1.0).normalize();
        let n_dot_l = normal.dot(&light_dir);
        let diffuse = smoothstep(-0.3, 0.5, n_dot_l) * 0.6 + 0.4;

        // Atmósfera brillante en los bordes
        let view_dir = Vec3::new(0.0, 0.0, 1.0);
        let atmosphere_glow = fresnel(&view_dir, normal, 3.5) * 0.3;
        let glow_color = Vec3::new(0.4, 0.6, 1.0);

        let final_color = atmosphere * diffuse + glow_color * atmosphere_glow;
        Color::from_vec3(final_color)
    }
}

// ===================================================================================
// ========== GENÉRICOS (Luna, Anillos, Nave, Asteroides) ===================
// ===================================================================================

/// Shader para Lunas
pub struct MoonShader;

impl PlanetShader for MoonShader {
    fn fragment(&self, pos: &Vec3, normal: &Vec3, _time: f32) -> Color {
        let normalized_pos = *normal;

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

/// Shader para Anillos Planetarios
/// Shader para Anillos Planetarios
pub struct RingShader;

impl PlanetShader for RingShader {
    fn fragment(&self, pos: &Vec3, normal: &Vec3, time: f32) -> Color {
        // ✅ Ahora pos es la posición real en model space
        let dist_from_center = (pos.x * pos.x + pos.z * pos.z).sqrt();

        // Normalizar al rango del anillo
        let ring_inner = 1.3;
        let ring_outer = 2.0;
        let normalized_dist = (dist_from_center - ring_inner) / (ring_outer - ring_inner);
        
        if normalized_dist < 0.0 || normalized_dist > 1.0 {
            return Color::from_vec3(Vec3::zeros());
        }

        // Bandas bien definidas
        let band_count = 8.0;
        let band = (normalized_dist * band_count).floor();
        let band_fraction = (normalized_dist * band_count).fract();

        // Colores contrastados
        let bright = Vec3::new(0.95, 0.88, 0.70);
        let medium = Vec3::new(0.82, 0.72, 0.58);
        let dark = Vec3::new(0.55, 0.48, 0.40);
        let very_dark = Vec3::new(0.35, 0.30, 0.25);
        
        let base_color = match (band as i32) % 4 {
            0 => bright,
            1 => medium,
            2 => dark,
            _ => very_dark,
        };

        // Gaps (espacios vacíos)
        let is_gap = (band as i32) % 3 == 2;
        if is_gap && band_fraction > 0.3 && band_fraction < 0.7 {
            return Color::from_vec3(Vec3::zeros());
        }

        // Transición suave entre bandas
        let transition = smoothstep(0.0, 0.15, band_fraction) 
            * smoothstep(1.0, 0.85, band_fraction);
        
        let next_band_color = match ((band as i32) + 1) % 4 {
            0 => bright,
            1 => medium,
            2 => dark,
            _ => very_dark,
        };
        
        let blended_color = mix_vec3(base_color, next_band_color, 1.0 - transition);

        // Ruido para textura
        let noise_val = perlin_noise(
            pos.x * 40.0, 
            time * 0.03, 
            pos.z * 40.0
        );
        let color_with_noise = blended_color * (0.9 + noise_val * 0.2);

        // Partículas
        let particle_noise = cellular_noise(
            pos.x * 60.0,
            time * 0.01,
            pos.z * 60.0
        );
        let particles = smoothstep(0.85, 0.92, particle_noise) * 0.3;
        let surface_color = color_with_noise * (1.0 + particles);

        // Iluminación
        let light_dir = Vec3::new(1.0, 0.5, 1.0).normalize();
        let n_dot_l = normal.dot(&light_dir).abs();
        let lit_color = surface_color * (0.7 + n_dot_l * 0.6);

        // Opacidad
        let band_opacity = if is_gap { 0.3 } else { 0.8 };
        let alpha_inner = smoothstep(0.0, 0.1, normalized_dist);
        let alpha_outer = smoothstep(1.0, 0.85, normalized_dist);
        let alpha = alpha_inner * alpha_outer * band_opacity;

        if alpha < 0.15 {
            Color::from_vec3(Vec3::zeros())
        } else {
            Color::from_vec3(lit_color * alpha.max(0.5))
        }
    }
}

/// Shader Metálico para la Nave
pub struct SimpleMetallicShader;

impl PlanetShader for SimpleMetallicShader {
    fn fragment(&self, pos: &Vec3, normal: &Vec3, time: f32) -> Color {
        let normalized_pos = *normal;
        
        // Patrón de paneles
        let panel_noise = perlin_noise(
            normalized_pos.x * 15.0,
            normalized_pos.y * 15.0,
            normalized_pos.z * 15.0,
        );
        
        // Colores metálicos variados
        let base_metal = Vec3::new(0.65, 0.7, 0.75);
        let dark_metal = Vec3::new(0.4, 0.45, 0.5);
        let bright_metal = Vec3::new(0.85, 0.88, 0.9);
        
        let surface_color = if panel_noise > 0.6 {
            mix_vec3(base_metal, bright_metal, (panel_noise - 0.6) * 2.5)
        } else if panel_noise > 0.4 {
            base_metal
        } else {
            mix_vec3(dark_metal, base_metal, panel_noise * 2.5)
        };
        
        // Iluminación direccional
        let light_dir = Vec3::new(1.0, 0.5, 1.0).normalize();
        let n_dot_l = normal.dot(&light_dir).max(0.0);
        let diffuse = n_dot_l * 0.7 + 0.3;
        
        // Especular metálico fuerte
        let view_dir = Vec3::new(0.0, 0.0, 1.0);
        let half_vec = (light_dir + view_dir).normalize();
        let spec_power = normal.dot(&half_vec).max(0.0).powf(64.0);
        let specular = spec_power * 0.8;
        
        // Rim lighting (efecto de borde)
        let rim = fresnel(&view_dir, normal, 3.0);
        let rim_color = Vec3::new(0.3, 0.5, 0.8) * rim * 0.4;
        
        // Luces de navegación pulsantes
        let nav_light_pattern = ((time * 3.0).sin() * 0.5 + 0.5) 
            * smoothstep(0.8, 0.9, normalized_pos.y.abs());
        let nav_light = Vec3::new(0.0, 0.8, 1.0) * nav_light_pattern * 0.3;
        
        let final_color = surface_color * diffuse 
            + Vec3::new(1.0, 1.0, 1.0) * specular 
            + rim_color
            + nav_light;
        
        Color::from_vec3(final_color)
    }
}

/// Shader para Asteroides
pub struct AsteroidShader;

impl PlanetShader for AsteroidShader {
    fn fragment(&self, pos: &Vec3, normal: &Vec3, _time: f32) -> Color {
        let normalized_pos = *normal;

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

// ===================================================================================
// ========== PLANETA ROCOSO GENÉRICO ===================
// ===================================================================================
pub struct RockyPlanet;

impl PlanetShader for RockyPlanet {
    fn fragment(&self, pos: &Vec3, normal: &Vec3, _time: f32) -> Color {
        let normalized_pos = *normal;

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
