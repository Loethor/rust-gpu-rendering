# rust-gpu-rendering 🦀

A from-scratch ray tracer in Rust, built step by step as a learning project.

The project starts with a tiny linear algebra library and a CPU path tracer
that renders shaded, reflective spheres, and gradually grows towards
a GPU compute-shader ray tracer.

![png_spheres output](images/output.png)

---

## 📦 What this project contains

```text
rust-gpu-rendering/
├── Cargo.toml
├── README.md                  # You are here: overview, math, roadmap
├── images/                    # Diagrams and rendered screenshots
├── src/
│   ├── lib.rs                 # Exposes the modules
│   ├── math.rs                # Pure linear algebra: Vec3, Ray, reflect
│   ├── scene.rs               # World objects: Sphere, Camera, Material
│   ├── colors.rs              # Named palette + Color -> RGBA8 (with gamma)
│   └── render/
│       ├── mod.rs             # Module wiring
│       ├── config.rs          # RenderConfig: the control panel
│       ├── cpu.rs             # The CPU integrator (bounce loop, tracing)
│       ├── framebuffer.rs     # Output abstraction (pixels + gamma)
│       └── shading.rs         # BRDFs / light models (Lambert)
└── examples/
    ├── README.md              # Per-example docs & experiments
    ├── ascii_sphere.rs        # One sphere, terminal output
    ├── ascii_spheres.rs       # Multiple spheres, closest-hit loop
    ├── png_spheres.rs         # Real pixels, materials, control panel
    └── diagram_ray_sphere.rs  # Regenerates the math diagram (plotters)
```

The layering is deliberate:

- **`math`** stays pure: vectors, rays, operations. No domain knowledge.
- **`scene`** is the world: objects that *use* the math (`Sphere`, `Camera`, `Material`).
- **`colors`** is the shared palette used by renderers and the diagram.
- **`render`** is the engine: the integrator, shading models, the `Framebuffer`, and the `RenderConfig` control panel.
- **`examples`** are thin scripts: they build a scene, pick a config, and choose an output format (Terminal vs PNG).
  See [examples/README.md](examples/README.md) for what each one teaches.

---

## 🚀 How to run

Requirements: a Rust toolchain ([rustup.rs](https://rustup.rs)). No GPU needed (yet).

```bash
cargo run --example ascii_sphere        # terminal rendering
cargo run --example ascii_spheres       # multiple spheres
cargo run --example png_spheres         # renders output.png (materials, shadows, reflections)
cargo run --example diagram_ray_sphere  # regenerates the math diagram
cargo test                              # library unit tests
```

`--example <name>` builds and runs the standalone program in
`examples/<name>.rs` — each example is its own tiny crate that links
against this library.

---

## 📐 The math: ray–sphere intersection

![Ray–sphere intersection](images/ray-sphere-intersection.png)

### 1. The core geometric rule

Every point on a sphere's surface is exactly `radius` away from `center`.
So if the ray hits the sphere at `P(t)`:

```text
length(P(t) - center) = radius
```

Squaring both sides (a vector dotted with itself is its squared length,
which avoids a square root):

```text
dot(P(t) - center, P(t) - center) = radius^2
```

### 2. Substitute the ray equation

A point on the ray is `P(t) = r.origin + t * r.direction`.
Define the vector from the sphere center to the ray origin:

```text
oc = r.origin - center
```

Then:

```text
P(t) - center = oc + t * r.direction
```

Substituting:

```text
dot(oc + t * r.direction, oc + t * r.direction) = radius^2
```

### 3. Expand into a quadratic equation

Expanding the dot product and grouping by powers of `t`:

```text
[dot(r.direction, r.direction)] * t²
  + [2 * dot(oc, r.direction)] * t
  + [dot(oc, oc) - radius²]  =  0
```

This is `a*t² + 2*half_b*t + c = 0`, which in code is exactly:

```rust
let oc     = r.origin - center;               // center → ray origin
let a      = r.direction.dot(r.direction);    // == 1.0 if direction is normalized
let half_b = oc.dot(r.direction);             // half of the classic "b"
let c      = oc.dot(oc) - radius * radius;    // squared distance minus radius²
```

### 4. Solve for `t`

Using the quadratic formula (the 2s and 4s cancel because we used `half_b`):

```text
discriminant = half_b² - a * c
t = (-half_b ± √discriminant) / a
```

```rust
let discriminant = half_b * half_b - a * c;
```

The discriminant tells us *whether* the ray hits at all:

| discriminant | Meaning |
|--------------|---------|
| `< 0` | ray **misses** the sphere |
| `= 0` | ray **grazes** the sphere (one hit) |
| `> 0` | ray **pierces** the sphere (entry + exit hit) |

We take the closest hit in front of the camera:

```rust
let mut t = (-half_b - discriminant.sqrt()) / a;  // front face
if t < 0.0 {
    t = (-half_b + discriminant.sqrt()) / a;      // camera inside the sphere
}
```

### 5. Shade the hit

With `t` known, the hit point and surface normal are:

```rust
let hit_point = r.at(t);                          // P(t) = origin + t * direction
let normal    = (hit_point - center).normalized();
```

Lambertian diffuse plus ambient light:

```rust
let diffuse = normal.dot(light_dir).max(0.0);
let shade   = ambient + (1.0 - ambient) * diffuse;
```

Finally, `shade` is mapped onto the ASCII ramp ` .:-=+*#%@`.

---

## 🗺️ Roadmap

### Phase 1 — Foundations (single crate)
- [x] `math`: `Vec3`, `Ray` + tests
- [x] ASCII sphere renderer
- [x] Multiple spheres + closest-hit loop
- [x] Diagram-as-code (plotters)
- [x] `scene` module: `Sphere`, `Camera` + tests
- [x] `colors` palette

### Phase 2 — CPU renderer with real pixels & materials
- [x] PNG output with real colors (albedo)
- [x] Sky gradient
- [x] Ground sphere + shadow rays
- [x] Gamma correction
- [x] Materials (`Diffuse` / `Metal`) + bounce loop (`Vec3::reflect`)
- [x] Renderer promoted into the library (`src/render/`)
- [x] `RenderConfig` control panel (shadows, bounces, samples)
- [x] `Framebuffer` abstraction (separates rendering from PNG encoding)

### Phase 3 — CPU path-tracing basics
- [ ] Random sampling (RNG)
- [ ] Anti-aliasing (multi-sampling per pixel)
- [ ] Rough metal (using the `fuzz` parameter)
- [ ] True diffuse bounces (Global Illumination / color bleeding)

### Phase 4 — First GPU steps (wgpu, still one crate)
- [ ] `gpu_gradient`: minimal compute shader → PNG
- [ ] `gpu_sphere`: port the math to WGSL
- [ ] `gpu_spheres`: scene in a storage buffer

### Phase 5 — Real-time
- [ ] `winit` window + interactive camera

### Phase 6 — Workspace split (only when it hurts)
- [ ] `crates/math`, `crates/scene`, `crates/cpu`, `crates/gpu`

---

## 📝 Notes

- The math library is intentionally tiny: it contains only what the renderer
  needs. If the project grows, it can later be swapped for `glam`.
- The library is dependency-free; PNG encoding is handled by the `image` crate
  in the examples (or via an opt-in Cargo feature).
- The CPU examples are deliberately simple and sequential; they exist to make
  the math obvious before parallelizing it on the GPU.
- The diagram is generated by code: `cargo run --example diagram_ray_sphere`.