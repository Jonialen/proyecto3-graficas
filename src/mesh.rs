use nalgebra_glm::Vec3;
use std::f32::consts::PI;

/// Representa un vértice de malla con posición y normal.
///
/// Esta estructura se utiliza de manera genérica por todos los objetos
/// tridimensionales, tanto los generados procedimentalmente
/// como los cargados desde archivos `.obj`.
#[derive(Debug, Clone)]
pub struct Vertex {
    /// Posición en el espacio 3D (en unidades del modelo).
    pub position: Vec3,
    /// Vector normal asociado al vértice (para iluminación).
    pub normal: Vec3,
}

/// Representa una malla 3D con vértices e índices de triángulo.
///
/// Se utiliza por el motor de renderizado software para representar
/// esferas, anillos, modelos importados o geometría auxiliar.
#[derive(Clone)]
pub struct ObjMesh {
    /// Lista de vértices de la malla en coordenadas locales.
    pub vertices: Vec<Vertex>,
    /// Índices de triángulo (tripletas que forman cada cara).
    pub indices: Vec<u32>,
}

impl ObjMesh {
    // ========================================================================
    // GENERACIÓN DE MALLAS PROCEDURALES
    // ========================================================================

    /// Genera una esfera procedimentalmente utilizando coordenadas esféricas.
    ///
    /// # Parámetros
    /// * `radius`: Radio de la esfera.
    /// * `rings`: Número de divisiones horizontales (de polo a polo).
    /// * `sectors`: Número de divisiones verticales (alrededor del eje Y).
    ///
    /// # Retorna
    /// Nueva malla `ObjMesh` que representa la esfera.
    pub fn create_sphere(radius: f32, rings: u32, sectors: u32) -> Self {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        // Polo norte
        vertices.push(Vertex {
            position: Vec3::new(0.0, radius, 0.0),
            normal: Vec3::new(0.0, 1.0, 0.0),
        });

        // Vértices intermedios por anillos
        for r in 1..rings {
            for s in 0..=sectors {
                let theta = PI * r as f32 / rings as f32;
                let phi = 2.0 * PI * s as f32 / sectors as f32;

                let x = theta.sin() * phi.cos();
                let y = theta.cos();
                let z = theta.sin() * phi.sin();

                let position = Vec3::new(x * radius, y * radius, z * radius);
                let normal = Vec3::new(x, y, z);

                vertices.push(Vertex { position, normal });
            }
        }

        // Polo sur
        vertices.push(Vertex {
            position: Vec3::new(0.0, -radius, 0.0),
            normal: Vec3::new(0.0, -1.0, 0.0),
        });

        // Triángulos que conectan el polo norte con el primer anillo
        for s in 0..sectors {
            indices.push(0);
            indices.push(1 + s);
            indices.push(1 + s + 1);
        }

        // Triángulos intermedios (entre los anillos)
        for r in 0..(rings - 2) {
            for s in 0..sectors {
                let current = 1 + r * (sectors + 1) + s;
                let next = current + sectors + 1;

                indices.push(current);
                indices.push(next);
                indices.push(current + 1);

                indices.push(current + 1);
                indices.push(next);
                indices.push(next + 1);
            }
        }

        // Triángulos que conectan el último anillo con el polo sur
        let south_pole_index = vertices.len() as u32 - 1;
        let last_ring_start = south_pole_index - (sectors + 1);

        for s in 0..sectors {
            indices.push(last_ring_start + s);
            indices.push(south_pole_index);
            indices.push(last_ring_start + s + 1);
        }

        ObjMesh { vertices, indices }
    }

    // ========================================================================
    // CARGA DE ARCHIVOS OBJ
    // ========================================================================

    /// Carga una malla desde un archivo `.obj` estándar.
    ///
    /// # Parámetros
    /// * `path` - Ruta al archivo OBJ.
    ///
    /// # Errores
    /// Devuelve un `Err(String)` si ocurre un error de lectura o
    /// si el archivo no contiene geometría válida.
    pub fn load_from_obj(path: &str) -> Result<Self, String> {
        let (models, _) =
            tobj::load_obj(path, &tobj::GPU_LOAD_OPTIONS)
                .map_err(|e| format!("Error al cargar OBJ: {}", e))?;

        if models.is_empty() {
            return Err("El archivo OBJ no contiene modelos válidos".to_string());
        }

        let mesh = &models[0].mesh;
        let mut vertices = Vec::new();

        for i in 0..mesh.positions.len() / 3 {
            let position = Vec3::new(
                mesh.positions[i * 3],
                mesh.positions[i * 3 + 1],
                mesh.positions[i * 3 + 2],
            );

            let normal = if !mesh.normals.is_empty() {
                Vec3::new(
                    mesh.normals[i * 3],
                    mesh.normals[i * 3 + 1],
                    mesh.normals[i * 3 + 2],
                )
                .normalize()
            } else {
                // Si no hay normales, se usa la dirección del vértice normalizada.
                position.normalize()
            };

            vertices.push(Vertex { position, normal });
        }

        Ok(ObjMesh {
            vertices,
            indices: mesh.indices.clone(),
        })
    }

    // ========================================================================
    // MALLA DE ANILLO (por ejemplo, para Saturno)
    // ========================================================================

    /// Genera una malla de anillo plano (por ejemplo, los anillos de Saturno).
    ///
    /// # Parámetros
    /// * `inner_radius` - Radio interno del anillo.
    /// * `outer_radius` - Radio externo del anillo.
    /// * `segments` - Número de divisiones angulares.
    pub fn create_ring(inner_radius: f32, outer_radius: f32, segments: u32) -> Self {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        // Dos anillos concéntricos: interno y externo
        for ring in 0..=1 {
            let radius = if ring == 0 { inner_radius } else { outer_radius };

            for s in 0..=segments {
                let angle = 2.0 * PI * s as f32 / segments as f32;
                let x = angle.cos() * radius;
                let z = angle.sin() * radius;

                vertices.push(Vertex {
                    position: Vec3::new(x, 0.0, z),
                    normal: Vec3::new(0.0, 1.0, 0.0),
                });
            }
        }

        // Triángulos entre los dos anillos
        for s in 0..segments {
            let i0 = s;
            let i1 = s + 1;
            let i2 = s + segments + 1;
            let i3 = s + segments + 2;

            indices.push(i0);
            indices.push(i2);
            indices.push(i1);

            indices.push(i1);
            indices.push(i2);
            indices.push(i3);
        }

        ObjMesh { vertices, indices }
    }
}