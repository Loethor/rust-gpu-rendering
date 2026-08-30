# learn-rendering 🦀

A from-scratch ray tracer in Rust, built step by step as a learning project.

The project starts with a tiny linear algebra library and a CPU ray tracer
that renders a shaded sphere into the terminal, and will gradually grow
towards a GPU compute-shader ray tracer.

---

## 📦 What this project contains

```text
learn-rendering/
├── Cargo.toml
├── README.md
├── images/
│   └── ray-sphere-intersection.png
├── src/
│   ├── lib.rs     # Exposes the `math` module
│   └── math.rs    # Minimal linear algebra: Vec3, Ray, operators
└── examples/
    └── ascii_sphere.rs   # CPU ray tracer that renders to the terminal
```

### `src/math.rs` — the math library

| Item     | Purpose |
|----------|---------|
| `Vec3`   | 3D vector with `+`, `-`, `*`, `/`, `dot`, `cross`, `normalized`, `length`, ... |
| `Point3` | Alias of `Vec3`, used for positions in space |
| `Color`  | Alias of `Vec3`, used for RGB colors |
| `Ray`    | `origin + t * direction`, with `at(t)` to evaluate a point along the ray |

### `examples/ascii_sphere.rs` — the renderer

A complete CPU ray tracer in ~100 lines. For every terminal cell it:

1. Places a **camera** at the origin and a virtual image plane at `z = -1`.
2. Computes `(u, v)` coordinates in `[-1, 1]` for the pixel (with an
   aspect-ratio correction, because terminal characters are ~2x taller than wide).
3. Shoots a **ray** from the camera through that pixel.
4. Tests the ray against a **sphere** using the quadratic equation (see below).
5. On a hit, computes the surface **normal** and shades it with
   Lambertian diffuse lighting plus ambient light.
6. Maps brightness to an ASCII ramp: ` .:-=+*#%@`

---

## 🚀 How to run

Requirements: a Rust toolchain ([rustup.rs](https://rustup.rs)). No GPU needed.

```bash
# Render the ASCII sphere into the terminal
cargo run --example ascii_sphere

# Run the math library unit tests
cargo test
```

Expected output: a shaded 3D sphere drawn with ASCII characters.

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

### Text version of the diagram

```text
                   P(t)   ← hit point on the sphere
                     ●
                    /|
                   / |
    r.direction   /  |  P(t) - center
              ↗  /   |  (length == radius)
              | /    |
              |/     |
              /      ● center
             /     / |
            /    /   | radius
           /   / t
          /  /
         / /
        ●
     r.origin          oc = r.origin - center
```

---

## 🗺️ Roadmap

- [x] Minimal linear algebra library (`Vec3`, `Ray`)
- [x] ASCII sphere — CPU ray tracer in the terminal
- [ ] PNG sphere — same renderer writing an image file
- [ ] Multiple spheres, materials, camera controls
- [ ] GPU version — port the same math to a `wgpu` compute shader (WGSL)

---

## 📝 Notes

- The math library is intentionally tiny: it contains only what the renderer
  needs. If the project grows, it can later be swapped for `glam`.
- The CPU examples are deliberately simple and sequential; they exist to make
  the math obvious before parallelizing it on the GPU.