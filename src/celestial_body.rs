use nalgebra_glm::{Vec3, Mat4, rotate_vec3};
use std::f32::consts::PI;

/// Enumeración que define los tipos posibles de cuerpos celestes.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CelestialType {
    /// Estrella principal (ej. el Sol).
    Star,
    /// Planeta.
    Planet,
    /// Luna o satélite natural.
    Moon,
    /// Asteroide u objeto menor.
    Asteroid,
}

/// Representa los parámetros orbitales de un cuerpo celeste según las leyes de Kepler.
///
/// Determina la posición relativa de un cuerpo en su órbita elíptica durante la simulación.
#[derive(Clone)]
pub struct OrbitalParameters {
    /// Semieje mayor de la órbita (en unidades arbitrarias).
    pub semi_major_axis: f32,
    /// Excentricidad orbital (0 = circular, <1 = elíptica).
    pub eccentricity: f32,
    /// Inclinación orbital respecto al plano de referencia (en radianes).
    pub inclination: f32,
    /// Longitud del nodo ascendente (Ω, en radianes).
    pub longitude_of_ascending_node: f32,
    /// Argumento del periapsis (ω, en radianes).
    pub argument_of_periapsis: f32,
    /// Período orbital (en segundos simulados).
    pub orbital_period: f32,
    /// Anomalía media inicial (posición angular inicial en la órbita).
    pub initial_mean_anomaly: f32,
}

impl OrbitalParameters {
    /// Crea una órbita circular simple con un radio y período definidos.
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

    /// Calcula la posición orbital tridimensional de un objeto en un tiempo dado.
    ///
    /// Implementa las ecuaciones de Kepler para órbitas elípticas.
    ///
    /// # Parámetros
    /// * `time`: Tiempo actual de simulación.
    ///
    /// # Retorna
    /// Vector 3D con la posición resultante.
    pub fn get_position(&self, time: f32) -> Vec3 {
        if self.orbital_period == 0.0 {
            return Vec3::zeros(); // Objeto estacionario (por ejemplo, el Sol).
        }

        // Cálculo de la anomalía media M = n * t, donde n = 2π / T
        let mean_motion = 2.0 * PI / self.orbital_period;
        let mean_anomaly = self.initial_mean_anomaly + mean_motion * time;

        // Resolución numérica de la ecuación de Kepler: E - e sin(E) = M
        let eccentric_anomaly = self.solve_kepler(mean_anomaly);

        // Coordenadas en el plano orbital
        let a = self.semi_major_axis;
        let e = self.eccentricity;
        let x = a * (eccentric_anomaly.cos() - e);
        let y = a * (1.0 - e * e).sqrt() * eccentric_anomaly.sin();

        // Posición inicial en el plano orbital
        let mut pos = Vec3::new(x, 0.0, y);

        // Aplicación de rotaciones orbitales en orden:
        pos = rotate_vec3(&pos, self.argument_of_periapsis, &Vec3::y()); // ω
        pos = rotate_vec3(&pos, self.inclination, &Vec3::x()); // i
        pos = rotate_vec3(&pos, self.longitude_of_ascending_node, &Vec3::y()); // Ω

        pos
    }

    /// Resuelve la ecuación de Kepler mediante el método de Newton-Raphson.
    ///
    /// # Parámetros
    /// * `mean_anomaly`: Anomalía media M (en radianes).
    ///
    /// # Retorna
    /// Anomalía excéntrica E (en radianes).
    fn solve_kepler(&self, mean_anomaly: f32) -> f32 {
        let mut eccentric_anomaly = mean_anomaly; // Estimación inicial
        let e = self.eccentricity;

        // Iteración de Newton-Raphson (máximo 10 pasos).
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

/// Representa un cuerpo celeste (estrella, planeta, luna o asteroide) dentro del sistema.
///
/// Incluye sus propiedades físicas, parámetros de rotación, y si aplica, su órbita alrededor de otro cuerpo.
pub struct CelestialBody {
    /// Nombre del cuerpo celeste (por ejemplo, "Tierra", "Marte").
    pub name: String,
    /// Tipo del cuerpo celeste (planeta, luna, etc.).
    pub body_type: CelestialType,
    /// Radio visual del cuerpo (en unidades de simulación).
    pub radius: f32,
    /// Parámetros orbitales. `None` si el cuerpo está fijo (por ejemplo, el Sol).
    pub orbital_params: Option<OrbitalParameters>,
    /// Periodo de rotación sobre su propio eje (en días simulados).
    pub rotation_period: f32,
    /// Vector unitario que define el eje de rotación.
    pub rotation_axis: Vec3,
    /// Índice del cuerpo padre en la jerarquía (por ejemplo, planeta padre de una luna).
    pub parent_index: Option<usize>,
}

impl CelestialBody {
    /// Retorna la posición absoluta del cuerpo en el sistema de coordenadas global.
    ///
    /// Si tiene un cuerpo padre, la posición resultante será relativa al mismo.
    pub fn get_world_position(&self, time: f32, parent_pos: Option<Vec3>) -> Vec3 {
        let orbital_pos = match &self.orbital_params {
            Some(params) => params.get_position(time),
            _ => Vec3::zeros(),
        };

        match parent_pos {
            Some(p) => p + orbital_pos,
            _none => orbital_pos,
        }
    }

    /// Calcula la matriz modelo del cuerpo para su representación gráfica.
    ///
    /// Incluye transformaciones de traslación, rotación y escala.
    pub fn get_model_matrix(&self, time: f32, world_pos: Vec3) -> Mat4 {
        let mut transform = Mat4::identity();

        // Traslación a la posición espacial del cuerpo.
        transform = nalgebra_glm::translate(&transform, &world_pos);

        // Rotación axial (solo si el periodo es distinto de cero).
        if self.rotation_period > 0.0 {
            let rotation_angle = (time / self.rotation_period) * 2.0 * PI;
            transform =
                nalgebra_glm::rotate(&transform, rotation_angle, &self.rotation_axis);
        }

        // Escala uniforme según el radio visual.
        transform =
            nalgebra_glm::scale(&transform, &Vec3::new(self.radius, self.radius, self.radius));

        transform
    }

    /// Genera un conjunto de puntos de la órbita para su visualización.
    ///
    /// Esto permite renderizar líneas orbitales o trayectorias.
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
            _ => Vec::new(),
        }
    }
}