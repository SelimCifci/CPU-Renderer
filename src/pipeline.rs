use glam::{Vec2, Vec3, Vec4};
use std::ops::{Add, Mul};

use crate::mesh::Mesh;
use crate::{color::Color, framebuffer::Framebuffer};

pub trait Interpolate: Copy {
    fn barycentric(v0: Self, v1: Self, v2: Self, w0: f32, w1: f32, w2: f32) -> Self;
}
impl Interpolate for Vec2 {
    fn barycentric(v0: Self, v1: Self, v2: Self, w0: f32, w1: f32, w2: f32) -> Self {
        v0 * w0 + v1 * w1 + v2 * w2
    }
}
impl Interpolate for Vec3 {
    fn barycentric(v0: Self, v1: Self, v2: Self, w0: f32, w1: f32, w2: f32) -> Self {
        v0 * w0 + v1 * w1 + v2 * w2
    }
}
impl<A: Interpolate, B: Interpolate> Interpolate for (A, B) {
    fn barycentric(v0: Self, v1: Self, v2: Self, w0: f32, w1: f32, w2: f32) -> Self {
        (
            A::barycentric(v0.0, v1.0, v2.0, w0, w1, w2),
            B::barycentric(v0.1, v1.1, v2.1, w0, w1, w2),
        )
    }
}
impl<A: Interpolate, B: Interpolate, C: Interpolate> Interpolate for (A, B, C) {
    fn barycentric(v0: Self, v1: Self, v2: Self, w0: f32, w1: f32, w2: f32) -> Self {
        (
            A::barycentric(v0.0, v1.0, v2.0, w0, w1, w2),
            B::barycentric(v0.1, v1.1, v2.1, w0, w1, w2),
            C::barycentric(v0.2, v1.2, v2.2, w0, w1, w2),
        )
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Vertex<V> {
    pub position: Vec3,
    pub varying: V,
}
struct ClipVertex<V> {
    position: Vec4,
    varying: V,
}
struct ScreenVertex<V> {
    position: Vec3,
    inv_w: f32,
    varying: V,
}

fn clip_to_screen<V>(cv: ClipVertex<V>, width: f32, height: f32) -> ScreenVertex<V> {
    let ndc_x = cv.position.x / cv.position.w;
    let ndc_y = cv.position.y / cv.position.w;
    let ndc_z = cv.position.z / cv.position.w;

    let screen_x = (ndc_x + 1.0) * 0.5 * width;
    let screen_y = (1.0 - ndc_y) * 0.5 * height;

    let inv_w = 1.0 / cv.position.w;

    ScreenVertex {
        position: Vec3::new(screen_x, screen_y, ndc_z),
        inv_w,
        varying: cv.varying,
    }
}

fn rasterize_triangle<V, FS>(
    framebuffer: &mut Framebuffer,
    sv0: ScreenVertex<V>,
    sv1: ScreenVertex<V>,
    sv2: ScreenVertex<V>,
    fragment_shader: FS,
) where
    V: Interpolate,
    FS: Fn(V) -> Option<Color>,
{
    let min_x = sv0
        .position
        .x
        .min(sv1.position.x)
        .min(sv2.position.x)
        .max(0.0);
    let max_x = sv0
        .position
        .x
        .max(sv1.position.x)
        .max(sv2.position.x)
        .min((framebuffer.get_width() - 1) as f32);
    let min_y = sv0
        .position
        .y
        .min(sv1.position.y)
        .min(sv2.position.y)
        .max(0.0);
    let max_y = sv0
        .position
        .y
        .max(sv1.position.y)
        .max(sv2.position.y)
        .min((framebuffer.get_height() - 1) as f32);

    fn edge_function(a: Vec3, b: Vec3, c: Vec3) -> f32 {
        (c.x - a.x) * (b.y - a.y) - (c.y - a.y) * (b.x - a.x)
    }

    let area = edge_function(sv0.position, sv1.position, sv2.position);
    if area <= 0.0 {
        return;
    }
    let inv_area = 1.0 / area;

    let min_x_int = min_x.floor() as u32;
    let max_x_int = max_x.ceil() as u32;
    let min_y_int = min_y.floor() as u32;
    let max_y_int = max_y.ceil() as u32;

    let step_x0 = (sv2.position.y - sv1.position.y) * inv_area;
    let step_y0 = (sv1.position.x - sv2.position.x) * inv_area;
    let step_x1 = (sv0.position.y - sv2.position.y) * inv_area;
    let step_y1 = (sv2.position.x - sv0.position.x) * inv_area;
    let step_x2 = (sv1.position.y - sv0.position.y) * inv_area;
    let step_y2 = (sv0.position.x - sv1.position.x) * inv_area;

    let p_start = Vec3::new(min_x_int as f32 + 0.5, min_y_int as f32 + 0.5, 0.0);
    let mut row_w0 = edge_function(sv1.position, sv2.position, p_start) * inv_area;
    let mut row_w1 = edge_function(sv2.position, sv0.position, p_start) * inv_area;
    let mut row_w2 = edge_function(sv0.position, sv1.position, p_start) * inv_area;

    for y in min_y_int..=max_y_int {
        let mut w0 = row_w0;
        let mut w1 = row_w1;
        let mut w2 = row_w2;

        for x in min_x_int..=max_x_int {
            if w0 >= 0.0 && w1 >= 0.0 && w2 >= 0.0 {
                let z = w0 * sv0.position.z + w1 * sv1.position.z + w2 * sv2.position.z;

                if z < framebuffer.get_pixel_z(x, y) {
                    let inv_w = w0 * sv0.inv_w + w1 * sv1.inv_w + w2 * sv2.inv_w;
                    let w = 1.0 / inv_w;

                    let k0 = w0 * sv0.inv_w * w;
                    let k1 = w1 * sv1.inv_w * w;
                    let k2 = w2 * sv2.inv_w * w;

                    let varying = V::barycentric(sv0.varying, sv1.varying, sv2.varying, k0, k1, k2);
                    if let Some(color) = fragment_shader(varying) {
                        framebuffer.test_z_set_pixel(x, y, z, color.into());
                    }
                }
            }

            w0 += step_x0;
            w1 += step_x1;
            w2 += step_x2;
        }

        row_w0 += step_y0;
        row_w1 += step_y1;
        row_w2 += step_y2;
    }
}

pub fn draw_triangle<In, Out, VS, FS>(
    framebuffer: &mut Framebuffer,
    v0: In,
    v1: In,
    v2: In,
    vertex_shader: VS,
    fragment_shader: FS,
) where
    Out: Interpolate,
    VS: Fn(In) -> (Vec4, Out),
    FS: Fn(Out) -> Option<Color>,
{
    let (p0, v0) = vertex_shader(v0);
    let (p1, v1) = vertex_shader(v1);
    let (p2, v2) = vertex_shader(v2);

    let cv0 = ClipVertex {
        position: p0,
        varying: v0,
    };
    let cv1 = ClipVertex {
        position: p1,
        varying: v1,
    };
    let cv2 = ClipVertex {
        position: p2,
        varying: v2,
    };

    if cv0.position.w <= 0.0 || cv1.position.w <= 0.0 || cv2.position.w <= 0.0 {
        return;
    }

    let all_left = cv0.position.x < -cv0.position.w
        && cv1.position.x < -cv1.position.w
        && cv2.position.x < -cv2.position.w;
    let all_right = cv0.position.x > cv0.position.w
        && cv1.position.x > cv1.position.w
        && cv2.position.x > cv2.position.w;
    let all_bottom = cv0.position.y < -cv0.position.w
        && cv1.position.y < -cv1.position.w
        && cv2.position.y < -cv2.position.w;
    let all_top = cv0.position.y > cv0.position.w
        && cv1.position.y > cv1.position.w
        && cv2.position.y > cv2.position.w;
    let all_far = cv0.position.z > cv0.position.w
        && cv1.position.z > cv1.position.w
        && cv2.position.z > cv2.position.w;
    let all_near = cv0.position.z < 0.0 && cv1.position.z < 0.0 && cv2.position.z < 0.0;
    if all_left || all_right || all_bottom || all_top || all_far || all_near {
        return;
    }

    let width = framebuffer.get_width() as f32;
    let height = framebuffer.get_height() as f32;

    let sv0 = clip_to_screen(cv0, width, height);
    let sv1 = clip_to_screen(cv1, width, height);
    let sv2 = clip_to_screen(cv2, width, height);

    rasterize_triangle(framebuffer, sv0, sv1, sv2, fragment_shader);
}

pub fn draw_mesh<In: Copy, Out, VS, FS>(
    framebuffer: &mut Framebuffer,
    mesh: &Mesh<In>,
    vertex_shader: VS,
    fragment_shader: FS,
) where
    Out: Interpolate,
    VS: Fn(Vertex<In>) -> (Vec4, Out),
    FS: Fn(Out) -> Option<Color>,
{
    for triangle in mesh.vertices.chunks_exact(3) {
        draw_triangle(
            framebuffer,
            triangle[0],
            triangle[1],
            triangle[2],
            &vertex_shader,
            &fragment_shader,
        );
    }
}
