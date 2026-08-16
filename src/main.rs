use glam::{Mat4, Vec2, Vec3};
use minifb::{Key, Window, WindowOptions};

mod color;
mod framebuffer;
mod mesh;
mod pipeline;
mod texture;
use crate::color::Color;
use crate::framebuffer::Framebuffer;
use crate::mesh::Mesh;
use crate::pipeline::{Vertex, draw_mesh};
use crate::texture::Texture;

const WIDTH: usize = 600;
const HEIGHT: usize = 600;

fn main() {
    let mut window = Window::new("CPU Renderer", WIDTH, HEIGHT, WindowOptions::default())
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

    window.set_target_fps(60);

    let mut framebuffer = Framebuffer::new(WIDTH, HEIGHT);
    framebuffer.set_clear_color(Color {
        r: 0.1,
        g: 0.1,
        b: 0.1,
    });

    let texture = Texture::load_from_file("assets/container.jpg");
    let cube = Mesh::cube();
    let monkey = Mesh::load_obj("assets/monkey.obj");

    let mut angle = 0.0;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        framebuffer.clear();
        framebuffer.clear_depth();

        angle += 0.02;

        let model = Mat4::from_translation(Vec3::new(0.0, 0.0, 4.5)) * Mat4::from_rotation_y(angle);
        //* Mat4::from_rotation_x(angle * 0.6);
        let view = glam::camera::lh::view::look_at_mat4(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
            Vec3::new(0.0, 1.0, 0.0),
        );
        let proj = glam::camera::lh::proj::directx::perspective(
            45.0_f32.to_radians(),
            framebuffer.get_width() as f32 / framebuffer.get_height() as f32,
            0.1,
            100.0,
        );
        let mvp = proj * view * model;

        let light_dir = Vec3::new(0.5, 1.0, -1.0).normalize();
        let camera_pos = Vec3::new(0.0, 0.0, 0.0);

        let vertex_shader = |vertex: Vertex<(Vec3, Vec2)>| {
            let clip_pos = mvp * vertex.position.extend(1.0);
            let world_pos = (model * vertex.position.extend(1.0)).truncate();
            let world_normal = (model * vertex.varying.0.extend(0.0))
                .truncate()
                .normalize();
            let uv = vertex.varying.1;

            (clip_pos, (world_pos, world_normal, uv))
        };

        let fragment_shader = |(world_pos, normal, uv): (Vec3, Vec3, Vec2)| -> Option<Color> {
            let n = normal.normalize();

            let ambient = 0.15;
            let diff = n.dot(light_dir).max(0.0);

            let view_dir = (camera_pos - world_pos).normalize();
            let half_dir = (light_dir + view_dir).normalize();
            let spec = n.dot(half_dir).max(0.0).powf(32.0) * 0.5;

            let base_color = Color::new(0.9, 0.6, 0.3);
            let total_intensity = ambient + diff;
            Some(Color {
                r: base_color.r * total_intensity + spec,
                g: base_color.g * total_intensity + spec,
                b: base_color.b * total_intensity + spec,
            })
        };

        draw_mesh(&mut framebuffer, &monkey, vertex_shader, fragment_shader);

        window
            .update_with_buffer(framebuffer.get_buffer(), WIDTH, HEIGHT)
            .unwrap();
    }
}
