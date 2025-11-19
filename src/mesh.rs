use nalgebra_glm::{Vec3};
use std::f32::consts::PI;

#[derive(Debug, Clone)]
pub struct Vertex {
    pub position: Vec3,
    pub normal: Vec3,
}

#[derive(Clone)]
pub struct ObjMesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
}

impl ObjMesh {
    pub fn create_sphere(radius: f32, rings: u32, sectors: u32) -> Self {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        vertices.push(Vertex {
            position: Vec3::new(0.0, radius, 0.0),
            normal: Vec3::new(0.0, 1.0, 0.0),
        });

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

        vertices.push(Vertex {
            position: Vec3::new(0.0, -radius, 0.0),
            normal: Vec3::new(0.0, -1.0, 0.0),
        });

        for s in 0..sectors {
            indices.push(0);
            indices.push(1 + s);
            indices.push(1 + s + 1);
        }

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

        let south_pole_index = vertices.len() as u32 - 1;
        let last_ring_start = south_pole_index - (sectors + 1);

        for s in 0..sectors {
            indices.push(last_ring_start + s);
            indices.push(south_pole_index);
            indices.push(last_ring_start + s + 1);
        }

        ObjMesh { vertices, indices }
    }

    pub fn load_from_obj(path: &str) -> Result<Self, String> {
        let (models, _) = tobj::load_obj(path, &tobj::GPU_LOAD_OPTIONS)
            .map_err(|e| format!("Error loading OBJ: {}", e))?;

        if models.is_empty() {
            return Err("No models found in OBJ file".to_string());
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
                position.normalize()
            };

            vertices.push(Vertex { position, normal });
        }

        Ok(ObjMesh {
            vertices,
            indices: mesh.indices.clone(),
        })
    }

    // FUNCIÓN AGREGADA
    pub fn create_ring(inner_radius: f32, outer_radius: f32, segments: u32) -> Self {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        for ring in 0..=1 {
            let radius = if ring == 0 {
                inner_radius
            } else {
                outer_radius
            };

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