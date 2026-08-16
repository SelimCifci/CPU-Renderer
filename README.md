# CPU Renderer

A high-performance 3D software rasterizer written from scratch in Rust with zero GPU acceleration. It implements a complete programmable graphics pipeline directly on the CPU, featuring a multi-object render queue, incremental edge rasterization, Early-Z optimization, frustum culling, and real-time Blinn-Phong illumination.

## Key Features

- **Programmable Graphics Pipeline:** Modular vertex and fragment shader closures with custom uniforms and perspective-correct attribute interpolation (`Interpolate` trait).
- **Render Queue & Scene Management:** Multi-object queuing (`Renderer`) with automatic **Front-to-Back depth sorting** to eliminate overdraw on occluded objects.
- **Incremental Edge Rasterization:** Sub-pixel triangle rasterization driven by linear step additions rather than per-pixel edge multiplications.
- **Early-Z Optimization:** Rejects occluded pixels before computing perspective division, barycentric interpolation, or fragment shading.
- **Multi-Level Culling:**
  - **Clip-Space Frustum Culling:** Discards off-screen triangles outside the 6 camera view planes before viewport transform and rasterization.
  - **Backface Culling:** Discards non-visible counter-clockwise triangles in screen space.
- **Depth Buffering:** Standard `[0.0, 1.0]` DirectX-style floating-point Z-buffer.
- **3D Lighting & Shading:** Real-time Blinn-Phong illumination (Ambient + Lambert Diffuse + Specular highlights) evaluated in world space.
- **Asset Loading & Texturing:** Wavefront `.obj` mesh loading with synchronized vertex indices (`tobj`) and UV texture sampling with coordinate wrapping (`image`).
- **Parallelism & Windowing:** Multithreaded frame clearing via `rayon` and 60 FPS real-time rendering via `minifb`.

## Tech Stack

- **Rust** (2024 Edition)
- **`glam`**: Linear algebra (vectors, matrices, camera projections)
- **`minifb`**: Real-time software framebuffer window
- **`rayon`**: Multithreaded CPU operations
- **`tobj`** & **`image`**: 3D asset and texture parsing

## Quick Start

Run in release mode for full compiler optimizations:

```bash
cargo run --release
```

Press **`Escape`** to exit the window.
