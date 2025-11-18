use nalgebra_glm::{Vec3, Mat4, rotate_vec3};
use std::f32::consts::PI;

/// Tipos de cuerpos celestes
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CelestialType {
    Star,
    Planet,
    Moon,
    Ring,
    Asteroid,
}

/// Parámetros orbitales siguiendo las leyes de Kepler
#[derive(Clone)]
pub struct OrbitalParameters {
    pub semi_major_axis: f32,      // Semieje mayor (a)
    pub eccentricity: f32,          // Excentricidad (e): 0 = circular, <1 = elipse
    pub inclination: f32,           // Inclinación orbital (radianes)
    pub longitude_of_ascending_node: f32, // Longitud del nodo ascendente (Ω)
    pub argument_of_periapsis: f32, // Argumento del periapsis (ω)
    pub orbital_period: f32,        // Período orbital (en segundos de simulación)
    pub initial_mean_anomaly: f32,  // Anomalía media inicial
}

impl OrbitalParameters {
    /// Crea una órbita circular simple
    pub fn circular(radius: f32, period: f32) -> Self {
        Self {
            semi_major_axis: radius,
            eccentricity: 0.0,
            inclination: 0.0,
            longitude_of_ascending_node: 0.0,
            argument_of_periapsis: 0.0,
            orbital_period: period,
            initial_mean_anomaly: 0.0,
        }
    }

    /// Calcula la posición orbital en un momento dado (órbita elíptica de Kepler)
    pub fn get_position(&self, time: f32) -> Vec3 {
        if self.orbital_period == 0.0 {
            return Vec3::zeros(); // Objeto estático (ej: Sol)
        }

        // Anomalía media: M = n*t donde n = 2π/T
        let mean_motion = 2.0 * PI / self.orbital_period;
        let mean_anomaly = self.initial_mean_anomaly + mean_motion * time;

        // Resolver ecuación de Kepler: E - e*sin(E) = M
        let eccentric_anomaly = self.solve_kepler(mean_anomaly);

        // Calcular posición en el plano orbital
        let a = self.semi_major_axis;
        let e = self.eccentricity;
        let x = a * (eccentric_anomaly.cos() - e);
        let y = a * (1.0 - e * e).sqrt() * eccentric_anomaly.sin();

        // Aplicar rotaciones para orientar la órbita correctamente
        let mut pos = Vec3::new(x, 0.0, y);

        // Rotar por argumento del periapsis
        pos = rotate_vec3(&pos, self.argument_of_periapsis, &Vec3::y());

        // Rotar por inclinación
        pos = rotate_vec3(&pos, self.inclination, &Vec3::x());

        // Rotar por longitud del nodo ascendente
        pos = rotate_vec3(&pos, self.longitude_of_ascending_node, &Vec3::y());

        pos
    }

    /// Resuelve la ecuación de Kepler usando el método de Newton-Raphson
    fn solve_kepler(&self, mean_anomaly: f32) -> f32 {
        let mut eccentric_anomaly = mean_anomaly; // Primera aproximación
        let e = self.eccentricity;

        // Iteración de Newton-Raphson (máximo 10 iteraciones)
        for _ in 0..10 {
            let f = eccentric_anomaly - e * eccentric_anomaly.sin() - mean_anomaly;
            let f_prime = 1.0 - e * eccentric_anomaly.cos();

            let delta = f / f_prime;
            eccentric_anomaly -= delta;

            if delta.abs() < 1e-6 {
                break;
            }
        }

        eccentric_anomaly
    }
}

/// Representa un cuerpo celeste en el sistema solar
pub struct CelestialBody {
    pub name: String,
    pub body_type: CelestialType,
    pub radius: f32,
    pub orbital_params: Option<OrbitalParameters>,
    pub rotation_period: f32,        // Período de rotación sobre su eje
    pub rotation_axis: Vec3,
    pub parent_index: Option<usize>, // Para lunas (índice del planeta padre)
}

impl CelestialBody {
    /// Obtiene la posición absoluta del cuerpo en el espacio
    pub fn get_world_position(&self, time: f32, parent_pos: Option<Vec3>) -> Vec3 {
        let orbital_pos = match &self.orbital_params {
            Some(params) => params.get_position(time),
            None => Vec3::zeros(),
        };

        match parent_pos {
            Some(p) => p + orbital_pos,
            None => orbital_pos,
        }
    }

    /// Obtiene la matriz de modelo para renderizado
    pub fn get_model_matrix(&self, time: f32, world_pos: Vec3) -> Mat4 {
        let mut transform = Mat4::identity();

        // Traslación a la posición orbital
        transform = nalgebra_glm::translate(&transform, &world_pos);

        // Rotación sobre su propio eje
        if self.rotation_period > 0.0 {
            let rotation_angle = (time / self.rotation_period) * 2.0 * PI;
            transform = nalgebra_glm::rotate(&transform, rotation_angle, &self.rotation_axis);
        }

        // Escala según el radio
        transform = nalgebra_glm::scale(&transform, &Vec3::new(self.radius, self.radius, self.radius));

        transform
    }

    /// Genera puntos de la órbita para visualización
    pub fn get_orbit_points(&self, num_points: usize) -> Vec<Vec3> {
        match &self.orbital_params {
            Some(params) => {
                let mut points = Vec::with_capacity(num_points);
                for i in 0..num_points {
                    let t = (i as f32 / num_points as f32) * params.orbital_period;
                    points.push(params.get_position(t));
                }
                points
            }
            None => Vec::new(),
        }
    }
}