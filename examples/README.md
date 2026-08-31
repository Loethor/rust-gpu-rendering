# Examples

Each example is a standalone renderer built on the library.
They are ordered by *concept*, not by quality: a later example is not
"better", it demonstrates the next idea.

| Example | Run | Teaches |
|---|---|---|
| `ascii_sphere` | `cargo run --example ascii_sphere` | the pixel loop, camera rays, ray–sphere intersection, Lambert shading |
| `ascii_spheres` | `cargo run --example ascii_spheres` | scene as data, the closest-hit loop |
| `png_spheres` | `cargo run --example png_spheres` | real pixels, albedo, sky, shadow rays, gamma |
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

The ASCII renderer graduated to real pixels:

- **Albedo colors** from the shared palette.
- **Sky gradient** blended from the ray's Y direction.
- **Ground** = one giant sphere far below (classic trick).
- **Shadow rays**: a second ray from the hit point toward the light;
  if anything blocks it, the point gets ambient light only.
- **Gamma correction** in `colors::to_rgba8`, so dark gradients look smooth.

**Notes:**
- Shadows are perfectly sharp because the light is a directional "sun".
  Soft shadows need random sampling (Phase 3).
- Spheres far from the image center look egg-shaped. That is correct:
  the default camera is a very wide-angle pinhole. If you want a calmer
  lens, add a `focal_length` field to `Camera` and increase it.

**Try:**
- Change `light_dir` and watch shadows slide across the ground.
- Raise the green sphere; its shadow detaches.
- Set `Sphere::EPSILON` to `0.0`, look for shadow-acne speckles, then
  set it back and appreciate it.

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