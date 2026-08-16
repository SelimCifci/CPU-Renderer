use glam::{Vec2, Vec3};
use tobj::{LoadOptions, load_obj};

use crate::pipeline::Vertex;

pub struct Mesh<V> {
    pub vertices: Vec<Vertex<V>>,
}

impl<V> Mesh<V> {
    pub fn new(vertices: Vec<Vertex<V>>) -> Self {
        Self { vertices }
    }
}

impl Mesh<(Vec3, Vec2)> {
    pub fn load_obj(path: &str) -> Self {
        let (models, _materials) = tobj::load_obj(
            path,
            &LoadOptions {
                triangulate: true,
                single_index: true,
                ..Default::default()
            },
        )
        .expect("Failed to load OBJ file");
        let mut vertices = Vec::new();

        for model in models {
            let mesh = &model.mesh;

            for triangle_indices in mesh.indices.chunks_exact(3) {
                let reordered_indices = [
                    triangle_indices[0],
                    triangle_indices[2],
                    triangle_indices[1],
                ];

                for &idx in &reordered_indices {
                    let i = idx as usize;

                    let px = mesh.positions[3 * i];
                    let py = mesh.positions[3 * i + 1];
                    let pz = mesh.positions[3 * i + 2];
                    let position = Vec3::new(px, py, pz);

                    let normal = if !mesh.normals.is_empty() {
                        Vec3::new(
                            mesh.normals[3 * i],
                            mesh.normals[3 * i + 1],
                            mesh.normals[3 * i + 2],
                        )
                    } else {
                        Vec3::new(0.0, 1.0, 0.0)
                    };

                    let uv = if !mesh.texcoords.is_empty() {
                        Vec2::new(mesh.texcoords[2 * i], mesh.texcoords[2 * i + 1])
                    } else {
                        Vec2::ZERO
                    };

                    vertices.push(Vertex {
                        position,
                        varying: (normal, uv),
                    });
                }
            }
        }

        Self::new(vertices)
    }

    pub fn cube() -> Self {
        let mut vertices = Vec::with_capacity(36);

        let mut add_quad = |bl: Vec3, br: Vec3, tr: Vec3, tl: Vec3, normal: Vec3| {
            let uv_bl = Vec2::new(0.0, 0.0);
            let uv_br = Vec2::new(1.0, 0.0);
            let uv_tr = Vec2::new(1.0, 1.0);
            let uv_tl = Vec2::new(0.0, 1.0);

            vertices.push(Vertex {
                position: bl,
                varying: (normal, uv_bl),
            });
            vertices.push(Vertex {
                position: tr,
                varying: (normal, uv_tr),
            });
            vertices.push(Vertex {
                position: br,
                varying: (normal, uv_br),
            });

            vertices.push(Vertex {
                position: bl,
                varying: (normal, uv_bl),
            });
            vertices.push(Vertex {
                position: tl,
                varying: (normal, uv_tl),
            });
            vertices.push(Vertex {
                position: tr,
                varying: (normal, uv_tr),
            });
        };

        add_quad(
            Vec3::new(-0.5, -0.5, 0.5),
            Vec3::new(0.5, -0.5, 0.5),
            Vec3::new(0.5, 0.5, 0.5),
            Vec3::new(-0.5, 0.5, 0.5),
            Vec3::new(0.0, 0.0, 1.0),
        );

        add_quad(
            Vec3::new(0.5, -0.5, -0.5),
            Vec3::new(-0.5, -0.5, -0.5),
            Vec3::new(-0.5, 0.5, -0.5),
            Vec3::new(0.5, 0.5, -0.5),
            Vec3::new(0.0, 0.0, -1.0),
        );

        add_quad(
            Vec3::new(-0.5, -0.5, -0.5),
            Vec3::new(-0.5, -0.5, 0.5),
            Vec3::new(-0.5, 0.5, 0.5),
            Vec3::new(-0.5, 0.5, -0.5),
            Vec3::new(-1.0, 0.0, 0.0),
        );

        add_quad(
            Vec3::new(0.5, -0.5, 0.5),
            Vec3::new(0.5, -0.5, -0.5),
            Vec3::new(0.5, 0.5, -0.5),
            Vec3::new(0.5, 0.5, 0.5),
            Vec3::new(1.0, 0.0, 0.0),
        );

        add_quad(
            Vec3::new(-0.5, 0.5, 0.5),
            Vec3::new(0.5, 0.5, 0.5),
            Vec3::new(0.5, 0.5, -0.5),
            Vec3::new(-0.5, 0.5, -0.5),
            Vec3::new(0.0, 1.0, 0.0),
        );

        add_quad(
            Vec3::new(-0.5, -0.5, -0.5),
            Vec3::new(0.5, -0.5, -0.5),
            Vec3::new(0.5, -0.5, 0.5),
            Vec3::new(-0.5, -0.5, 0.5),
            Vec3::new(0.0, -1.0, 0.0),
        );

        Self::new(vertices)
    }
}
