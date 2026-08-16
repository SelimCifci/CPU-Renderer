use glam::{Mat4, Vec2, Vec3, Vec4};

use crate::color::Color;
use crate::framebuffer::Framebuffer;
use crate::mesh::Mesh;
use crate::pipeline::{Interpolate, Vertex, draw_mesh};

pub struct RenderObject<'a> {
    pub mesh: &'a Mesh<(Vec3, Vec2)>,
    pub model: Mat4,
}

pub struct Renderer<'a> {
    render_objects: Vec<RenderObject<'a>>,
}

impl<'a> Renderer<'a> {
    pub fn new() -> Self {
        Self {
            render_objects: Vec::new(),
        }
    }

    pub fn queue_mesh(&mut self, mesh: &'a Mesh<(Vec3, Vec2)>, model: Mat4) {
        self.render_objects.push(RenderObject { mesh, model });
    }

    pub fn render<Out, VS, FS>(
        &mut self,
        framebuffer: &mut Framebuffer,
        view: Mat4,
        proj: Mat4,
        camera_pos: Vec3,
        vertex_shader: VS,
        fragment_shader: FS,
    ) where
        Out: Interpolate,
        VS: Fn(Vertex<(Vec3, Vec2)>, Mat4, Mat4) -> (Vec4, Out),
        FS: Fn(Out) -> Option<Color>,
    {
        self.render_objects.sort_by(|a, b| {
            let pos_a = a.model.w_axis.truncate();
            let pos_b = b.model.w_axis.truncate();

            let dist_a = pos_a.distance_squared(camera_pos);
            let dist_b = pos_b.distance_squared(camera_pos);

            dist_a
                .partial_cmp(&dist_b)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        for object in &self.render_objects {
            let mvp = proj * view * object.model;

            draw_mesh(
                framebuffer,
                object.mesh,
                |vertex| vertex_shader(vertex, object.model, mvp),
                &fragment_shader,
            );
        }

        self.render_objects.clear();
    }
}
