use crate::celestial_body::*;
use nalgebra_glm::Vec3;
use std::f32::consts::PI;

pub struct SolarSystemBuilder;

impl SolarSystemBuilder {
    /// Sistema solar con escala épica (1 unidad ≈ 200,000 km)
    /// Escalas aumentadas 50x para sensación más realista
    pub fn build_realistic() -> Vec<CelestialBody> {
        vec![
            // SOL (índice 0) - ENORME
            CelestialBody {
                name: "Sol".to_string(),
                body_type: CelestialType::Star,
                radius: 350.0,  // Masivo - 50x más grande
                orbital_params: None,
                rotation_period: 25.0,
                rotation_axis: Vec3::y(),
                parent_index: None,
            },

            // MERCURIO (índice 1)
            CelestialBody {
                name: "Mercurio".to_string(),
                body_type: CelestialType::Planet,
                radius: 12.0,  // Más pequeño que antes pero visible
                orbital_params: Some(OrbitalParameters {
                    semi_major_axis: 2895.0,  // 57.9 * 50
                    eccentricity: 0.206,
                    inclination: 7.0_f32.to_radians(),
                    longitude_of_ascending_node: 0.0,
                    argument_of_periapsis: 0.0,
                    orbital_period: 88.0,
                    initial_mean_anomaly: 0.0,
                }),
                rotation_period: 58.6,
                rotation_axis: Vec3::y(),
                parent_index: None,
            },

            // VENUS (índice 2)
            CelestialBody {
                name: "Venus".to_string(),
                body_type: CelestialType::Planet,
                radius: 30.0,  // Planeta del tamaño de la Tierra
                orbital_params: Some(OrbitalParameters {
                    semi_major_axis: 5410.0,  // 108.2 * 50
                    eccentricity: 0.007,
                    inclination: 3.4_f32.to_radians(),
                    longitude_of_ascending_node: 0.0,
                    argument_of_periapsis: 0.0,
                    orbital_period: 224.7,
                    initial_mean_anomaly: PI / 4.0,
                }),
                rotation_period: -243.0,
                rotation_axis: Vec3::y(),
                parent_index: None,
            },

            // TIERRA (índice 3)
            CelestialBody {
                name: "Tierra".to_string(),
                body_type: CelestialType::Planet,
                radius: 32.0,  // Planeta grande
                orbital_params: Some(OrbitalParameters {
                    semi_major_axis: 7480.0,  // 149.6 * 50
                    eccentricity: 0.017,
                    inclination: 0.0,
                    longitude_of_ascending_node: 0.0,
                    argument_of_periapsis: 0.0,
                    orbital_period: 365.25,
                    initial_mean_anomaly: PI / 2.0,
                }),
                rotation_period: 1.0,
                rotation_axis: Vec3::new(0.0, 1.0, 0.01).normalize(),
                parent_index: None,
            },

            // LUNA (índice 4)
            CelestialBody {
                name: "Luna".to_string(),
                body_type: CelestialType::Moon,
                radius: 8.7,  // Luna pequeña pero visible
                orbital_params: Some(OrbitalParameters::circular(192.0, 27.3)),  // 3.84 * 50
                rotation_period: 27.3,
                rotation_axis: Vec3::y(),
                parent_index: Some(3),
            },

            // MARTE (índice 5)
            CelestialBody {
                name: "Marte".to_string(),
                body_type: CelestialType::Planet,
                radius: 17.0,
                orbital_params: Some(OrbitalParameters {
                    semi_major_axis: 11395.0,  // 227.9 * 50
                    eccentricity: 0.093,
                    inclination: 1.85_f32.to_radians(),
                    longitude_of_ascending_node: 0.0,
                    argument_of_periapsis: 0.0,
                    orbital_period: 687.0,
                    initial_mean_anomaly: PI,
                }),
                rotation_period: 1.03,
                rotation_axis: Vec3::y(),
                parent_index: None,
            },

            // JÚPITER (índice 6) - GIGANTE
            CelestialBody {
                name: "Júpiter".to_string(),
                body_type: CelestialType::Planet,
                radius: 350.0,  // Tan grande como el Sol renderizado
                orbital_params: Some(OrbitalParameters {
                    semi_major_axis: 38925.0,  // 778.5 * 50
                    eccentricity: 0.048,
                    inclination: 1.3_f32.to_radians(),
                    longitude_of_ascending_node: 0.0,
                    argument_of_periapsis: 0.0,
                    orbital_period: 4332.6,
                    initial_mean_anomaly: PI * 1.5,
                }),
                rotation_period: 0.4,
                rotation_axis: Vec3::y(),
                parent_index: None,
            },

            // SATURNO (índice 7) - GIGANTE CON ANILLOS
            CelestialBody {
                name: "Saturno".to_string(),
                body_type: CelestialType::Planet,
                radius: 300.0,
                orbital_params: Some(OrbitalParameters {
                    semi_major_axis: 71675.0,  // 1433.5 * 50
                    eccentricity: 0.054,
                    inclination: 2.49_f32.to_radians(),
                    longitude_of_ascending_node: 0.0,
                    argument_of_periapsis: 0.0,
                    orbital_period: 10759.0,
                    initial_mean_anomaly: 0.0,
                }),
                rotation_period: 0.45,
                rotation_axis: Vec3::new(0.0, 1.0, 0.1).normalize(),
                parent_index: None,
            },

            // URANO (índice 8)
            CelestialBody {
                name: "Urano".to_string(),
                body_type: CelestialType::Planet,
                radius: 127.0,
                orbital_params: Some(OrbitalParameters {
                    semi_major_axis: 143625.0,  // 2872.5 * 50
                    eccentricity: 0.047,
                    inclination: 0.77_f32.to_radians(),
                    longitude_of_ascending_node: 0.0,
                    argument_of_periapsis: 0.0,
                    orbital_period: 30688.5,
                    initial_mean_anomaly: PI / 3.0,
                }),
                rotation_period: -0.72,
                rotation_axis: Vec3::new(0.98, 0.0, 0.17).normalize(),
                parent_index: None,
            },

            // NEPTUNO (índice 9)
            CelestialBody {
                name: "Neptuno".to_string(),
                body_type: CelestialType::Planet,
                radius: 123.0,
                orbital_params: Some(OrbitalParameters {
                    semi_major_axis: 224755.0,  // 4495.1 * 50
                    eccentricity: 0.009,
                    inclination: 1.77_f32.to_radians(),
                    longitude_of_ascending_node: 0.0,
                    argument_of_periapsis: 0.0,
                    orbital_period: 60182.0,
                    initial_mean_anomaly: PI / 6.0,
                }),
                rotation_period: 0.67,
                rotation_axis: Vec3::y(),
                parent_index: None,
            },
        ]
    }
}