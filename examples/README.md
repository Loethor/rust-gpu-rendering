# Examples

Each example is a standalone program built on the `rust_gpu_rendering` library.
They are grouped by *concept*: CPU path tracing, GPU compute pipelines, and visualization tools.

| Example | Run | Output | Teaches |
|---|---|---|---|
| `ascii_sphere` | `cargo run --example ascii_sphere` | Terminal | Pixel loops, camera rays, ray-sphere intersection |
| `ascii_spheres` | `cargo run --example ascii_spheres` | Terminal | Scene as data, the closest-hit loop |
| `png_spheres` | `cargo run --example png_spheres` | `output.png` | Full CPU path tracing, materials, the control panel |
| `gpu_gradient` | `cargo run --example gpu_gradient` | `gpu_gradient.png` | wgpu 30 boilerplate, storage buffers, basic WGSL |
| `gpu_sphere` | `cargo run --example gpu_sphere` | `gpu_sphere.png` | Porting CPU ray-math to parallel GPU compute shaders |
| `diagram_ray_sphere`| `cargo run --example diagram_ray_sphere`| `images/*.png` | Diagram-as-code with plotters |

---

## CPU Renderers

### `ascii_sphere` & `ascii_spheres`
The whole renderer in one screen. These examples deliberately avoid the `src/render/` engine and do the math manually in the file. They serve as a "look under the hood" to show the raw, unabstracted math of ray tracing (generating rays, testing intersections, mapping brightness to ASCII characters).

### `png_spheres`
The flagship CPU renderer. It builds a scene, configures the `RenderConfig` control panel, and hands it to the library's CPU integrator (`src/render/cpu.rs`). 

**Features demonstrated:**
* **Materials:** `Diffuse` (matte) and `Metal` (shiny/rough).
* **Reflections:** Multi-bounce light transport (metal spheres reflecting the sky, ground, and each other).
* **Shadow rays:** Secondary rays shot toward the light to determine occlusion.
* **The Control Panel:** Try changing `.shadows(false)`, `.bounces(1)`, or `.gi(true)` to see how the integrator toggles features on the fly.

---

## GPU Compute Renderers

### `gpu_gradient`
The "Hello World" of GPU compute shaders. This example contains ~150 lines of pure `wgpu` boilerplate required to:
1. Find a GPU adapter and open a logical device.
2. Allocate VRAM (Storage Buffers and Uniform Buffers).
3. Compile a WGSL shader and create a Compute Pipeline.
4. Dispatch 16x16 thread workgroups.
5. Map the GPU memory back to the CPU to save a PNG.

The shader itself just writes a simple red/green gradient, isolating the plumbing from the math.

### `gpu_sphere`
Takes the exact same `wgpu` plumbing from `gpu_gradient`, but replaces the shader body with a WGSL translation of `src/scene.rs`. 
* Notice how `dot(a, b)` and `normalize(v)` in Rust map perfectly to WGSL. 
* 480,000 pixels are shaded simultaneously by the GPU, proving that the rendering theory is identical to the CPU version, just executed in massive parallel.

---

## Utilities

### `diagram_ray_sphere`
Renders `images/ray-sphere-intersection.png` using the `plotters` crate. It uses the exact same variable names as the code (`oc`, `half_b`, `P(t)`). 
Tweak the coordinates in the code and rerun it to regenerate the diagram — the main benefit of diagram-as-code.

---

## Adding a new example

1. Create `examples/<name>.rs`.
2. Add any heavy dependencies (like `wgpu` or `plotters`) to `[dev-dependencies]` in `Cargo.toml` so the core library remains lightweight.
3. Run it with `cargo run --example <name>`.
4. Document it in this README.