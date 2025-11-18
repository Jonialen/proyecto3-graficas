use crate::celestial_body::*;
use nalgebra_glm::Vec3;
use std::f32::consts::PI;
use rand::Rng;

pub struct SolarSystemBuilder;

impl SolarSystemBuilder {
    pub fn build_realistic() -> Vec<CelestialBody> {
        let mut bodies = vec![
            // SOL (índice 0)
            CelestialBody {
                name: "Sol".to_string(),
                body_type: CelestialType::Star,
                radius: 350.0,
                orbital_params: None,
                rotation_period: 25.0,
                rotation_axis: Vec3::y(),
                parent_index: None,
            },
            // MERCURIO (índice 1)
            CelestialBody {
                name: "Mercurio".to_string(),
                body_type: CelestialType::Planet,
                radius: 12.0,
                orbital_params: Some(OrbitalParameters {
                    semi_major_axis: 2895.0,
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
                radius: 30.0,
                orbital_params: Some(OrbitalParameters {
                    semi_major_axis: 5410.0,
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
                radius: 32.0,
                orbital_params: Some(OrbitalParameters {
                    semi_major_axis: 7480.0,
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
                radius: 8.7,
                orbital_params: Some(OrbitalParameters::circular(192.0, 27.3)),
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
                    semi_major_axis: 11395.0,
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
            // FOBOS (índice 6)
            CelestialBody {
                name: "Fobos".to_string(),
                body_type: CelestialType::Moon,
                radius: 3.5,
                orbital_params: Some(OrbitalParameters::circular(47.0, 0.32)),
                rotation_period: 0.32,
                rotation_axis: Vec3::y(),
                parent_index: Some(5),
            },
            // DEIMOS (índice 7)
            CelestialBody {
                name: "Deimos".to_string(),
                body_type: CelestialType::Moon,
                radius: 2.5,
                orbital_params: Some(OrbitalParameters::circular(117.5, 1.26)),
                rotation_period: 1.26,
                rotation_axis: Vec3::y(),
                parent_index: Some(5),
            },
        ];

        // JÚPITER y sus lunas principales
        let jupiter_idx = bodies.len();
        bodies.push(CelestialBody {
            name: "Júpiter".to_string(),
            body_type: CelestialType::Planet,
            radius: 350.0,
            orbital_params: Some(OrbitalParameters {
                semi_major_axis: 38925.0,
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
        });

        // Lunas galileanas
        bodies.extend(vec![
            CelestialBody {
                name: "Ío".to_string(),
                body_type: CelestialType::Moon,
                radius: 9.1,
                orbital_params: Some(OrbitalParameters::circular(1055.0, 1.77)),
                rotation_period: 1.77,
                rotation_axis: Vec3::y(),
                parent_index: Some(jupiter_idx),
            },
            CelestialBody {
                name: "Europa".to_string(),
                body_type: CelestialType::Moon,
                radius: 7.8,
                orbital_params: Some(OrbitalParameters::circular(1681.0, 3.55)),
                rotation_period: 3.55,
                rotation_axis: Vec3::y(),
                parent_index: Some(jupiter_idx),
            },
            CelestialBody {
                name: "Ganimedes".to_string(),
                body_type: CelestialType::Moon,
                radius: 13.1,
                orbital_params: Some(OrbitalParameters::circular(2679.0, 7.15)),
                rotation_period: 7.15,
                rotation_axis: Vec3::y(),
                parent_index: Some(jupiter_idx),
            },
            CelestialBody {
                name: "Calisto".to_string(),
                body_type: CelestialType::Moon,
                radius: 12.0,
                orbital_params: Some(OrbitalParameters::circular(4712.0, 16.69)),
                rotation_period: 16.69,
                rotation_axis: Vec3::y(),
                parent_index: Some(jupiter_idx),
            },
        ]);

        // SATURNO y sus lunas
        let saturn_idx = bodies.len();
        bodies.push(CelestialBody {
            name: "Saturno".to_string(),
            body_type: CelestialType::Planet,
            radius: 300.0,
            orbital_params: Some(OrbitalParameters {
                semi_major_axis: 71675.0,
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
        });

        bodies.extend(vec![
            CelestialBody {
                name: "Titán".to_string(),
                body_type: CelestialType::Moon,
                radius: 12.9,
                orbital_params: Some(OrbitalParameters::circular(3059.0, 15.95)),
                rotation_period: 15.95,
                rotation_axis: Vec3::y(),
                parent_index: Some(saturn_idx),
            },
            CelestialBody {
                name: "Rea".to_string(),
                body_type: CelestialType::Moon,
                radius: 3.8,
                orbital_params: Some(OrbitalParameters::circular(1318.0, 4.52)),
                rotation_period: 4.52,
                rotation_axis: Vec3::y(),
                parent_index: Some(saturn_idx),
            },
            CelestialBody {
                name: "Encélado".to_string(),
                body_type: CelestialType::Moon,
                radius: 1.3,
                orbital_params: Some(OrbitalParameters::circular(596.0, 1.37)),
                rotation_period: 1.37,
                rotation_axis: Vec3::y(),
                parent_index: Some(saturn_idx),
            },
        ]);

        // URANO
        bodies.push(CelestialBody {
            name: "Urano".to_string(),
            body_type: CelestialType::Planet,
            radius: 127.0,
            orbital_params: Some(OrbitalParameters {
                semi_major_axis: 143625.0,
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
        });

        // NEPTUNO
        bodies.push(CelestialBody {
            name: "Neptuno".to_string(),
            body_type: CelestialType::Planet,
            radius: 123.0,
            orbital_params: Some(OrbitalParameters {
                semi_major_axis: 224755.0,
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
        });

        // CINTURÓN DE ASTEROIDES
        bodies.extend(Self::create_asteroid_belt(100));

        bodies
    }

    fn create_asteroid_belt(count: usize) -> Vec<CelestialBody> {
        let mut rng = rand::rng();
        let mut asteroids = Vec::new();

        for i in 0..count {
            let radius = rng.random_range(0.5..2.5);
            let semi_major_axis = rng.random_range(16000.0..25000.0);
            let eccentricity = rng.random_range(0.0..0.3);
            let inclination = rng.random_range(-15.0..15.0_f32).to_radians();
            let initial_anomaly = rng.random_range(0.0..2.0 * PI);
            let period = rng.random_range(1000.0..2500.0);

            asteroids.push(CelestialBody {
                name: format!("Asteroide-{}", i + 1),
                body_type: CelestialType::Asteroid,
                radius,
                orbital_params: Some(OrbitalParameters {
                    semi_major_axis,
                    eccentricity,
                    inclination,
                    longitude_of_ascending_node: rng.random_range(0.0..2.0 * PI),
                    argument_of_periapsis: rng.random_range(0.0..2.0 * PI),
                    orbital_period: period,
                    initial_mean_anomaly: initial_anomaly,
                }),
                rotation_period: rng.random_range(0.1..5.0),
                rotation_axis: Vec3::new(
                    rng.random_range(-1.0..1.0),
                    rng.random_range(-1.0..1.0),
                    rng.random_range(-1.0..1.0),
                )
                .normalize(),
                parent_index: None,
            });
        }

        asteroids
    }
}