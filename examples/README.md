# Examples

Each example is a standalone renderer built on the library.
They are ordered by *concept*, not by quality: a later example is not
"better", it demonstrates the next idea.

| Example | Run | Teaches |
|---|---|---|
| `ascii_sphere` | `cargo run --example ascii_sphere` | the pixel loop, camera rays, ray–sphere intersection, Lambert shading |
| `ascii_spheres` | `cargo run --example ascii_spheres` | scene as data, the closest-hit loop |
| `png_spheres` | `cargo run --example png_spheres` | real pixels, materials, reflections, shadows, the control panel |
| `diagram_ray_sphere` | `cargo run --example diagram_ray_sphere` | diagram-as-code with plotters |

---

## `ascii_sphere`

The whole renderer in one screen: for each terminal cell, build a ray,
test it against one sphere, shade it, map brightness to an ASCII ramp.

**Look at:** `camera.ray_through(u, v)` and `sphere.hit(&ray)` — the entire
"engine" is those two calls.

**Try:**
- Change the light direction and watch the highlight move.
- Change the sphere radius / position.
- Edit the ASCII ramp and see the "material" change.

---

## `ascii_spheres`

Same renderer, but the scene is now *data*: an array of spheres.
Introduces the most important loop in ray tracing — for each ray, keep the
**closest** hit (`smallest positive t`).

**Try:**
- Move two spheres so they overlap on screen; the closest `t` must win.
- Add a sphere: one more line in the array.

---

## `png_spheres`

The ASCII renderer graduated to real pixels and a professional architecture.
The scene is rendered by the library's CPU integrator (`src/render/cpu.rs`),
which returns a `Framebuffer` that the example simply saves to disk.

**Features:**
- **Materials:** `Diffuse` (matte) and `Metal` (shiny) via the `Material` enum.
- **Reflections:** The bounce loop in the integrator allows metal spheres to reflect the sky, the ground, and each other.
- **Shadow rays:** A second ray from the hit point toward the light determines if the point is in shadow.
- **Gamma correction:** `Framebuffer` handles linear-to-sRGB conversion so dark gradients look smooth.
- **The Control Panel:** The `RenderConfig` struct allows toggling renderer features without touching the integrator code.

**Notes:**
- Shadows are perfectly sharp because the light is a directional "sun".
  Soft shadows need random sampling (Phase 3).
- Spheres far from the image center look egg-shaped. That is correct:
  the default camera is a very wide-angle pinhole.

**Try (The Control Panel):**
- Change `.shadows(false)` and watch the shadow blobs on the ground disappear.
- Change `.bounces(1)`. The metal spheres will turn pitch black! This proves that their color comes entirely from bouncing light, not direct illumination.
- Change `.bounces(2)`. The metal spheres will reflect the ground and sky, but not each other.
- Change `.samples(4)`. Nothing visible happens yet (the rays are identical), but this knob will become Anti-Aliasing in Phase 3 when we add randomness.

---

## `diagram_ray_sphere`

Renders `images/ray-sphere-intersection.png` with plotters, using the same
variable names as the code (`oc`, `half_b`, `P(t)`, ...).
Tweak positions in code and rerun — that's the whole point of
diagram-as-code.

---

## Adding a new example

1. Create `examples/<name>.rs`.
2. Run it with `cargo run --example <name>`.
3. Document it in this README (what it teaches + things to try).